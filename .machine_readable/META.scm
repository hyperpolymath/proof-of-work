;; SPDX-License-Identifier: PMPL-1.0-or-later
;; META.scm - Meta-level information for proof-of-work
;; Media type: application/meta+scheme

(define meta
  `((metadata
      ((version . "1.0.0")
       (schema-version . "1.0")
       (created . "2026-01-30")
       (updated . "2026-03-02")
       (project . "proof-of-work")))
    (architecture-decisions
      ((adr-001 . ((status . "accepted")
                   (date . "2026-01-30")
                   (context . "Game engine selection for puzzle game with formal verification")
                   (decision . "Use Bevy ECS engine with Z3 SMT solver for verification")
                   (consequences . "Modern ECS architecture, static Z3 linking, optional Steam/network features")))
       (adr-002 . ((status . "accepted")
                   (date . "2026-03-02")
                   (context . "License selection for hyperpolymath game project")
                   (decision . "Use PMPL-1.0-or-later (Palimpsest License)")
                   (consequences . "Consistent with entire hyperpolymath ecosystem")))))
    (development-practices
      ((code-style . "rustfmt default")
       (security . "cargo-audit, ClusterFuzzLite, hypatia-scan")
       (testing . "cargo test, fuzzing")
       (versioning . "semantic versioning")
       (documentation . "rustdoc, AsciiDoc")
       (branching . "GitHub Flow")
       (containers . "Chainguard Wolfi, Podman")))
    (design-rationale
      ((why-bevy . "Modern ECS architecture, Rust-native, active community")
       (why-z3 . "Industry-standard SMT solver, static linking for portability")
       (why-feature-flags . "Optional Steam/network support for headless testing")))))
