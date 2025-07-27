{ lib, pkgs }:

with pkgs;
rustPlatform.buildRustPackage {
  pname = "wf-bot";
  version = "0.2.0";

  nativeBuildInputs = with pkgs; [
    openssl.dev
    pkg-config
  ];

  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  checkFlags = [
    # Test is impure, requires network usage
    "--skip=news_wrapper::news_wrapper_test::serialize_is_deserialize"
  ];

  meta = {
    license = lib.licenses.gpl3;
  };
}
