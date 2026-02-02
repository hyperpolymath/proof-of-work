;; SPDX-License-Identifier: PMPL-1.0-or-later
;; ECOSYSTEM.scm - Ecosystem relationships for proof-of-work
;; Media type: application/vnd.ecosystem+scm

(ecosystem
  (metadata
    ((version . "1.0.0")
     (name . "proof-of-work")
     (type . "cryptography-tool")
     (purpose . "Part of hyperpolymath tool ecosystem")))
  
  (position-in-ecosystem
    "Provides cryptography-tool functionality within the hyperpolymath suite")
  
  (related-projects
    ((disinfo-nesy-detector . "verification-consumer")))
  
  (what-this-is
    "proof-of-work is a specialized tool in the hyperpolymath ecosystem")
  
  (what-this-is-not
    "Not a general-purpose framework"
    "Not intended as standalone product"))
