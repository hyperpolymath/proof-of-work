# PROOF-NEEDS.md — proof-of-work

## Current State (reconciled 2026-05-19)

The earlier "src/abi/*.idr: NO / ABI layer: Missing" lines were **stale**.
Ground truth:

- **src/abi/\*.idr**: PRESENT — `ProofOfWork.ABI.{Types,Foreign,Invariants}`
  under `src/abi/ProofOfWork/ABI/`, packaged by
  `src/abi/proof-of-work-abi.ipkg`.
- **ABI layer**: present and **machine-checked** (`idris2 --build` green;
  enforced by `.github/workflows/abi-verify.yml`). Before 2026-05-19 the
  seam existed but did NOT build (flat layout: `import ProofOfWork.ABI.Types`
  failed) — it was structurally present but never verified.
- **Dangerous patterns**: 0. **LOC**: ~5,300 (Rust/Bevy).
- **RUST-SPARK-STANCE.adoc**: present (structural compliance documented).

## Invariant register (I1–I7) — status of the seam obligations

`src/abi/.../Invariants.idr` cross-references these IDs; this is the
authoritative status table. **DISCHARGED** = machine-checked proof in the
seam; **ASSUMPTION** = stated cryptographic hardness axiom (intentionally
unprovable); **OWED** = stated as an erased obligation the Rust does not yet
establish.

| ID | Property | Status |
|----|----------|--------|
| I1 | Verification soundness — a positive verdict implies a real `VerifiedSolution` certificate (adjacency + SMT entailment) | **OWED** — Rust does not return the witness; certificate type defined, refinement obligation open |
| I2 | Mock verifier no weaker than the Z3 path (no false wins in no-Z3 builds) | **OWED + KNOWN-VIOLATED** — mock accepts on connectivity alone; highest-value defect |
| I3 | `placePiece` preserves board well-formedness (in-bounds + no overlap) | **OWED (dischargeable)** — true theorem; needs `all`/`any` cons-distribution lemmas |
| I4 | Every shipped/generated level is solvable | **OWED** — no solver-side existence proof in Rust |
| I5 | Pack difficulty sequence is non-decreasing & in [1,5] | **DISCHARGED** — `decNonDecreasing` (total decision proc) + `builtinPackMonotone : NonDecreasing [1,2,3,4,5]` machine-checked; blanket form correctly retained only as the erased Rust-side obligation |
| I6 | Submission-signature binding (leaderboard integrity) | **ASSUMPTION** — rests on SHA-256 collision/2nd-preimage resistance; `sha256CollisionResistant` is a stated hardness axiom, conditional soundness `signatureBindsPayload` proven under it |
| I7 | Level-pack save/load round-trip identity | **OWED** — serde assumed correct; target for a future property/SPARK proof |

## What Needs Proving (priority order)

| Component | What | Why | Maps to |
|-----------|------|-----|---------|
| Mock verifier (I2) | Mock must not accept what Z3 rejects | No-Z3 builds otherwise grant false wins | I2 (defect) |
| Cryptographic verification (I1) | Positive verdict returns/justifies a certificate | The game's core mechanic: "prove your work, literally" | I1 |
| Board safety (I3) | Discharge `placePreservesWF` (cons-lemmas) | Silent board corruption | I3 |
| Puzzle generation (I4) | Generated puzzles always solvable | Unsolvable puzzles break the game | I4 |
| Pack round-trip (I7) | `load . save = id` on well-formed packs | Community-pack corruption across disk | I7 |
| Level progression (I5) | Loader must call `decNonDecreasing` per pack | Non-monotonic difficulty breaks progression | I5 (validator done) |

## Recommended Prover

**Idris2** — The game's THEME is cryptographic proof-of-work. Having formal proofs that the verification is sound would be thematically perfect and practically valuable.

## Priority

**LOW** (severity) but the seam is now real: structural compliance done,
I5 discharged, I6 stated as an explicit hardness assumption, and I1–I4/I7
tracked as erased OWED obligations under CI (`abi-verify.yml`) so they
cannot silently rot into `believe_me`. Highest-value remaining target is
**I2** (mock-vs-Z3 soundness — a known defect, not just an absence).
The "proof-of-work game with no proofs" irony is no longer true.
