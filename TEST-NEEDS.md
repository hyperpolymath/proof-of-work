# TEST-NEEDS.md — proof-of-work

> Generated 2026-03-29 by punishing audit.

## Current State

| Category     | Count | Notes |
|-------------|-------|-------|
| Unit tests   | ~20   | Inline `#[test]`: editor(3), board(6), validation(6), levels(3), verification(2) |
| Integration  | 0     | None |
| E2E          | 0     | None |
| Benchmarks   | 1     | benches/proof_of_work_bench.rs — **PLACEHOLDER** (`black_box(42)`, no real logic) |

**Source modules:** ~18 Rust source files covering: editor, game (board, validation), levels, verification + main.

## What's Missing

### P2P (Property-Based) Tests
- [ ] Board: property tests for valid board states (no impossible configurations)
- [ ] Validation: property tests for rule consistency
- [ ] Levels: property tests for level solvability
- [ ] Verification: property tests for proof verification soundness

### E2E Tests
- [ ] Full game: start level -> play -> solve -> verify -> next level
- [ ] Editor: create level -> validate -> save -> load -> play
- [ ] Verification: generate proof -> submit -> verify -> accept/reject

### Aspect Tests
- **Security:** No tests for level data tampering, verification bypass, editor injection
- **Performance:** Benchmark exists but is FAKE (`black_box(42)`). No real performance measurement
- **Concurrency:** N/A (single-player game)
- **Error handling:** No tests for malformed level data, invalid moves, corrupted save

### Build & Execution
- [ ] `cargo test`
- [ ] Fuzz target execution (`fuzz/` exists)

### Benchmarks Needed
- [ ] Level solving time by difficulty
- [ ] Board state validation throughput
- [ ] Proof verification time
- [ ] Editor operation responsiveness

### Self-Tests
- [ ] All bundled levels are solvable
- [ ] Verification accepts valid proofs and rejects invalid ones

## Priority

**HIGH.** 18 modules with 20 inline tests is decent unit coverage. BUT the benchmark is a fraud — `black_box(42)` measures nothing. The game logic (board + validation) has the most tests (12 combined), which is good. Missing: all E2E (nobody tested actually playing the game), the fake benchmark needs replacement, fuzz targets need execution.
