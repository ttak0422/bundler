{
  description = "Vim with bundler-vim";

  inputs = {
    bundler.url = "github:ttak0422/bundler";
    nixpkgs.follows = "bundler/nixpkgs";
    flake-parts.follows = "bundler/flake-parts";
  };

  outputs = inputs@{ bundler, nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ bundler.flakeModules.vim ];
      systems = nixpkgs.lib.systems.flakeExposed;

      perSystem = { pkgs, ... }: {
        bundler-vim = {
          default = {
            eagerPlugins = with pkgs.vimPlugins; [{
              plugin = iceberg-vim;
              startupConfig = ''
                colorscheme iceberg
              '';
            }];
          };
        };
      };
    };
}
