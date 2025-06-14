{
  description = "Rust project using fenix and rust-toolchain.toml";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { nixpkgs, flake-utils, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust-toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-X/4ZBHO3iW0fOenQ3foEvscgAPJYl2abspaBThDOukI=";
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rust-toolchain
            pkgs.cargo
            pkgs.pkg-config
            pkgs.openssl
            # pkgs.protoc-gen-tonic
            pkgs.protobuf
          ];
          RUST_BACKTRACE = "1";
        };
      });
}
