{
  description = "Neovim with bundler-nvim";

  inputs = {
    bundler.url = "github:ttak0422/bundler";
    nixpkgs.follows = "bundler/nixpkgs";
    flake-parts.follows = "bundler/flake-parts";
  };

  outputs =
    inputs@{
      bundler,
      nixpkgs,
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ bundler.flakeModules.neovim ];
      systems = nixpkgs.lib.systems.flakeExposed;

      perSystem =
        { pkgs, ... }:
        {
          bundler = {
            default = {
              package = pkgs.neovim;

              logLevel = "info";

              eager = {
                inherit (pkgs) ddu;
                ddc0 = {
                  package = pkgs.ddc;
                };
                ddc = {
                  packages = [
                    pkgs.ddc-vim
                    {
                      # alias packages = [ pkgs.ddc-source ];
                      package = pkgs.ddc-source;
                      preConfig0 = ''
                        lua
                      '';
                      preConfig1 = {
                        language = "lua";
                        code = ''
                          -- test
                        '';
                      };
                      postConfig0 = ''
                        lua
                      '';
                      postConfig1 = {
                        language = "lua";
                        code = ''
                          -- test
                        '';
                      };
                      extraPackages = with pkgs; [ cowsay ];
                    }
                  ];
                  preConfig0 = ''
                    lua
                  '';
                  postConfig1 = {
                    language = "vim";
                    code = ''
                      vim
                    '';
                  };
                };
              };
              lazy = { };
              after = {
                java = {
                  language = "lua";
                  code = ''
                    " test
                  '';
                };
              };
            };
          };
        };
    };
}
