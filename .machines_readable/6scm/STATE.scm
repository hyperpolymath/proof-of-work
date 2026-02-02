;; SPDX-License-Identifier: PMPL-1.0-or-later
;; STATE.scm - Project state for proof-of-work
;; Media-Type: application/vnd.state+scm

(state
  (metadata
    (version "0.1.0")
    (schema-version "1.0")
    (created "2026-01-03")
    (updated "2026-01-04")
    (project "proof-of-work")
    (repo "github.com/hyperpolymath/proof-of-work"))

  (project-context
    (name "proof-of-work")
    (tagline "Puzzle game where solutions are cryptographically verified")
    (tech-stack
      ("Rust 2021 edition"
       "Bevy 0.17 game engine"
       "bevy_egui 0.38 for UI"
       "Z3 SMT solver for verification"
       "Steamworks SDK for Steam integration"
       "tokio/reqwest for networking"
       "serde/ron/json for serialization")))

  (current-position
    (phase "active-development")
    (overall-completion 45)
    (components
      ((name "game-core") (status "implemented") (completion 80))
      ((name "logic-pieces") (status "implemented") (completion 90))
      ((name "board-system") (status "implemented") (completion 75))
      ((name "verification-engine") (status "implemented") (completion 70))
      ((name "level-system") (status "implemented") (completion 85))
      ((name "level-editor") (status "implemented") (completion 80))
      ((name "ui-system") (status "partial") (completion 60))
      ((name "steam-integration") (status "implemented") (completion 70))
      ((name "network-client") (status "partial") (completion 40))
      ((name "fuzzing") (status "configured") (completion 50)))
    (working-features
      ("Game state machine"
       "Logic piece rendering"
       "Level pack loading/saving"
       "Progress tracking"
       "Level editor"
       "Z3-powered verification"
       "SMT-LIB2 proof export"
       "Steam achievements"
       "Network proof submission")))

  (route-to-mvp
    (milestones
      ((id "m1") (name "v0.1.0 - Foundation") (status "in-progress") (completion 45))
      ((id "m2") (name "v0.2.0 - Steam Integration") (status "pending"))
      ((id "m3") (name "v1.0.0 - Stable Release") (status "pending"))))

  (blockers-and-issues
    (critical)
    (high ((id "H1") (description "Headless testing needs feature flag")))
    (medium ((id "M1") (description "Network uses blocking thread spawn")))
    (low ((id "L1") (description "Some modules are placeholder stubs"))))

  (critical-next-actions
    (immediate ("Add more tutorial levels" "Polish level select UI"))
    (this-week ("Editor UX improvements" "Keyboard shortcuts"))
    (this-month ("Steam Workshop" "Multiplayer sharing")))

  (session-history
    ((date "2026-01-04")
     (accomplishments ("Populated SCM files with project metadata")))))
