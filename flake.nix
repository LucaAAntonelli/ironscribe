{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/unstable";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk/master"
    flake-compat = {
        url = "github:edolstra/flake-copat";
        flake = false;
      };
  };

  outputs = { self, nixpkgs, utils, naersk, ... }: {
      utils.lib.eachDefaultSystem (system: 
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk {};
        libPath = with pkgs; lib.makeLibraryPath [
          libGL
          libxkbcommon
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ]
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
    };
}
