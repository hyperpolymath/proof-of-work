;; SPDX-License-Identifier: AGPL-3.0-or-later
;; META.scm - Meta-level information for proof-of-work
;; Media-Type: application/meta+scheme

(meta
  (architecture-decisions
    ((id "adr-001")
     (title "Use Bevy ECS for game architecture")
     (status "accepted")
     (date "2024-01-01")
     (context "Need a modern, performant game engine with good Rust ecosystem support")
     (decision "Adopt Bevy 0.17 as the game engine with ECS architecture")
     (consequences
       ("Steep learning curve for ECS patterns"
        "Excellent performance and parallelism"
        "Active community and ecosystem"
        "Hot reloading support for development")))

    ((id "adr-002")
     (title "Z3 SMT solver for proof verification")
     (status "accepted")
     (date "2024-01-01")
     (context "Puzzle solutions need mathematical verification to ensure correctness")
     (decision "Integrate Z3 SMT solver via z3-rs crate with static linking")
     (consequences
       ("Formal verification of logical proofs"
        "Large binary size due to static linking"
        "Can export proofs in SMT-LIB2 and Isabelle formats"
        "Feature-gated to allow builds without Z3")))

    ((id "adr-003")
     (title "Feature flags for optional integrations")
     (status "accepted")
     (date "2024-01-01")
     (context "Steam and network features should be optional for development builds")
     (decision "Use Cargo feature flags: z3-verify, steam, network, headless")
     (consequences
       ("Faster compile times for development"
        "Smaller binaries when features not needed"
        "More complex conditional compilation")))

    ((id "adr-004")
     (title "RON format for level definitions")
     (status "accepted")
     (date "2024-01-01")
     (context "Need human-readable level format that works well with Rust")
     (decision "Use RON (Rusty Object Notation) for level files")
     (consequences
       ("Direct serde integration"
        "Readable by humans"
        "Less widespread than JSON but better Rust fit")))

    ((id "adr-005")
     (title "egui for in-game UI")
     (status "accepted")
     (date "2024-01-01")
     (context "Need immediate-mode UI for editor and menus")
     (decision "Use bevy_egui for all UI components")
     (consequences
       ("Consistent UI across editor and game"
        "Fast iteration on UI design"
        "Limited styling compared to retained-mode UI"))))

  (development-practices
    (code-style
      ("Follow Rust 2021 edition idioms"
       "Use clippy with default lints"
       "Format with rustfmt"
       "Document public APIs with doc comments"
       "Use descriptive variable names"))
    (security
      (principle "Defense in depth")
      (practices
        ("Static Z3 linking avoids dynamic library issues"
         "Feature-gated network code"
         "No unsafe code in game logic"
         "Input validation for user-created levels")))
    (testing
      (unit-tests "src/**/tests.rs modules")
      (integration-tests "tests/ directory")
      (fuzzing "fuzz/ with cargo-fuzz/ClusterFuzzLite")
      (ci "GitHub Actions with Rust CI"))
    (versioning "SemVer")
    (documentation "AsciiDoc for user docs, rustdoc for API")
    (branching
      (main "stable, release-ready code")
      (develop "integration branch")
      (feature "feature/* branches for new work")))

  (design-rationale
    ((why-bevy
      "Bevy provides a modern ECS architecture that maps well to puzzle games.
       Each logic piece is an entity with components for rendering, position,
       and game logic. Systems handle input, verification, and rendering cleanly.")
     (why-z3
      "Z3 provides rigorous mathematical verification of logical proofs.
       Players cannot cheat by finding exploits - if Z3 says the proof is valid,
       it is provably correct. This is core to the proof of work concept.")
     (why-steam
      "Steam Workshop enables community content sharing. Players can create
       and share puzzle packs. Cloud saves and achievements add engagement.")
     (why-smt-export
      "Exporting proofs in SMT-LIB2 format allows verification by any
       SMT solver. Optional Isabelle export enables formal verification
       in proof assistants."))))
