{
  outputs = inputs@{ self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };

  in
  {
    devShells.${system}.default = pkgs.mkShell rec {
      buildInputs = with pkgs; [
        cargo
        rustc
        rustup

        # Required for cargo build
        gcc
        pkg-config
        alsa-lib

        # APIs  to display window
        xorg.libX11
        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi
        libxkbcommon

        vulkan-loader # Grapics API

      ];
      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath buildInputs;
    };
  };
}
