{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    fenix,
    pre-commit-hooks,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          fenix.overlays.default
        ];
      };
      naersk-lib = pkgs.callPackage naersk {};
    in {
      defaultPackage = naersk-lib.buildPackage {
        src = ./.;
        buildInputs = [];
        RUST_LOG = "trace";
      };
      checks = {
        pre-commit-check = pre-commit-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            nixpkgs-fmt.enable = true;
            cargo-check.enable = true;
            rustfmt.enable = true;
          };
        };
      };

      devShell = with pkgs;
        mkShell {
          buildInputs = [
            (pkgs.fenix.complete.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])
            cargo-watch
            pre-commit
          ];

          RUST_LOG = "debug";
          nativeBuildInputs = [pkgs.pkg-config];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
    });
}