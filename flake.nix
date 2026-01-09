# SPDX-License-Identifier: AGPL-3.0-or-later
# SPDX-FileCopyrightText: 2025 hyperpolymath
{
  description = "Proof-of-Work puzzle game library with Bevy";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };

        # Bevy runtime dependencies
        bevyDeps = with pkgs; [
          # Audio
          alsa-lib
          # Graphics (X11)
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          # Graphics (Wayland)
          wayland
          libxkbcommon
          # Vulkan
          vulkan-loader
          vulkan-headers
          # Input
          udev
        ];

        # Build-time dependencies
        buildDeps = with pkgs; [
          pkg-config
          z3
          clang
          mold
        ];

        # Development tools
        devTools = with pkgs; [
          just
          cargo-edit
          cargo-audit
          cargo-outdated
          cargo-tarpaulin
          cargo-deny
          cargo-watch
          jq
        ];

      in {
        # Development shell
        devShells.default = pkgs.mkShell {
          buildInputs = [ rustToolchain ] ++ buildDeps ++ bevyDeps ++ devTools;

          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath bevyDeps}:$LD_LIBRARY_PATH"
            export RUST_LOG=info
            echo "Proof-of-Work development environment"
            echo "Run: cargo build --features headless"
          '';

          # Faster linking
          RUSTFLAGS = "-C link-arg=-fuse-ld=mold";
        };

        # Headless package (for servers/CI)
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "proof-of-work";
          version = "0.1.0";

          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ z3 alsa-lib ];

          buildFeatures = [ "headless" ];

          meta = with pkgs.lib; {
            description = "Proof-of-Work puzzle game library";
            homepage = "https://github.com/hyperpolymath/proof-of-work";
            license = licenses.agpl3Plus;
            maintainers = [ ];
          };
        };

        # Full package with graphics
        packages.full = pkgs.rustPlatform.buildRustPackage {
          pname = "proof-of-work";
          version = "0.1.0";

          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = [ pkgs.z3 ] ++ bevyDeps;

          buildFeatures = [ "z3-verify" ];

          # Runtime library path for graphics
          postFixup = ''
            patchelf --set-rpath "${pkgs.lib.makeLibraryPath bevyDeps}:$(patchelf --print-rpath $out/bin/proof-of-work)" $out/bin/proof-of-work
          '';

          meta = with pkgs.lib; {
            description = "Proof-of-Work puzzle game with full graphics";
            homepage = "https://github.com/hyperpolymath/proof-of-work";
            license = licenses.agpl3Plus;
            maintainers = [ ];
          };
        };

        # CI check package
        packages.ci = pkgs.stdenv.mkDerivation {
          name = "proof-of-work-ci";
          src = ./.;

          nativeBuildInputs = [ rustToolchain pkgs.pkg-config ];
          buildInputs = with pkgs; [ z3 alsa-lib ];

          buildPhase = ''
            export HOME=$TMPDIR
            cargo fmt --check
            cargo clippy --features headless -- -D warnings
            cargo test --features headless
            cargo audit
          '';

          installPhase = ''
            mkdir -p $out
            echo "CI checks passed" > $out/result
          '';
        };
      }
    );
}
