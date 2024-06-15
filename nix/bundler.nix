{
  system,
  pkgs,
  crane,
}:
let
  inherit (pkgs.lib) optionals;
  inherit (pkgs.stdenv) isDarwin;
in
rec {
  toolchain = pkgs.fenix.stable.withComponents [
    "cargo"
    "clippy"
    "rustfmt"
    "rustc"
    "rust-src"
  ];

  craneLib = crane.lib.${system}.overrideToolchain toolchain;

  bundler = with craneLib; rec {
    commonArgs = {
      src = cleanCargoSource (path ./../bundler);
      buildInputs = optionals isDarwin (
        with pkgs;
        [
          libiconv
          darwin.apple_sdk.frameworks.Security
        ]
      );
      nativeBuildgInputs = [ ];
    };
    artifacts = buildDepsOnly (commonArgs // { pname = "bundler-deps"; });
    clippy = cargoClippy (commonArgs // { cargoArtifacts = artifacts; });
    nextest = cargoNextest (commonArgs // { cargoArtifacts = artifacts; });
    package = buildPackage (commonArgs // { cargoArtifacts = artifacts; });
  };
}
