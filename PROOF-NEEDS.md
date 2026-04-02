# PROOF-NEEDS.md — proof-of-work

## Current State

- **src/abi/*.idr**: NO
- **Dangerous patterns**: 0
- **LOC**: ~5,300 (Rust/Bevy)
- **ABI layer**: Missing

## What Needs Proving

| Component | What | Why |
|-----------|------|-----|
| Cryptographic verification | Solution verification is sound (no false positives) | The game's core mechanic: "prove your work, literally" |
| Puzzle generation | Generated puzzles always have at least one valid solution | Unsolvable puzzles break the game |
| Network protocol | Multiplayer state sync preserves game integrity | Desynced state ruins multiplayer experience |
| Level progression | Level difficulty is monotonically increasing | Non-monotonic difficulty breaks player progression |

## Recommended Prover

**Idris2** — The game's THEME is cryptographic proof-of-work. Having formal proofs that the verification is sound would be thematically perfect and practically valuable.

## Priority

**LOW** — It is a puzzle game, not safety-critical infrastructure. However, the thematic irony of a "proof-of-work" game lacking formal proofs is notable. Cryptographic verification soundness is the highest-value target.
