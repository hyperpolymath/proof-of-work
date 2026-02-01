;; SPDX-License-Identifier: PMPL-1.0-or-later
;; SPDX-FileCopyrightText: 2025 hyperpolymath
;;
;; Guix package definition for proof-of-work
;; Build: guix build -f guix.scm
;; Shell: guix shell -D -f guix.scm
;; Install: guix package -f guix.scm

(use-modules (guix packages)
             (guix gexp)
             (guix git-download)
             (guix build-system cargo)
             (guix licenses)
             (gnu packages rust)
             (gnu packages rust-apps)
             (gnu packages pkg-config)
             (gnu packages compression)
             (gnu packages linux)
             (gnu packages audio)
             (gnu packages maths)
             (gnu packages xorg)
             (gnu packages freedesktop))

(define-public proof-of-work
  (package
    (name "proof-of-work")
    (version "0.1.0")
    (source
     (local-file "." "proof-of-work-checkout"
                 #:recursive? #t
                 #:select? (git-predicate ".")))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-build-flags '("--features" "headless")
       #:cargo-test-flags '("--features" "headless")
       #:phases
       (modify-phases %standard-phases
         (add-after 'unpack 'set-env
           (lambda _
             (setenv "BEVY_HEADLESS" "1"))))))
    (native-inputs
     (list pkg-config rust rust-cargo))
    (inputs
     (list z3
           alsa-lib
           eudev))
    (synopsis "Proof-of-Work puzzle game library")
    (description
     "A puzzle game library built with Bevy game engine.  Includes optional
Z3-based formal verification of puzzle solutions.  Supports headless mode
for server/CI environments and full graphics mode for gameplay.")
    (home-page "https://github.com/hyperpolymath/proof-of-work")
    (license agpl3+)))

;; Development variant with full graphics support
(define-public proof-of-work-dev
  (package
    (inherit proof-of-work)
    (name "proof-of-work-dev")
    (arguments
     `(#:cargo-build-flags '("--features" "z3-verify")
       #:phases
       (modify-phases %standard-phases
         (add-after 'unpack 'set-env
           (lambda* (#:key inputs #:allow-other-keys)
             (setenv "LD_LIBRARY_PATH"
                     (string-join
                      (list (string-append (assoc-ref inputs "vulkan-loader") "/lib")
                            (string-append (assoc-ref inputs "wayland") "/lib")
                            (string-append (assoc-ref inputs "libxkbcommon") "/lib"))
                      ":")))))))
    (inputs
     (list z3
           alsa-lib
           eudev
           libx11
           libxcursor
           libxrandr
           libxi
           wayland
           libxkbcommon
           vulkan-loader))
    (synopsis "Proof-of-Work puzzle game (development version)")
    (description
     "Development version of proof-of-work with full graphics support.")))

proof-of-work
