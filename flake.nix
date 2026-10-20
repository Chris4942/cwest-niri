{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              openssl
              pkg-config
              rust-bin.beta.latest.default
            ];
          };
        packages.cwest-niri = (pkgs.makeRustPlatform { inherit (pkgs) rustc cargo; }).buildRustPackage {
          pname = "cwest-niri";
          version = "0.1.0";
          src = ./.;
          cargoHash = "sha256-G52m3MFDyq6l+eOIjT/O4jaR+hY52azXVlpNhaRK7Yc=";
        };
      }
    );
}
