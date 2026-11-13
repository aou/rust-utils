# clanky: to build static, uncomment glibc.static, then
# RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-unknown-linux-gnu
{
  description = "nix-c2";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        # pkgs = nixpkgs.legacyPackages.${system};
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            fenix.overlays.default
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            (pkgs.fenix.stable.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])
            # pkgs.gcc
            # pkgs.openssl
            # pkgs.postgresql
            # pkgs.sqlite-interactive
            pkgs.rust-analyzer-nightly
          ];
          nativeBuildInputs = with pkgs; [
            # duckdb
            # postgresql
            # glibc
            # openssl
            # pkgconf
            # sqlite
          ];
        };
      }
    );
}
