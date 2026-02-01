;; SPDX-License-Identifier: PMPL-1.0-or-later
;; PLAYBOOK.scm - Operational runbook for proof-of-work

(define playbook
  `((version . "1.0.0")
    (procedures
      ((build
         ((dev . "cargo build")
          (release . "cargo build --release")
          (with-z3 . "cargo build --features z3-verify")
          (full . "cargo build --features full")
          (headless . "cargo build --features headless")))
       (test
         ((unit . "cargo test")
          (integration . "cargo test --test '*'")
          (with-z3 . "cargo test --features z3-verify")
          (fuzz . "cargo +nightly fuzz run fuzz_verification")))
       (run
         ((dev . "cargo run")
          (release . "cargo run --release")
          (with-steam . "cargo run --features steam")
          (with-network . "cargo run --features network")))
       (lint
         ((format . "cargo fmt")
          (check-format . "cargo fmt --check")
          (clippy . "cargo clippy")
          (clippy-strict . "cargo clippy -- -D warnings")))
       (release
         ((build . "cargo build --release --features full")
          (package . "cargo package")
          (publish . "cargo publish")))
       (debug
         ((verbose . "RUST_LOG=debug cargo run")
          (trace . "RUST_LOG=trace cargo run")
          (backtrace . "RUST_BACKTRACE=1 cargo run")))))
    (troubleshooting
      ((z3-not-found
         ((symptom . "Error: z3 library not found")
          (cause . "Z3 not installed or not in library path")
          (solution . "Install libz3-dev or build with --no-default-features")))
       (steam-init-failed
         ((symptom . "Steam not available: initialization failed")
          (cause . "Steam client not running or steam_appid.txt missing")
          (solution . "Start Steam client and ensure steam_appid.txt exists")))
       (display-error
         ((symptom . "Failed to create window")
          (cause . "No display available (headless environment)")
          (solution . "Build with --features headless for CI/testing")))
       (bevy-panic
         ((symptom . "thread main panicked at bevy_...")
          (cause . "ECS query or resource access error")
          (solution . "Check system ordering and resource initialization")))))
    (environments
      ((development
         ((features . "default")
          (opt-level . "1")
          (debug . #t)))
       (testing
         ((features . "headless")
          (opt-level . "1")
          (debug . #t)))
       (staging
         ((features . "full")
          (opt-level . "3")
          (debug . #f)))
       (production
         ((features . "full")
          (opt-level . "3")
          (lto . "thin")
          (debug . #f)))))
    (monitoring
      ((logs . "RUST_LOG environment variable controls log level")
       (metrics . "Steam stats for gameplay metrics")
       (errors . "tracing-subscriber for structured logging")))
    (contacts
      ((maintainer . "hyperpolymath")
       (repo . "github.com/hyperpolymath/proof-of-work")
       (issues . "github.com/hyperpolymath/proof-of-work/issues")))))
