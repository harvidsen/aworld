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
        gcc
      ];
      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };
  };
}
