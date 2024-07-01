{ withSystem, ... }:
{ flake-parts-lib, lib, ... }:
{
  options.perSystem = flake-parts-lib.mkPerSystemOption (
    { pkgs, ... }:
    with lib.types;
    let
      inherit (lib) mkOption;
      /**
        # Type

        ```
        { language: "vim" | "lua";
          code: String;
          args: { ... };
        }
        ````
      */
      configDetail = submodule {
        options = {
          language = mkOption {
            type = enum [
              "vim"
              "lua"
            ];
            default = "lua";
          };
          code = mkOption {
            type = lines;
            default = "";
          };
          args = mkOption {
            type = attrsOf str;
            default = { };
          };
        };
      };
      # String | configDetail
      config = either lines configDetail;

      startupConfig = mkOption {
        type = config;
        description = "executed at startup";
        default = "";
      };
      preConfig = mkOption {
        type = config;
        description = "executed before plugin load";
        default = "";
      };
      postConfig = mkOption {
        type = config;
        description = "executed after plugin load";
        default = "";
      };
      extraPackages = mkOption {
        type = listOf package;
        description = "extra packages";
        default = [ ];
      };
      eagerConfig = submodule {
        options = {
          inherit extraPackages startupConfig;
          package = mkOption {
            type = nullOr package;
            description = "package alias";
            default = null;
          };
          packages = mkOption {
            type = listOf package;
            default = [ ];
          };
        };
      };
      lazyConfig = submodule {
        options = {
          inherit
            extraPackages
            startupConfig
            preConfig
            postConfig
            ;
          package = mkOption {
            type = nullOr package;
            description = "packages alias";
            default = null;
          };
          packages = mkOption {
            type = listOf package;
            default = [ ];
          };
          depends = mkOption {
            type = listOf (either package lazyConfig);
            default = [ ];
            description = "`lazyConfig` on which this depends";
          };
          hooks = mkOption {
            type = submodule {
              options = {
                modules = mkOption {
                  type = listOf str;
                  default = [ ];
                };
                events = mkOption {
                  type = listOf str;
                  default = [ ];
                  example = ''
                    events = [ "BufEnter" ];
                  '';
                };
                userEvents = mkOption {
                  type = listOf str;
                  default = [ ];
                };
                fileTypes = mkOption {
                  type = listOf str;
                  default = [ ];
                };
                commands = mkOption {
                  type = listOf str;
                  default = [ ];
                };
              };
            };
            default = { };
          };
          useDenops = mkOption {
            type = bool;
            default = false;
          };
        };
      };
      pluginOptions = {
        eager = mkOption {
          type = attrsOf eagerConfig;
          description = "eagerly loaded plugins";
          default = { };
        };
        lazy = mkOption {
          type = attrsOf lazyConfig;
          description = "lazily loaded plugins";
          default = { };
        };
      };

      neovimOptions = {
        inherit extraPackages;
        package = mkOption {
          type = types.package;
          description = "neovim package";
          default = pkgs.neovim-unwrapped;
        };
        extraConfig = mkOption {
          type = config;
          description = "extra configuration to add to top of init file";
          default = "";
        };
        logLevel = mkOption {
          type = enum [
            "error"
            "warn"
            "info"
            "debug"
          ];
          description = "log level";
          default = "warn";
        };
        after = mkOption {
          type = attrsOf (attrsOf config);
          description = "for preferences to overrule or add to the distributed defaults or system-wide settings.";
          example = ''
            after = {
              ftplugin = {
                nix = {
                  language = "lua";
                  code = "vim.bo.shiftwidth = 2";
                };
              };
            };
          '';
          default = { };
        };
      };
    in
    {
      options.bundler-nvim = mkOption {
        description = "bundler configuration";
        type = attrsOf (submodule {
          options = neovimOptions // pluginOptions;
        });
      };
    }
  );

  config.perSystem =
    {
      system,
      config,
      pkgs,
      ...
    }:
    let
      inherit (builtins)
        toJSON
        readFile
        replaceStrings
        listToAttrs
        ;
      inherit (lib)
        escapeShellArgs
        flatten
        isString
        makeBinPath
        mapAttrs'
        nameValuePair
        optionalString
        ;
      inherit (lib.lists) unique;
      inherit (lib.attrsets) attrValues;
      inherit (lib.strings) concatStringsSep;

      flatMap = f: xs: flatten (map f xs);
      concat = concatStringsSep "\n";

      # (Package | eagerConfig | lazyConfig) -> [Package]
      extractVimPluginsPackages =
        let
          f =
            src:
            if src ? pname then
              # src is Package
              [ src ]
            else
              # src is eagerConfig | lazyConfig
              let
                packages = if src.package != null then [ src.package ] else src.packages;
                depends = src.depends or [ ];
              in
              map f (packages ++ depends);
        in
        f;

      # (Package | eagerConfig | lazyConfig) -> [Package]
      extractExtraPackages =
        let
          f =
            src:
            if src ? pname then
              # src is Package
              [ ]
            else
              # src is eagerConfig | lazyConfig
              let
                packages = if src.package != null then [ src.package ] else src.packages;
                depends = src.depends or [ ];
              in
              (map f (packages ++ depends)) ++ src.extraPackages;
        in
        f;

      # String -> AttrSet -> Package
      mkNeovimPackage =
        name: cfg:
        let
          bundler = withSystem system ({ config, ... }: config.packages.bundler);
          # [{ plugin: Package }]
          eagerPluginPackages =
            let
              src = attrValues cfg.eager;
              packages = flatMap extractVimPluginsPackages src;
            in
            map (plugin: { inherit plugin; }) (unique packages);
          # [{ plugin: Package; optional = true; }]
          lazyPluginPackages =
            let
              src = attrValues cfg.lazy;
              packages = flatMap extractVimPluginsPackages src;
            in
            map (plugin: {
              inherit plugin;
              optional = true;
            }) (unique packages);
          # AttrSet
          packagePaths = listToAttrs (
            map (p: {
              name = p.plugin.pname;
              value = p.plugin;
            }) (eagerPluginPackages ++ lazyPluginPackages)
          );
          # [ Package ]
          extraPackages = unique (
            cfg.extraPackages
            ++ (flatMap extractExtraPackages (
              with cfg;
              flatMap attrValues [
                eager
                lazy
              ]
            ))
          );
          # String
          extraConfig =
            if isString cfg.extraConfig then
              # lua lines
              cfg.extraConfig
            else if cfg.extraConfig.language == "lua" then
              # configDetail (lua)
              let
                args =
                  if cfg.extraConfig.args != { } then
                    [ "local args = vim.json.decode([[${toJSON cfg.extraConfig.args}]])" ]
                  else
                    [ ];
              in
              concat (args ++ [ cfg.extraConfig.code ])
            else
              # configDetail (vim)
              let
                args =
                  if cfg.extraConfig.args != { } then
                    [ "let s:args = json_decode('${toJSON cfg.extraConfig.args}')" ]
                  else
                    [ ];
                file = pkgs.writeText "extra.vim" (concat (args ++ [ cfg.extraConfig.code ]));
              in
              "vim.cmd('source ${file}')";
          payload = pkgs.writeText "payload.json" (toJSON {
            vimConfig = {
              inherit (cfg) after;
            };
            pluginConfig = {
              inherit (cfg) eager lazy;
            };
            meta = {
              inherit extraPackages packagePaths;
            };
          });
          configRoot = pkgs.stdenv.mkDerivation {
            pname = "bundler-nvim-config";
            version = "transfer";
            phases = [ "installPhase" ];
            installPhase = ''
              mkdir $out
              ln -s ${payload} $out/payload.json
              ${bundler}/bin/bundler ${payload} $out
            '';
          };
          # TODO rename
          file = pkgs.writeText "file.lua" ''
            print("bundler v3.0.0")
            ${extraConfig}
            ${replaceStrings [ "REPLACED_BY_NIX" ] [
              "{root='${configRoot}',log_level=3}"
            ] (readFile ./runtime.lua)}
          '';
          entrypoint =
            let
              buildScript = pkgs.writeText "batch.vim" ''
                lua << EOF
                  local target_filename="${file}"
                  local out_filename="rc-content"
                  local chunk = assert(loadfile(target_filename))
                  local file = assert(io.open(out_filename, "w+b"))
                  file:write(string.dump(chunk))
                  file:close()
                EOF
              '';
            in
            pkgs.stdenv.mkDerivation {
              pname = "entrypoint";
              version = "bundled";
              src = ./.;
              outputs = [ "out" ];
              preferLocalBuild = true;
              buildInputs = [ cfg.package ];
              buildPhase = ''
                nvim --clean -es -S ${buildScript} -V
              '';
              installPhase = ''
                cp rc-content $out
              '';
            };

          neovimConfig = pkgs.neovimUtils.makeNeovimConfig {
            plugins = eagerPluginPackages ++ lazyPluginPackages;
            wrapRc = true;
          };
        in
        pkgs.wrapNeovimUnstable cfg.package (
          neovimConfig
          // {
            extraName = name;
            wrapperArgs =
              (escapeShellArgs neovimConfig.wrapperArgs)
              + (optionalString (extraPackages != [ ]) " --suffix PATH : \"${makeBinPath extraPackages}\"");
            luaRcContent = ''
              vim.g.bundler_root = "${configRoot}";
              -- origin ${file}
              dofile('${entrypoint}')
            '';
          }
        );

      # String -> String
      mkPackageName = name: if name == "default" then "bundler-nvim" else "bundler-nvim-${name}";
    in
    {
      packages = mapAttrs' (
        name: cfg: nameValuePair (mkPackageName name) (mkNeovimPackage name cfg)
      ) config.bundler-nvim;
      apps = mapAttrs' (
        name: cfg:
        nameValuePair (mkPackageName name) {
          type = "app";
          program = "${mkNeovimPackage name cfg}/bin/nvim";
        }
      ) config.bundler-nvim;
    };
}
