# SPDX-License-Identifier: MIT OR AGPL-3.0-or-later
# SPDX-FileCopyrightText: 2025 Jonathan D.A. Jewell
# flake.nix — proof-of-work Nix Flake (Fallback)
# Primary package manager: Guix (see guix.scm)
# Run: nix develop or nix build
{
  description = "Puzzle game with cryptographic solution verification using Z3";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rustToolchain
            cargo
            clippy
            rustfmt

            # Build dependencies
            pkg-config
            openssl
            z3

            # Bevy dependencies (Linux)
            udev
            alsa-lib
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            libxkbcommon
            wayland

            # Development tools
            just
            cargo-audit
            cargo-watch
          ];

          shellHook = ''
            echo "proof-of-work development shell (Nix fallback)"
            echo "Primary package manager: Guix (see guix.scm)"
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
              pkgs.vulkan-loader
              pkgs.wayland
              pkgs.libxkbcommon
            ]}"
          '';

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "proof-of-work";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl
            z3
            udev
            alsa-lib
            vulkan-loader
          ];

          meta = with pkgs.lib; {
            description = "Puzzle game with cryptographic solution verification";
            homepage = "https://github.com/hyperpolymath/proof-of-work";
            license = with licenses; [ mit agpl3Plus ];
            maintainers = [ ];
          };
        };
      }
    );
}
