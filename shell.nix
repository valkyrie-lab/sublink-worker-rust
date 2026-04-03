let
  pkgs = import <nixpkgs> { };

  # mipsel-linux-musl cross toolchain from musl.cc
  mipselToolchain = pkgs.stdenv.mkDerivation {
    name = "mipsel-linux-musl-cross";
    src = pkgs.fetchurl {
      url = "https://musl.cc/mipsel-linux-musl-cross.tgz";
      sha256 = "gmJlM79+Z3wiXny+3x1bDWvGDD2q8oJJ5U8OuAXYmxM=";
    };
    unpackPhase = ''
      tar -xzf $src
      mv mipsel-linux-musl-cross $out
    '';
    installPhase = "true";
  };
in
pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
    pkgs.pkg-config
    mipselToolchain
  ];

  shellHook = ''
    rustup default nightly
    rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

    export CARGO_TARGET_MIPSEL_UNKNOWN_LINUX_MUSL_LINKER="${mipselToolchain}/bin/mipsel-linux-musl-gcc"
    export CC_mipsel_unknown_linux_musl="${mipselToolchain}/bin/mipsel-linux-musl-gcc"

    echo "Ready to build!"
    echo "Build command: cargo build --target mipsel-unknown-linux-musl --release -Zbuild-std=std,panic_abort"
  '';
}
