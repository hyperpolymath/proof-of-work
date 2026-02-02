; SPDX-License-Identifier: PMPL-1.0-or-later
; AGENTIC.scm - AI agent instructions for proof-of-work
; Media type: application/vnd.agentic+scm

(agentic
  (metadata
    (version "1.0.0")
    (schema-version "1.0")
    (created "2026-01-30")
    (updated "2026-01-30"))

  (agent-identity
    (project "proof-of-work")
    (role "development-assistant")
    (capabilities "Code review" "Testing" "Documentation" "Security"))

  (language-policy
    (allowed
      (language "Rust" (use-case "primary implementation"))
      (language "Guile Scheme" (use-case "SCM files")))
    (banned
      (language "TypeScript" (replacement "ReScript"))
      (language "Python" (replacement "Rust"))
      (language "Go" (replacement "Rust"))))

  (code-standards
    (general
      (line-endings "LF")
      (indent "spaces")
      (spdx-headers required)))

  (prohibited-actions
    "Never introduce banned languages"
    "Never remove SPDX headers"))
