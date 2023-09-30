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
      inputs.flake-compat.follows = "flake-compat";
      inputs.flake-utils.follows = "flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.nixpkgs-stable.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.flake-utils.follows = "flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-compat.follows = "flake-compat";
      inputs.rust-overlay.follows = "";
    };
  };

  outputs = inputs@{ flake-parts, crane, ... }:
    flake-parts.lib.mkFlake { inherit inputs; }
    ({ self, flake-parts-lib, withSystem, ... }:
      let
        inherit (flake-parts-lib) importApply;
        flakeModules = {
          nvim = importApply ./nix/flake-module.nix { inherit withSystem; };
        };

      in {
        systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
        perSystem = { self', system, pkgs, config, flake-parts-lib, ... }:
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
          };
        flake = { inherit flakeModules; };
      });
}
