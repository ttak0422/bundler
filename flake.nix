{
  description = "Vim/Neovim package manager.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs = {
        flake-compat.follows = "flake-compat";
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
        nixpkgs-stable.follows = "nixpkgs";
      };
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, crane, pre-commit-hooks, ... }:
    flake-parts.lib.mkFlake { inherit inputs; }
    ({ flake-parts-lib, withSystem, ... }:
      let
        inherit (flake-parts-lib) importApply;
        flakeModules = {
          nvim = importApply ./nix/flake-module.nix { inherit withSystem; };
        };

      in {
        systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
        perSystem = { self', system, pkgs, lib, ... }:
          let
            inherit (import ./nix/bundler.nix { inherit system pkgs crane; })
              bundler toolchain;
            inherit (import ./nix/bundler-nvim.nix { inherit pkgs; })
              bundler-nvim;

          in {
            _module.args.pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = with inputs; [ fenix.overlays.default ];
            };
            packages = {
              bundler = bundler.package;
              bundler-nvim = bundler-nvim.package;
            };
            checks = {
              pre-commit-check = pre-commit-hooks.lib.${system}.run {
                src = ./.;
                hooks = {
                  deadnix.enable = true;
                  stylua.enable = true;
                  nixfmt.enable = true;
                  statix.enable = true;
                  # WIP
                  # rustfmt.enable = true;
                };
              };
              inherit (bundler) clippy nextest;
            };
            devShells.default = pkgs.mkShell {
              inherit (self'.checks.pre-commit-check) shellHook;
              packages = [ toolchain ]
                ++ (with pkgs; [ mdbook rust-analyzer-nightly ])
                ++ (with pkgs; lib.optional stdenv.isDarwin libiconv);
              inputsFrom = [ bundler ];
              RUST_BACKTRACE = "full";
            };
          };
        flake = { inherit flakeModules; };
      });
}
