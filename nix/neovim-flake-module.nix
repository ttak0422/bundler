{ withSystem, ... }:
{ config, lib, flake-parts-lib, ... }:
let inherit (flake-parts-lib) mkPerSystemOption;
in {
  options = {
    perSystem = mkPerSystemOption ({ pkgs, ... }:
      let
        inherit (lib) types mkEnableOption mkOption;
        neovim = {
          target = mkOption {
            type = types.enum [ "neovim" ];
            default = "neovim";
            description = "internal usage";
            visible = false;
          };
          package = mkOption {
            type = types.package;
            description = "Neovim package";
            default = pkgs.neovim-unwrapped;
          };
          extraPackages = mkOption {
            type = with types; listOf package;
            description = "Extra packages to install";
            default = [ ];
          };
          extraConfig = mkOption {
            type = types.lines;
            description =
              "Extra configuration (viml) to add to top of init.vim";
            default = "";
          };
          extraLuaConfig = mkOption {
            type = types.lines;
            description = "Extra configuration (lua) to add to top of init.vim";
            default = "";
          };
          after = {
            ftplugin = mkOption {
              type = with types; attrsOf lines;
              description = "after/ftplugin configuration (viml)";
              example = ''
                {
                  lua = "setlocal expandtab";
                  nix = builtins.readFile ./path/to/after/ftplugin/nix.vim;
                  # ...
                }
              '';
              default = { };
            };
          };
          withNodeJs = mkEnableOption "withNodeJs" // {
            description = "Alias for neovim.withNodeJs";
          };
          withPython3 = mkEnableOption "withPython3" // {
            description = "Alias for neovim.withPython3";
          };
          withRuby = mkEnableOption "withRuby" // {
            description = "Alias for neovim.withRuby";
          };
        };
        bundlerPlugin = let
          pluginConfigDetail = types.submodule {
            options = {
              language = mkOption {
                type = types.enum [ "vim" "lua" ];
                default = "vim";
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
                description = "Configuration to add before plugin is loaded";
                default = "";
              };
              extraPackages = mkOption {
                type = with types; listOf package;
                description = "Extra packages to install";
                default = [ ];
              };
            };
          };
          lazyPluginConfig = types.submodule {
            options = {
              plugin = mkOption { type = types.package; };
              startupConfig = mkOption {
                type = with types; either lines pluginConfigDetail;
                description = "Configuration to add before plugin is loaded";
                default = "";
              };
              extraPackages = mkOption {
                type = with types; listOf package;
                description = "Extra packages to install";
                default = [ ];
              };
              preConfig = mkOption {
                type = with types; either lines pluginConfigDetail;
                description = "Configuration to add before plugin is loaded";
                default = "";
              };
              postConfig = mkOption {
                type = with types; either lines pluginConfigDetail;
                description = "Configuration to add after plugin is loaded";
                default = "";
              };
              dependPlugins = mkOption {
                type = with types; listOf (either package lazyPluginConfig);
                description = "Plugins to load before this plugin";
                default = [ ];
              };
              dependGroups = mkOption {
                type = with types; listOf str;
                description = "Groups to load before this plugin";
                default = [ ];
              };
              onModules = mkOption {
                type = with types; listOf str;
                default = [ ];
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
              useDenops = mkEnableOption "useDenops";
            };
          };
          lazyGroupConfig = types.submodule {
            options = {
              name = mkOption {
                type = types.str;
                description = "Name of the group";
              };
              plugins = mkOption {
                type = with types; listOf (either package lazyPluginConfig);
                default = [ ];
              };
              startupConfig = mkOption {
                type = with types; either lines pluginConfigDetail;
                description = "Configuration to add before plugin is loaded";
                default = "";
              };
              extraPackages = mkOption {
                type = with types; listOf package;
                description = "Extra packages to install";
                default = [ ];
              };
              preConfig = mkOption {
                type = with types; either lines pluginConfigDetail;
                description = "Configuration to add before plugin is loaded";
                default = "";
              };
              postConfig = mkOption {
                type = with types; either lines pluginConfigDetail;
                description = "Configuration to add after plugin is loaded";
                default = "";
              };
              dependPlugins = mkOption {
                type = with types; listOf (either package lazyPluginConfig);
                description = "Plugins to load before this plugin";
                default = [ ];
              };
              dependGroups = mkOption {
                type = with types; listOf str;
                description = "Groups to load before this plugin";
                default = [ ];
              };
              onModules = mkOption {
                type = with types; listOf str;
                default = [ ];
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
            description = "Plugins to install and load on startup";
            default = [ ];
          };
          lazyPlugins = mkOption {
            type = with types; listOf (either package lazyPluginConfig);
            description = "Plugins to install and load on demand";
            default = [ ];
          };
          lazyGroups = mkOption {
            type = types.listOf lazyGroupConfig;
            description = "Plugin groups to install and load on demand";
            default = [ ];
          };
          timer = mkOption {
            type = types.int;
            description =
              "Time in milliseconds to wait before loading lazy plugins";
            default = 100;
          };
          logLevel = mkOption {
            type = types.enum [ "debug" "info" "warn" "error" ];
            description = "log level of bundler-nvim";
            default = "warn";
          };
        };
      in {
        options.bundler-nvim = mkOption {
          description = "bundler-nvim configuration";
          type = with types;
            attrsOf (submodule {
              options = {
                packageNamePrefix = mkOption {
                  type = types.str;
                  default = "bundler-nvim";
                };
                appNamePrefix = mkOption {
                  type = types.str;
                  default = "bundler-nvim";
                };
              } // neovim // bundlerPlugin;
            });
        };
      });
  };

  config = {
    perSystem = { system, config, lib, pkgs, ... }:
      let
        inherit (builtins) toJSON;
        inherit (lib)
          mapAttrs' nameValuePair flatten optionalString makeBinPath
          escapeShellArgs;
        inherit (lib.lists) unique;
        inherit (pkgs) writeText;
        inherit (pkgs.stdenv) mkDerivation;

        bundler = withSystem system ({ config, ... }: config.packages.bundler);
        bundler-nvim =
          withSystem system ({ config, ... }: config.packages.bundler-nvim);

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

        # (package | eagerPluginConfig | lazyPluginConfig | lazyGroupConfig) -> package[]
        extractExtraPackages = x:
          let
            arg = (x.extraPackages or [ ]) ++ (if x ? plugins then
              map extractExtraPackages x.plugins
            else
              [ ]);
            depends =
              flatten (map extractExtraPackages (x.dependPlugins or [ ]));
          in arg ++ depends;

        mkNvimPackage = name: cfg:
          let
            eagerVimPluginPackages = [ bundler-nvim ]
              ++ unique (flatten (map extractVimPlugins cfg.eagerPlugins));
            lazyVimPluginPackages =
              let plugins = with cfg; lazyPlugins ++ lazyGroups;
              in unique (flatten (map extractVimPlugins plugins));
            normalizedStartVimPluginPackages =
              map (p: { plugin = p; }) eagerVimPluginPackages;
            normalizedOptVimPluginPackages = map (p: {
              plugin = p;
              optional = true;
            }) lazyVimPluginPackages;

            extraPackages =
              let plugins = with cfg; eagerPlugins ++ lazyPlugins ++ lazyGroups;
              in unique (cfg.extraPackages
                ++ (flatten (map extractExtraPackages plugins)));

            payload = writeText "payload.json" (toJSON {
              config = cfg;
              meta = {
                inherit extraPackages;
                inherit (cfg) target;
                # hack to escape GC.
                bundlerBin = bundler;
                idMap = map (p: {
                  package = p;
                  pluginId = p.pname;
                }) (eagerVimPluginPackages ++ lazyVimPluginPackages);
              };
            });

            cfgFiles = mkDerivation {
              pname = "bundler-nvim-config";
              version = "2.2.1";
              phases = [ "installPhase" ];
              installPhase = ''
                mkdir $out
                ${bundler}/bin/bundler ${payload} $out
              '';
            };

            extraPackagesArgs = optionalString (extraPackages != [ ])
              ''--suffix PATH : "${makeBinPath extraPackages}"'';

            neovimConfig = pkgs.neovimUtils.makeNeovimConfig {
              inherit (cfg) withRuby withPython3 withNodeJs;
              customRC = ''
                " ${name}
                ${cfg.extraConfig}
                lua << EOF
                ${cfg.extraLuaConfig}
                vim.opt.runtimepath:append("${cfgFiles}/after");
                require("bundler").new({
                  root = "${cfgFiles}",
                  timer = ${toString cfg.timer},
                  log_level = "${cfg.logLevel}",
                }):setup_loader()
                EOF
              '';
              wrapRc = true;
              plugins = normalizedStartVimPluginPackages
                ++ normalizedOptVimPluginPackages;
            };
          in pkgs.wrapNeovimUnstable cfg.package (neovimConfig // {
            wrapperArgs = (escapeShellArgs neovimConfig.wrapperArgs) + " "
              + extraPackagesArgs;
          });
      in {
        packages = mapAttrs' (name: cfg:
          let
            fullName = if name == "default" then
              "${cfg.packageNamePrefix}"
            else
              "${cfg.packageNamePrefix}-${name}";
          in nameValuePair fullName (mkNvimPackage name cfg))
          config.bundler-nvim;

        apps = mapAttrs' (name: cfg:
          let
            fullName = if name == "default" then
              "${cfg.appNamePrefix}"
            else
              "${cfg.appNamePrefix}-${name}";
          in nameValuePair fullName {
            type = "app";
            program = "${mkNvimPackage name cfg}/bin/nvim";
          }) config.bundler-nvim;
      };
  };
}
