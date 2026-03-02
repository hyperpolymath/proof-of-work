;; SPDX-License-Identifier: PMPL-1.0-or-later
;; NEUROSYM.scm - Neurosymbolic context for proof-of-work
;; Media type: application/vnd.neurosym+scm

(neurosym
  (metadata
    (version "1.0.0")
    (schema-version "1.0")
    (created "2026-01-30")
    (updated "2026-03-02"))

  (conceptual-model
    (domain "game-verification")
    (subdomain "formal-methods")
    (core-concepts
      (concept "puzzle"
        (definition "A logic puzzle with constraints that must be satisfied")
        (properties "grid" "pieces" "constraints" "solution"))
      (concept "proof"
        (definition "A Z3-verified certificate that a solution is mathematically valid")
        (properties "smt2-encoding" "satisfiability" "model"))))

  (knowledge-graph-hints
    (entities "proof-of-work" "Bevy" "Z3" "SMT-LIB2" "puzzle" "verification")
    (relationships
      ("proof-of-work" uses "Bevy" for "game-engine")
      ("proof-of-work" uses "Z3" for "solution-verification")
      ("proof-of-work" exports "SMT-LIB2" as "proof-format"))))
