# PROOF-NEEDS.md — proof-of-work

## Current State (reconciled 2026-05-20)

The earlier "src/abi/*.idr: NO / ABI layer: Missing" lines were **stale**.
Ground truth:

- **src/abi/\*.idr**: PRESENT — `ProofOfWork.ABI.{Types,Foreign,Invariants}`
  under `src/abi/ProofOfWork/ABI/`, packaged by
  `src/abi/proof-of-work-abi.ipkg`.
- **ABI layer**: present and **machine-checked** (`idris2 --build` green;
  enforced by `.github/workflows/abi-verify.yml`). Before 2026-05-19 the
  seam existed but did NOT build (flat layout: `import ProofOfWork.ABI.Types`
  failed) — it was structurally present but never verified.
- **Dangerous patterns**: 0 in the seam (no `believe_me` / `assert_total` /
  `idris_crash` / `%default partial` in `src/abi/ProofOfWork/ABI/*.idr`,
  verified by grep 2026-05-20).
- **LOC**: ~5,300 (Rust/Bevy).
- **RUST-SPARK-STANCE.adoc**: present (structural compliance documented).
- **Foreign.idr hang**: pre-existing defect, hangs under idris2 0.8.0 when
  type-checked in isolation. `Invariants.idr` does not import it so the
  ABI-verify CI still runs green. Tracked separately, not on the I1–I7
  register.

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
| I3 | `placePiece` preserves board well-formedness (in-bounds + no overlap) | **DISCHARGED** — `placePreservesWF` machine-checked in `Invariants.idr` (PR #60, 2026-05-19); the `all`/`any` cons-distribution lemmas needed for the foldl-based Prelude predicates landed inline as part of that PR; `idris2 --check` green |
| I4 | Every shipped/generated level is solvable | **OWED** — no solver-side existence proof in Rust |
| I5 | Pack difficulty sequence is non-decreasing & in [1,5] | **DISCHARGED** — `decNonDecreasing` (total decision proc) + `builtinPackMonotone : NonDecreasing [1,2,3,4,5]` machine-checked; blanket form correctly retained only as the erased Rust-side obligation |
| I6 | Submission-signature binding (leaderboard integrity) | **ASSUMPTION** — rests on SHA-256 collision/2nd-preimage resistance; `sha256CollisionResistant` is a stated hardness axiom, conditional soundness `signatureBindsPayload` proven under it |
| I7 | Level-pack save/load round-trip identity | **ASSUMPTION** — reframed in PR #62 (2026-05-20) as an explicit serde-correctness postulate (`serdeRoundTripCorrect`); `levelRoundTrip` is now a derived alias rather than a bare OWED postulate. Promotion to a discharged theorem would require either property-testing against the Rust serde implementation or a SPARK proof of the encoder/decoder pair |

## What Needs Proving (priority order)

Each row's "Where" column points at the Rust function carrying a matching
`PROOF-OBLIGATION I_n` comment, so grep for `PROOF-OBLIGATION I2` (etc.)
locates the obligation site from the Rust side; the seam docstring in
`src/abi/ProofOfWork/ABI/Invariants.idr` points the other way.

| Component | What | Why | Maps to | Where |
|-----------|------|-----|---------|-------|
| Mock verifier (I2) | Mock must not accept what Z3 rejects | No-Z3 builds otherwise grant false wins | I2 (defect) | **Rust code defect** — `src/verification/mod.rs::verify_level_solution` (`#[cfg(not(feature="z3-verify"))]` body). Needs design call (mock weakening to a non-accepting verdict, or removal). Not Idris2-tractable in isolation. |
| Cryptographic verification (I1) | Positive verdict returns/justifies a certificate | The game's core mechanic: "prove your work, literally" | I1 | **Rust API change** — `src/verification/mod.rs::verify_level_solution` (`#[cfg(feature="z3-verify")]` body) and `src/verification/z3_integration.rs::verify_formula` both return `bool`; must return the `VerifiedSolution` certificate the seam already types. Idris2 statement waits on that. |
| Puzzle generation (I4) | Generated puzzles always solvable | Unsolvable puzzles break the game | I4 | **Rust solver-side** — readiness check is `src/game/validation.rs::is_ready_for_verification` (necessary but not sufficient). A generator that emits an existence witness alongside the level would inhabit `packLevelsSolvable`; until then the Idris2 statement is intentionally unprovable. |
| Pack round-trip (I7) | `load . save = id` on well-formed packs | Community-pack corruption across disk | I7 (now ASSUMPTION) | `src/levels/mod.rs::LevelPack::save` / `::load`. Discharge route: property-test the Rust serde against `serdeRoundTripCorrect`, or write a SPARK proof of the encoder/decoder pair. Not blocking. |
| Level progression (I5) | Loader must call `decNonDecreasing` per pack | Non-monotonic difficulty breaks progression | I5 (validator done) | **Rust caller-side** — `src/levels/mod.rs::LevelPack::load` must invoke the discharged decision procedure on the loaded difficulty sequence; `LevelPackManager::next_level` is advance-by-index only and intentionally does not re-check. |

I3 is **DISCHARGED** (see register above); not in the remaining-proof list. I6 is an intentional cryptographic-hardness assumption and will not migrate to a theorem under any realistic schedule.

## Recommended Prover

**Idris2** — The game's THEME is cryptographic proof-of-work. Having formal proofs that the verification is sound would be thematically perfect and practically valuable. The Idris2 ABI seam is in place; remaining work is mostly Rust-side (I1, I2, I4 all need API or implementation changes before the corresponding Idris2 statements become inhabitable).

## Priority

**LOW** (severity) but the seam is now real: structural compliance done,
I3 + I5 discharged, I6 and I7 stated as explicit assumptions (cryptographic
hardness, serde correctness), and I1/I2/I4 tracked as erased OWED
obligations under CI (`abi-verify.yml`) so they cannot silently rot into
`believe_me`. Highest-value remaining target is **I2** (mock-vs-Z3
soundness — a known defect, not just an absence). The "proof-of-work game
with no proofs" irony is no longer true.
