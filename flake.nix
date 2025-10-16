{
  description = "Drift Bevy Dev Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        pkg-config
        wayland
        wayland-protocols
        xorg.libX11
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
        libxkbcommon
        vulkan-loader
        alsa-lib
        udev
      ];

      shellHook = ''
        echo "DevEnv is ready twin"
      '';
    };
  };
}
