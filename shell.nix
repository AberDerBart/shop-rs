let
  oxalica_overlay = import (builtins.fetchTarball
    "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ oxalica_overlay ]; };
  unstable = import <nixos-unstable> {};
  rust_channel = nixpkgs.rust-bin.stable."1.56.1".default;
in with nixpkgs;
pkgs.mkShell {
  buildInputs = [
  ];

  nativeBuildInputs = [
    (rust_channel.override {
      extensions = [ "rust-src" ];
    })

    # tools
    git
    clippy
    llvmPackages.clang

    unstable.cargo-audit
    unstable.cargo-outdated
  ];

  # Useful backtraces
  RUST_BACKTRACE = 1;

  # compilation of -sys packages requires manually setting LIBCLANG_PATH
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";

  # Workaround for https://github.com/mozilla/nixpkgs-mozilla/issues/240
  # with Rust 1.46.0
  LD_LIBRARY_PATH = ''${zlib.out}/lib''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}'';
}
