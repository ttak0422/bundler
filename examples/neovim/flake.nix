{
  description = "Neovim with bundler-nvim";

  inputs = {
    bundler.url = "github:ttak0422/bundler";
    nixpkgs.follows = "bundler/nixpkgs";
    flake-parts.follows = "bundler/flake-parts";
  };

  outputs = inputs@{ bundler, nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ bundler.flakeModules.neovim ];
      systems = nixpkgs.lib.systems.flakeExposed;

      perSystem = { pkgs, ... }: {

        # Please see the documentation for more information.
        bundler-nvim = {

          # The configuration defined in `default` can be accessed as `bundler-nvim`.
          default = {

            # Define viml code to be executed at startup.
            extraConfig = ''
              set laststatus=3
            '';

            # Define lua code to be executed at startup.
            extraLuaConfig = ''
              vim.g.mapleader = " "
              vim.loader.enable()
            '';

            # Define plugins to be loaded at startup.
            eagerPlugins = with pkgs.vimPlugins; [{
              plugin = kanagawa-nvim;

              # Define viml code to be executed at startup.
              #
              # following sample code is equivalent to
              #
              # ```nix
              # startupConfig = {
              #   language = "vim";
              #   code = ''
              #    colorscheme kanagawa
              #   '';
              # };
              #
              startupConfig = ''
                colorscheme kanagawa
              '';
            }];

            # define plugins to be lazy loaded.
            lazyPlugins = with pkgs.vimPlugins; [{
              plugin = telescope-nvim;

              # `startupConfig` is executed at startup, regardless of whether the plugin is loaded or not.
              startupConfig = ''
                nnoremap <leader>ff <cmd>Telescope find_files<cr>
                nnoremap <leader>fg <cmd>Telescope live_grep<cr>
                nnoremap <leader>fb <cmd>Telescope buffers<cr>
                nnoremap <leader>fh <cmd>Telescope help_tags<cr>
              '';

              # `preConfig` is executed before the plugin is loaded.
              preConfig = "";

              # `postConfig` is executed after the plugin is loaded.
              postConfig = {
                # Explicit language specification is required when use lua code.
                language = "lua";
                code = ''
                  require('telescope').setup()
                '';
              };

              # Load the plugin when the following commands are executed.
              onCommands = [ "Telescope" ];
            }];
          };

          # The configuration defined in `foo` can be accessed as `bundler-nvim-foo`.
          foo = {
            # ...
          };
        };
      };
    };
}
