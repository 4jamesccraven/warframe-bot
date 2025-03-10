{
  description = "Warframe discord bot";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = (with pkgs; [
            cargo
            rustc
            libgcc

            pkg-config
            openssl

            # For quick api testing
            python312
          ]) ++ (with pkgs.python312Packages; [
            requests
            rich
          ]);

          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };
      }
    );
}
