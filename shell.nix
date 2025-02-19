let
  oxalica_overlay = import (
    builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"
  );
  pkgs = import <nixpkgs> { overlays = [ oxalica_overlay ]; };
in
pkgs.mkShell {
  packages = with pkgs; [
    openssl
    pkg-config
    binutils
    (rust-bin.stable.latest.default.override {
      extensions = [ "rust-src" ];
    })
    cargo-insta
    cargo-nextest
    cargo-expand
    rust-analyzer
    cargo-modules
    maturin
    uv
    python313
    workshop-runner
  ];
  shellHook = ''
    # Augment the dynamic linker path
    export "LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.stdenv.cc.cc.lib}/lib"
  '';
}
