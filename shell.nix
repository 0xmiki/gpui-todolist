{ pkgs ? import <nixpkgs> {} }:

let
  # Import the official rust overlay so we can pick a version/channel
  rustOverlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgsWithRust = import <nixpkgs> { overlays = [ rustOverlay ]; };
in

pkgsWithRust.mkShell {
  name = "rust-gpui-todolist-env";

  buildInputs = with pkgsWithRust; [
    # Specify nightly toolchain (exact date optional)
    (rust-bin.nightly."2025-11-06".default)

    pkg-config
    openssl
    # GPU / Vulkan / Wayland dependencies
    mesa
    vulkan-loader
    vulkan-tools
    vulkan-validation-layers
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    libxkbcommon
    wayland
  ];

  shellHook = ''

    export LD_LIBRARY_PATH=${pkgsWithRust.lib.makeLibraryPath [
      pkgsWithRust.wayland
      pkgsWithRust.libxkbcommon
      pkgsWithRust.xorg.libX11
      pkgsWithRust.xorg.libXcursor
      pkgsWithRust.xorg.libXrandr
      pkgsWithRust.xorg.libXi
      pkgsWithRust.mesa
      pkgsWithRust.vulkan-loader
      pkgsWithRust.vulkan-validation-layers
    ]}:$LD_LIBRARY_PATH

    export RUST_BACKTRACE=1

    echo "🦀 Ready to develop Rust GPUI Calculator with nightly 2025-11-06!"
  '';
}
