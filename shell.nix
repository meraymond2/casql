with (import <nixpkgs> {});

mkShell rec {
  name = "casql";

  buildInputs = [
    cargo
    openssl
    pkgconfig
  ];


}
