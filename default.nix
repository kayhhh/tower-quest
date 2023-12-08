{ lib, pkgs, system, build_inputs, native_build_inputs, makeRustPlatform }:
let
  rustBin = pkgs.rust-bin.stable.latest.default.override {
    targets = [ "wasm32-unknown-unknown" ];
  };

  rustPlatform = makeRustPlatform {
    cargo = rustBin;
    rustc = rustBin;
  };

  common = {
    version = "0.0.0";
    src = ./.;
    cargoLock.lockFile = ./Cargo.lock;
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

    buildInputs = build_inputs;
    nativeBuildInputs = native_build_inputs;

    LD_LIBRARY_PATH = lib.makeLibraryPath build_inputs;
  };
in {
  bin = rustPlatform.buildRustPackage (common // {
    pname = "tower-quest";
    buildPhase = ''
      cargo build --release
    '';
    installPhase = ''
      mkdir -p $out/release
      cp -r target/release $out
      cp -r assets $out/release/assets
    '';
  });
  wasm = rustPlatform.buildRustPackage (common // {
    pname = "tower-quest";
    buildPhase = ''
      cargo build --target wasm32-unknown-unknown --profile wasm-release
      wasm-bindgen --no-typescript --out-dir target/web --out-name tower_quest target/wasm32-unknown-unknown/wasm-release/tower_quest.wasm
    '';
    installPhase = ''
      mkdir -p $out/web
      cp -r target/web $out
      cp -r assets $out/web/assets
      cp index.html $out/web
    '';
  });
}
