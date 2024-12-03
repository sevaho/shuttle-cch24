# shell.nix
let
  pkgs = import <nixpkgs> {};
in pkgs.mkShell {
    packages = with pkgs; [
        rustc
        cargo

        # IDE
        clippy
        rust-analyzer
    ];

    LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [
        pkgs.stdenv.cc.cc
    ]}";

}
