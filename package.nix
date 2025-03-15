{lib, pkgs}:

with pkgs;
rustPlatform.buildRustPackage {
  pname = "wf-bot";
  version = "0.1.0";

  nativeBuildInputs = with pkgs; [
    openssl.dev
    pkg-config
  ];

  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;
}

