let
  pkgs = import <nixpkgs> {};

  mkShell = pkgs.mkShell.override {
    stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.stdenv;
  };
in
  mkShell {
    name = "rust-dev-env";

    buildInputs = with pkgs; [
      rustup
      pkg-config
      cargo-criterion
    ];
  }
