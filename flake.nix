{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk/master";
    flake-compat = {
        url = "github:edolstra/flake-compat";
        flake = false;
      };
    nixpkgs-mozilla = {
        url = "github:mozilla/nixpkgs-mozilla";
        flake = false;
      };
  };

  outputs = { self, nixpkgs, utils, naersk, nixpkgs-mozilla,  ... }: 
      utils.lib.eachDefaultSystem (system: 
      let
        pkgs = import nixpkgs { inherit system; 
          overlays = [
            (import nixpkgs-mozilla)
          ];
        };
        toolchain = (pkgs.rustChannelOf {
            rustToolchain = ./rust-toolchain.toml;
            sha256 = "X/4ZBHO3iW0fOenQ3foEvscgAPJYl2abspaBThDOukI=";
          }).rust;
        naersk-lib = pkgs.callPackage naersk {
            cargo = toolchain;
            rustc = toolchain;
          };
        libPath = with pkgs; lib.makeLibraryPath [
          libGL
          libxkbcommon
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ];
      in {
          defaultPackage = naersk-lib.buildPackage {
              src = ./.;
              doCheck = true;
              pname = "ironscribe";
              nativeBuildInputs = [ pkgs.makeWrapper];
              buildInputs = with pkgs; [
                xorg.libxcb
              ];
              postInstall = ''
                wrapProgram "$out/bin/ironscribe" --prefix LD_LIBRARY_PATH : "${libPath}"
              '';
            };
          
          defaultApp = utils.lib.mkApp {
              drv = self.defaultPackage."${system}";
            };

        });
}
