{ withSystem, ... }:
{ config, lib, flake-parts-lib, ... }:
let inherit (flake-parts-lib) mkPerSystemOption;
in {
  options = {
    perSystem = mkPerSystemOption ({ pkgs, ... }:
      let
        inherit (lib) types mkEnableOption mkOption;
        neovim = {
          package = mkOption {
            type = types.package;
            description = "The neovim package";
            default = pkgs.neovim-unwrapped;
          };
          extraPackages = mkOption {
            type = with types; listOf package;
            description = "Extra packages to install";
            default = [ ];
          };
          extraConfig = mkOption {
            type = types.lines;
            description = "Extra configuration to add to top of init.vim";
            default = "";
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
              lang = mkOption {
                # WIP: support "fennel"
                type = types.enum [ "vim" "lua" "fennel" ];
                default = "lua";
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
          startPluginConfig = types.submodule {
            options = {
              plugin = mkOption { type = types.package; };
              startup = mkOption {
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
          optPluginConfig = types.submodule {
            options = {
              plugin = mkOption { type = types.package; };
              startup = mkOption {
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
              config = mkOption {
                type = with types; either lines pluginConfigDetail;
                description = "Configuration to add after plugin is loaded";
                default = "";
              };
              depends = mkOption {
                type = with types; listOf (either package optPluginConfig);
                description = "Plugins to load before this plugin";
                default = [ ];
              };
              dependBundles = mkOption {
                type = with types; listOf str;
                description = "Bundles to load before this plugin";
                default = [ ];
              };
              modules = mkOption {
                type = with types; listOf str;
                default = [ ];
              };
              events = mkOption {
                type = with types; listOf str;
                default = [ ];
              };
              filetypes = mkOption {
                type = with types; listOf str;
                default = [ ];
              };
              commands = mkOption {
                type = with types; listOf str;
                default = [ ];
              };
              lazy = mkEnableOption "lazy";
            };
          };
          bundleConfig = types.submodule {
            options = {
              name = mkOption {
                type = types.str;
                description = "Name of the bundle";
              };
              plugins = mkOption {
                type = with types; listOf (either package optPluginConfig);
                default = [ ];
              };
              startup = mkOption {
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
              config = mkOption {
                type = with types; either lines pluginConfigDetail;
                description = "Configuration to add after plugin is loaded";
                default = "";
              };
              depends = mkOption {
                type = with types; listOf (either package optPluginConfig);
                description = "Plugins to load before this plugin";
                default = [ ];
              };
              dependBundles = mkOption {
                type = with types; listOf str;
                description = "Bundles to load before this plugin";
                default = [ ];
              };
              modules = mkOption {
                type = with types; listOf str;
                default = [ ];
              };
              events = mkOption {
                type = with types; listOf str;
                default = [ ];
              };
              filetypes = mkOption {
                type = with types; listOf str;
                default = [ ];
              };
              commands = mkOption {
                type = with types; listOf str;
                default = [ ];
              };
              lazy = mkEnableOption "lazy";
            };
          };
        in {
          startPlugins = mkOption {
            type = with types; listOf (either package startPluginConfig);
            description = "Plugins to install and load on startup";
            default = [ ];
          };
          optPlugins = mkOption {
            type = with types; listOf (either package optPluginConfig);
            description = "Plugins to install and load on demand";
            default = [ ];
          };
          bundles = mkOption {
            type = types.listOf bundleConfig;
            description = "Bundles to install and load on demand";
            default = [ ];
          };
          lazyTime = mkOption {
            type = types.int;
            description =
              "Time in milliseconds to wait before loading lazy plugins";
            default = 100;
          };
        };
      in {
        options.bundler-nvim = mkOption {
          description = "bundler-nvim configuration";
          type = with types;
            attrsOf (submodule {
              options = {
                enable = mkEnableOption "bundler-nvim";
              } // neovim // bundlerPlugin;
            });
        };
      });
  };

  config = {
    perSystem = { system, config, lib, pkgs, ... }:
      let
        inherit (builtins) toJSON;
        inherit (lib) mapAttrs flatten;
        inherit (lib.lists) unique;
        inherit (pkgs) writeText;
        inherit (pkgs.stdenv) mkDerivation;

        bundler = withSystem system ({ config, ... }: config.packages.bundler);
        bundler-nvim =
          withSystem system ({ config, ... }: config.packages.bundler-nvim);

        # (package | startPluginConfig | optPluginConfig | bundleConfig) -> package[]
        extractVimPlugins = x:
          let
            arg = if x ? plugin then
              [ x.plugin ]
            else if x ? plugins then
              flatten (map extractVimPlugins x.plugins)
            else
              [ x ];
            depends = if x ? depends then
              flatten (map extractVimPlugins x.depends)
            else
              [ ];
          in arg ++ depends;

        # (package | startPluginConfig | optPluginConfig | bundleConfig) -> package[]
        extractExtraPackages = x:
          let
            arg = x.extraPackages or (if x ? plugins then
              map extractExtraPackages x.plugins
            else
              [ ]);
            depends = if x ? depends then
              flatten (map extractExtraPackages x.depends)
            else
              [ ];
          in arg ++ depends;

        mkNvimPackage = name: cfg:
          let
            startVimPluginPackages = [ bundler-nvim ]
              ++ unique (flatten (map extractVimPlugins cfg.startPlugins));

            optVimPluginPackages =
              let plugins = with cfg; startPlugins ++ optPlugins ++ bundles;
              in unique (flatten (map extractVimPlugins plugins));

            extraPackages =
              let plugins = with cfg; startPlugins ++ optPlugins ++ bundles;
              in unique (flatten (map extractExtraPackages plugins));

            payload = writeText "payload.json" (toJSON {
              inherit cfg;
              meta = {
                inherit extraPackages;
                idMap = map (p: {
                  package = p;
                  id = p.pname;
                }) (startVimPluginPackages ++ optVimPluginPackages);
              };
            });

            cfgFiles = mkDerivation {
              pname = "bundler-nvim-config";
              version = "0.1.0";
              phases = [ "installPhase" ];
              installPhase = ''
                mkdir $out
                ${bundler}/bin/bundler ${payload} $out
              '';
            };
          in pkgs.wrapNeovim cfg.package {
            inherit (cfg) withRuby withPython3 withNodeJs;
            configure = {
              customRC = ''
                lua << EOF
                -- ${name}
                require("bundler").new({
                  root = "${cfgFiles}",
                  lazy_time = ${toString cfg.lazyTime},
                })
                EOF
              '';
              packages.bundler-nvim = {
                start = startVimPluginPackages;
                opt = optVimPluginPackages;
              };
            };
          };
        mkApp = name: cfg: {
          type = "app";
          program = "${mkNvimPackage name cfg}/bin/nvim";
        };
      in {
        packages = mapAttrs mkNvimPackage config.bundler-nvim;
        apps = mapAttrs mkApp config.bundler-nvim;
      };
  };
}
