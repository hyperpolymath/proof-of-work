;; SPDX-License-Identifier: AGPL-3.0-or-later
;; AGENTIC.scm - AI agent interaction patterns for proof-of-work

(define agentic-config
  `((version . "1.0.0")
    (claude-code
      ((model . "claude-opus-4-5-20251101")
       (tools . ("read" "edit" "bash" "grep" "glob"))
       (permissions . "read-all")))
    (patterns
      ((code-review . "thorough")
       (refactoring . "conservative")
       (testing . "comprehensive")))
    (constraints
      ((languages . ("rust"))
       (banned . ("typescript" "go" "python" "makefile"))))
    (project-specific
      ((build-command . "cargo build")
       (test-command . "cargo test")
       (run-command . "cargo run")
       (format-command . "cargo fmt")
       (lint-command . "cargo clippy")
       (feature-flags . ("default" "z3-verify" "steam" "network" "headless"))))
    (code-patterns
      ((ecs-components . "Use Component derive macro")
       (ecs-resources . "Use Resource derive macro")
       (ecs-systems . "Use fn system_name(Query<...>, Res<...>) pattern")
       (error-handling . "Use Result with custom error types")
       (serialization . "Use serde derive for all data types")
       (documentation . "Document all public items with rustdoc")))
    (testing-patterns
      ((unit-tests . "Place in same file with #[cfg(test)] module")
       (integration-tests . "Place in tests/ directory")
       (fuzz-tests . "Use libfuzzer via cargo-fuzz")))
    (safety-rules
      ((no-unsafe . "Avoid unsafe code except for FFI")
       (feature-gates . "Gate optional dependencies behind features")
       (input-validation . "Validate all user-created level data")))))
