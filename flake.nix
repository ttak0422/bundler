{
  description = "Vim/Neovim package manager.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    pre-commit-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs = {
        flake-compat.follows = "flake-compat";
        nixpkgs.follows = "nixpkgs";
      };
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    nix-filter.url = "github:numtide/nix-filter";
  };

  outputs =
    inputs@{
      flake-parts,
      crane,
      pre-commit-hooks,
      ...
    }:
    (
      flake-parts.lib.mkFlake { inherit inputs; } (
        { flake-parts-lib, withSystem, ... }:
        let
          inherit (flake-parts-lib) importApply;
          flakeModules = {
            neovim = importApply ./modules { inherit withSystem; };
          };
        in
        {
          systems = [
            "x86_64-linux"
            "aarch64-linux"
            "aarch64-darwin"
          ];
          perSystem =
            {
              self',
              system,
              pkgs,
              lib,
              ...
            }:
            let
              inherit (pkgs.lib) optionals;
              inherit (pkgs.stdenv) isDarwin;
              toolchain = pkgs.fenix.fromToolchainFile {
                file = ./bundler/rust-toolchain.toml;
                sha256 = "sha256-opUgs6ckUQCyDxcB9Wy51pqhd0MPGHUVbwRKKPGiwZU=";
              };
              craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
              bundler = with craneLib; rec {
                commonArgs = {
                  src = cleanCargoSource (path ./bundler);
                  buildInputs = optionals isDarwin (with pkgs; [ libiconv ]);
                  nativeBuildgInputs = [ ];
                };
                artifacts = buildDepsOnly (commonArgs // { pname = "bundler-deps"; });
                clippy = cargoClippy (commonArgs // { cargoArtifacts = artifacts; });
                nextest = cargoNextest (commonArgs // { cargoArtifacts = artifacts; });
                package = buildPackage (commonArgs // { cargoArtifacts = artifacts; });
              };
            in
            {
              _module.args.pkgs = import inputs.nixpkgs {
                inherit system;
                overlays = with inputs; [
                  fenix.overlays.default
                  nix-filter.overlays.default
                ];
              };
              packages = {
                bundler = bundler.package;
                mdbook =
                  let
                    inherit (pkgs) nix-filter;
                    src = nix-filter {
                      root = ./.;
                      include = [
                        ./docs
                        ./book.toml
                        (nix-filter.matchExt "md")
                      ];
                    };
                  in
                  pkgs.runCommand "mdbook" { } ''
                    ${pkgs.mdbook}/bin/mdbook build --dest-dir $out ${src}
                  '';
              };
              checks = {
                pre-commit-check = pre-commit-hooks.lib.${system}.run {
                  src = ./.;
                  settings.rust.cargoManifestPath = "./bundler/Cargo.toml";
                  hooks = {
                    deadnix.enable = true;
                    stylua.enable = true;
                    nixfmt-rfc-style.enable = true;
                    statix.enable = true;
                    rustfmt = {
                      enable = true;
                      packageOverrides = with pkgs.fenix.stable; {
                        inherit cargo rustfmt;
                      };
                    };
                  };
                };
                inherit (bundler) clippy nextest;
              };
              devShells.default = pkgs.mkShell {
                inherit (self'.checks.pre-commit-check) shellHook;
                packages = [
                  toolchain
                ]
                ++ (with pkgs; [
                  mdbook
                  rust-analyzer-nightly
                  nixfmt-rfc-style
                ])
                ++ (with pkgs; lib.optional stdenv.isDarwin libiconv);
                inputsFrom = [ bundler ];
                RUST_BACKTRACE = "full";
              };
            };
          flake = {
            inherit flakeModules;
          };
        }
      )
      // {
        templates = {
          neovim = {
            path = examples/neovim;
            description = "neovim with bundler";
            welcomeText = ''
              Run `nix run .#bundler-nvim`
            '';
          };
        };
      }
    );
}
