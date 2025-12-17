;; SPDX-License-Identifier: MIT OR AGPL-3.0-or-later
;; SPDX-FileCopyrightText: 2025 Jonathan D.A. Jewell
;;; guix.scm — proof-of-work Guix Package Definition
;; Run: guix shell -D -f guix.scm

(use-modules (guix packages)
             (guix gexp)
             (guix git-download)
             (guix build-system cargo)
             ((guix licenses) #:prefix license:)
             (gnu packages base))

(define-public proof_of_work
  (package
    (name "proof-of-work")
    (version "0.1.0")
    (source (local-file "." "proof-of-work-checkout"
                        #:recursive? #t
                        #:select? (git-predicate ".")))
    (build-system cargo-build-system)
    (synopsis "Puzzle game with cryptographic solution verification")
    (description "A puzzle game where solutions are cryptographically verified using
Z3 SMT solver integration.  Features include Steam integration, multiplayer support,
level editor, and puzzle sharing.")
    (home-page "https://github.com/hyperpolymath/proof-of-work")
    (license license:agpl3+)))

;; Return package for guix shell
proof_of_work
