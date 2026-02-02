; SPDX-License-Identifier: PMPL-1.0-or-later
; NEUROSYM.scm - Neurosymbolic context for proof-of-work
; Media type: application/vnd.neurosym+scm

(neurosym
  (metadata
    (version "1.0.0")
    (schema-version "1.0")
    (created "2026-01-30")
    (updated "2026-01-30"))

  (conceptual-model
    (domain "cryptography")
    (subdomain "automation")
    (core-concepts
      (concept "tool"
        (definition "A software component that automates tasks")
        (properties "input" "output" "configuration"))))

  (knowledge-graph-hints
    (entities "proof-of-work" "Rust" "automation")
    (relationships
      ("proof-of-work" provides "automation-capabilities"))))
