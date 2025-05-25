{
  description = "Rust dev environment for IronScribe";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, flake-utils, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = fenix.packages.${system}.complete; # or .stable.toolchain
      in {
        devShells.default = pkgs.mkShell {
          packages = [
            toolchain
            pkgs.pkg-config
            pkgs.openssl
          ];
          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
