{ withSystem, ... }:
{ config, lib, flake-parts-lib, ... }:
let inherit (flake-parts-lib) mkPerSystemOption;
in {
  options = {
    perSystem = mkPerSystemOption ({ pkgs, ... }:
      let
        inherit (lib) types mkEnableOption mkOption;
        vim = {
          target = mkOption {
            type = types.enum [ "vim" ];
            default = "vim";
            description = "internal usage";
            visible = false;
          };
          package = mkOption {
            type = types.package;
            description = "vim package configurable";
            default = pkgs.vim-full;
            visible = false;
          };
          extraConfig = mkOption {
            type = types.lines;
            description = "configure at startup";
            default = "";
          };
          after = {
            ftplugin = mkOption {
              type = with types; attrsOf lines;
              description = "not yet support";
              default = { };
            };
          };
        };
        bundlerPlugin = let
          pluginConfigDetail = types.submodule {
            options = {
              language = mkOption {
                type = types.enum [ "vim" ];
                default = "vim";
                visible = false;
              };
              code = mkOption {
                type = types.lines;
                default = "";
              };
              args = mkOption {
                type = types.attrs;
                default = { };
              };
            };
          };
          eagerPluginConfig = types.submodule {
            options = {
              plugin = mkOption { type = types.package; };
              startupConfig = mkOption {
                type = with types; either lines pluginConfigDetail;
                description = "not yet support";
                default = "";
              };
              extraPackages = mkOption {
                type = with types; listOf package;
                description = "not yet support";
                default = [ ];
                visible = false;
              };
            };
          };
          lazyPluginConfig = types.submodule {
            options = {
              plugin = mkOption { type = types.package; };
              startupConfig = mkOption {
                type = with types; either lines pluginConfigDetail;
                description = "not yet support";
                default = "";
              };
              extraPackages = mkOption {
                type = with types; listOf package;
                description = "not yet support";
                default = [ ];
              };
              preConfig = mkOption {
                type = with types; either lines pluginConfigDetail;
                description = "not yet support";
                default = "";
              };
              postConfig = mkOption {
                type = with types; either lines pluginConfigDetail;
                description = "not yet support";
                default = "";
              };
              dependPlugins = mkOption {
                type = with types; listOf (either package lazyPluginConfig);
                description = "not yet support";
                default = [ ];
              };
              dependGroups = mkOption {
                type = with types; listOf str;
                description = "not yet support";
                default = [ ];
              };
              onModules = mkOption {
                type = with types; listOf str;
                description = "not yet support";
                default = [ ];
                visible = false;
              };
              onEvents = mkOption {
                type = with types; listOf str;
                description = "not yet support";
                default = [ ];
              };
              onFiletypes = mkOption {
                type = with types; listOf str;
                description = "not yet support";
                default = [ ];
              };
              onCommands = mkOption {
                type = with types; listOf str;
                description = "not yet support";
                default = [ ];
              };
              useTimer = mkOption {
                type = types.bool;
                description = "not yet support";
                default = false;
              };
              useDenops = mkOption {
                type = types.bool;
                description = "not yet support";
                default = false;
              };
            };
          };

          lazyGroupConfig = types.submodule {
            options = {
              name = mkOption { type = types.str; };
              plugins = mkOption {
                type = with types; listOf (either package lazyPluginConfig);
                default = [ ];
              };
              startupConfig = mkOption {
                type = with types; either lines pluginConfigDetail;
                default = "";
              };
              extraPackages = mkOption {
                type = with types; listOf package;
                default = [ ];
              };
              preConfig = mkOption {
                type = with types; either lines pluginConfigDetail;
                default = "";
              };
              postConfig = mkOption {
                type = with types; either lines pluginConfigDetail;
                default = "";
              };
              dependPlugins = mkOption {
                type = with types; listOf (either package lazyPluginConfig);
                default = [ ];
              };
              dependGroups = mkOption {
                type = with types; listOf str;
                default = [ ];
              };
              onModules = mkOption {
                type = with types; listOf str;
                default = [ ];
                visible = false;
              };
              onEvents = mkOption {
                type = with types; listOf str;
                default = [ ];
              };
              onFiletypes = mkOption {
                type = with types; listOf str;
                default = [ ];
              };
              onCommands = mkOption {
                type = with types; listOf str;
                default = [ ];
              };
              useTimer = mkEnableOption "useTimer";
            };
          };
        in {
          eagerPlugins = mkOption {
            type = with types; listOf (either package eagerPluginConfig);
            default = [ ];
          };
          lazyPlugins = mkOption {
            type = with types; listOf (either package lazyPluginConfig);
            default = [ ];
          };
          lazyGroups = mkOption {
            type = types.listOf lazyGroupConfig;
            default = [ ];
          };
        };
      in {
        options.bundler-vim = mkOption {
          description = "bundler-vim configuration";
          type = with types;
            attrsOf (submodule {
              options = {
                packageNamePrefix = mkOption {
                  type = types.str;
                  default = "bundler-vim";
                };
                appNamePrefix = mkOption {
                  type = types.str;
                  default = "bundler-vim";
                };
              } // vim // bundlerPlugin;
            });
        };
      });
  };

  config = {
    perSystem = { system, config, lib, pkgs, ... }:
      let
        inherit (builtins) toJSON;
        inherit (lib) mapAttrs' nameValuePair flatten;
        inherit (lib.lists) unique;
        inherit (pkgs) writeText;
        inherit (pkgs.stdenv) mkDerivation;

        bundler = withSystem system ({ config, ... }: config.packages.bundler);
        bundler-vim =
          withSystem system ({ config, ... }: config.packages.bundler-vim);

        # (package | eagerPluginConfig | lazyPluginConfig | lazyGroupConfig) -> package[]
        extractVimPlugins = x:
          let
            arg = if x ? plugin then
              [ x.plugin ]
            else if x ? plugins then
              flatten (map extractVimPlugins x.plugins)
            else
              [ x ];
            depends = flatten (map extractVimPlugins (x.dependPlugins or [ ]));
          in arg ++ depends;

        mkVimPackage = name: cfg:
          let
            eagerVimPluginPackages = [ bundler-vim ]
              ++ unique (flatten (map extractVimPlugins cfg.eagerPlugins));
            lazyVimPluginPackages =
              let plugins = with cfg; lazyPlugins ++ lazyGroups;
              in unique (flatten (map extractVimPlugins plugins));

            # TODO: support extra packages.
            extraPackages = [ ];

            payload = writeText "payload.json" (toJSON {
              config = cfg;
              meta = {
                inherit extraPackages;
                inherit (cfg) target;
                # hack to escape GC.
                bundlerBin = bundler;
                idMap = map (p: {
                  package = p;
                  pluginId = p.name;
                }) (eagerVimPluginPackages ++ lazyVimPluginPackages);
              };
            });

            cfgFiles = mkDerivation {
              pname = "bundler-vim-config";
              version = "2.2.0";
              phases = [ "installPhase" ];
              installPhase = ''
                mkdir $out
                ${bundler}/bin/bundler ${payload} $out
              '';
            };

          in cfg.package.customize {
            vimrcConfig.customRC = ''
              " WIP
              " payload: ${payload}
              " cfgFiles: ${cfgFiles}

              ${cfg.extraConfig}
            '';
            vimrcConfig.packages.bundlerVim = {
              start = eagerVimPluginPackages;
              opt = lazyVimPluginPackages;
            };
          };
      in {
        packages = mapAttrs' (name: cfg:
          let
            fullName = if name == "default" then
              "${cfg.packageNamePrefix}"
            else
              "${cfg.packageNamePrefix}-${name}";
          in nameValuePair fullName (mkVimPackage name cfg)) config.bundler-vim;

        apps = mapAttrs' (name: cfg:
          let
            fullName = if name == "default" then
              "${cfg.appNamePrefix}"
            else
              "${cfg.appNamePrefix}-${name}";
          in nameValuePair fullName {
            type = "app";
            program = "${mkVimPackage name cfg}/bin/vim";
          }) config.bundler-vim;
      };
  };
}
