{ lib, pkgs }:

with pkgs;
let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = with pkgs; [
    openssl.dev
    pkg-config
  ];
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

  meta = {
    license = lib.licenses.gpl3;
  };
}
