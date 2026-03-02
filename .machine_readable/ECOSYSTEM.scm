;; SPDX-License-Identifier: PMPL-1.0-or-later
;; ECOSYSTEM.scm - Ecosystem relationships for proof-of-work
;; Media type: application/vnd.ecosystem+scm

(ecosystem
  (metadata
    ((version . "1.0.0")
     (name . "proof-of-work")
     (type . "game")
     (purpose . "Puzzle game with formal verification of solutions")))

  (position-in-ecosystem
    "Demonstrates Z3 SMT solver integration in games within the hyperpolymath suite")

  (related-projects
    ((proven . "verification-infrastructure")
     (proven-servers . "server-side-verification")
     (idaptik . "sibling-game-project")
     (hypatia . "ci-cd-scanner")))

  (what-this-is
    "A puzzle game built with Bevy engine where player solutions are verified "
    "using Z3 SMT solver, proving mathematical correctness of completed puzzles")

  (what-this-is-not
    "Not a cryptocurrency proof-of-work implementation"
    "Not a general-purpose verification framework"))
