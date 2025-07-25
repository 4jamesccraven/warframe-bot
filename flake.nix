{
  description = "Warframe discord bot";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
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
          buildInputs = with pkgs; [
            cargo
            clippy
            rustc
            rustfmt
            libgcc

            pkg-config
            openssl
          ];

          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };

        packages.default = pkgs.callPackage ./package.nix { };
      }
    );
}
