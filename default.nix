{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs.buildPackages; [
       gcc
       alsaLib
       pkg-config
       udev
       vulkan-loader
       m4
       libxkbcommon
       xorg.libxcb
       xorg.libX11
       xorg.libXrandr
       xorg.libXcursor
       xorg.libXi ];
}