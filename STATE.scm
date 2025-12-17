;;; STATE.scm — proof-of-work
;; SPDX-License-Identifier: AGPL-3.0-or-later
;; SPDX-FileCopyrightText: 2025 Jonathan D.A. Jewell

(define metadata
  '((version . "0.1.0") (updated . "2025-12-17") (project . "proof-of-work")))

(define current-position
  '((phase . "v0.1 - Initial Setup")
    (overall-completion . 40)
    (components ((rsr-compliance ((status . "complete") (completion . 100)))
                 (security-fixes ((status . "complete") (completion . 100)))))))

(define blockers-and-issues '((critical ()) (high-priority ())))

(define critical-next-actions
  '((immediate (("Verify CI/CD" . high))) (this-week (("Expand tests" . medium)))))

(define session-history
  '((snapshots ((date . "2025-12-15") (session . "initial") (notes . "SCM files added"))
               ((date . "2025-12-15") (session . "security-fixes")
                (notes . "OpenSSF Scorecard fixes: SHA-pinned actions, fixed CodeQL matrix, removed duplicate workflow"))
               ((date . "2025-12-17") (session . "scm-security-review")
                (notes . "Fixed security.txt expiry, META.scm syntax, added SPDX to guix.scm, created flake.nix")))))

(define state-summary
  '((project . "proof-of-work") (completion . 45) (blockers . 0) (updated . "2025-12-17")))
