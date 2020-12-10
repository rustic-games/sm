{ pkgs ? import <nixpkgs> {}
, lib ? pkgs.lib
, rustPlatform ? pkgs.rustPlatform
, openssl ? pkgs.openssl
, pkgconfig ? pkgs.pkgconfig
}:
rustPlatform.buildRustPackage rec {
  pname = "sm";
  version = "1.0.0";

  nativeBuildInputs = [ pkgconfig ];
  buildInputs = [];

  src = lib.cleanSource ./.;
  cargoSha256 = "14wpjibb68s9vpr7gf64fn4y35yjk6hr64zrvbbqf8d3jc5xvssd";

  meta = with lib; {
    homepage = "https://github.com/rustic-games/sm";
    description = "Rust State Machine Library";
    license = licenses.mit;
  };
}
