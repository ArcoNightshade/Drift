{
  devShells.default = pkgs.mkShell {
    buildInputs = with pkgs; [
      pkg-config
      wayland
      wayland-protocols
      xorg.libX11
      xorg.libXcursor
      xorg.libXi
      xorg.libXrandr
      libxkbcommon
      vulkan-loader
    ];
  };
}
