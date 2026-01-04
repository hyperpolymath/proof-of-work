;; SPDX-License-Identifier: AGPL-3.0-or-later
;; ECOSYSTEM.scm - Ecosystem position for proof-of-work
;; Media-Type: application/vnd.ecosystem+scm

(ecosystem
  (version "1.0")
  (name "proof-of-work")
  (type "game")
  (purpose "Logic puzzle game with cryptographic proof verification")

  (position-in-ecosystem
    (category "games")
    (subcategory "puzzle")
    (unique-value
      ("First puzzle game using Z3 SMT solver for verification"
       "Proofs are mathematically verified, not just pattern-matched"
       "Educational tool for learning propositional and predicate logic"
       "Community content via Steam Workshop")))

  (related-projects
    ((name "bevy")
     (relationship "dependency")
     (role "Game engine providing ECS architecture, rendering, input handling"))
    ((name "z3")
     (relationship "dependency")
     (role "SMT solver for proof verification"))
    ((name "bevy_egui")
     (relationship "dependency")
     (role "Immediate-mode UI for editor and menus"))
    ((name "steamworks-rs")
     (relationship "optional-dependency")
     (role "Steam SDK bindings for achievements, leaderboards, workshop"))
    ((name "januskey")
     (relationship "sibling-standard")
     (role "Authentication standard for cross-platform identity"))
    ((name "bunsenite")
     (relationship "inspiration")
     (role "Nickel-based configuration management approach"))
    ((name "affinescript")
     (relationship "potential-consumer")
     (role "Could use proof-of-work verification as test suite")))

  (what-this-is
    ("A puzzle game where players construct logical proofs"
     "Solutions are verified by Z3 SMT solver for mathematical correctness"
     "Includes level editor for creating and sharing puzzles"
     "Steam integration for achievements, leaderboards, and Workshop"
     "Educational tool for learning formal logic"
     "Competitive speedrunning with verified solutions"))

  (what-this-is-not
    ("Not a blockchain or cryptocurrency application"
     "Not a proof assistant like Coq or Isabelle (though exports to them)"
     "Not a casual match-3 style puzzle game"
     "Not an online-only game (works fully offline)"
     "Not a programming game (uses visual puzzle pieces)")))
