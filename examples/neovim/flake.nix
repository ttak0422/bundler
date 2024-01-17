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
        bundler-nvim = {
          default = {
            extraLuaConfig = ''
              vim.loader.enable()
            '';
            eagerPlugins = with pkgs.vimPlugins; [{
              plugin = kanagawa-nvim;
              startupConfig = ''
                colorscheme kanagawa
              '';
            }];
            lazyPlugins = with pkgs.vimPlugins; [{
              plugin = telescope-nvim;
              startupConfig = {
                language = "lua";
                code = ''
                  local builtin = require('telescope.builtin')
                  vim.keymap.set('n', '<leader>ff', builtin.find_files, {})
                  vim.keymap.set('n', '<leader>fg', builtin.live_grep, {})
                  vim.keymap.set('n', '<leader>fb', builtin.buffers, {})
                  vim.keymap.set('n', '<leader>fh', builtin.help_tags, {})
                '';
              };
              postConfig = {
                language = "lua";
                code = ''
                  require('telescope').setup()
                '';
              };
            }];
          };
        };
      };
    };
}
