---
title: Proof of Work
template: default
---

# Proof of Work

**A puzzle game where solutions are cryptographically verified — prove your work, literally.**

## What Is It?

Proof of Work is a logic puzzle game built with [Bevy](https://bevyengine.org/) where players solve constraint-based puzzles. What makes it unique: every solution is verified using the [Z3 SMT solver](https://github.com/Z3Prover/z3), producing a mathematical certificate that the solution is correct.

## Architecture

```
proof-of-work/
├── src/
│   ├── game/           Game logic: board, pieces, validation
│   ├── verification/   Z3 SMT solver integration
│   ├── editor/         Level editor with live preview
│   ├── levels/         Level pack management and UI
│   ├── ui/             HUD, menus, completion screens
│   ├── network/        Proof submission and leaderboards
│   └── steam/          Steam achievements and stats
└── levels/             Puzzle definitions (RON format)
```

## Key Features

| Feature | Description |
|---------|-------------|
| **Z3 Verification** | Solutions verified by SMT solver, exported as SMT-LIB2 |
| **Bevy 0.18 ECS** | Modern entity-component-system game architecture |
| **Level Editor** | Create and share custom puzzles |
| **Steam Integration** | Achievements, stats, and leaderboards (optional) |
| **Network Play** | Submit proofs to global leaderboard (optional) |
| **Headless Mode** | Run without display for CI/CD testing |

## How Verification Works

1. Player places logic pieces on the puzzle grid
2. Board connections are checked for validity
3. Z3 encodes the puzzle constraints as SMT-LIB2 formulas
4. Solver proves satisfiability — the solution is mathematically correct
5. Proof is exported and optionally submitted to the leaderboard

## Feature Flags

Build with different feature combinations:

```bash
# Default: Z3 verification only
cargo build --release

# Headless (CI/CD testing, no display)
cargo build --release --no-default-features --features headless

# Full: Z3 + Steam + Network
cargo build --release --features full
```

## Tech Stack

- **Language:** Rust 2021 edition
- **Engine:** Bevy 0.18
- **UI:** bevy_egui 0.39
- **Verification:** Z3 0.19 (statically linked)
- **Serialization:** serde + RON + JSON
- **License:** PMPL-1.0-or-later

## Links

- [Source Code](https://github.com/hyperpolymath/proof-of-work)
- [Issue Tracker](https://github.com/hyperpolymath/proof-of-work/issues)
- [Bevy Engine](https://bevyengine.org/)
- [Z3 Prover](https://github.com/Z3Prover/z3)
