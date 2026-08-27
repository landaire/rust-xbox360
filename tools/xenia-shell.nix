# Dev shell providing Xenia's Linux build dependencies via nix, so premake's
# pkg-config lookups resolve and the emulator builds for headless verification.
{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = [ pkgs.pkg-config pkgs.gnumake pkgs.python3 pkgs.clang ];
  buildInputs = [
    pkgs.gtk3
    pkgs.lz4
    pkgs.xorg.libX11
    pkgs.xorg.libxcb
    pkgs.vulkan-headers
    pkgs.vulkan-loader
    pkgs.SDL2
    pkgs.libunwind
    pkgs.libgcc.lib
  ];
}
