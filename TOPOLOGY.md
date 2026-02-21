<!-- SPDX-License-Identifier: PMPL-1.0-or-later -->
<!-- TOPOLOGY.md — Project architecture map and completion dashboard -->
<!-- Last updated: 2026-02-19 -->

# proof-of-work — Project Topology

## System Architecture

```
                        ┌─────────────────────────────────────────┐
                        │              PLAYER / CLIENT            │
                        │        (Game GUI / Steam Workshop)      │
                        └───────────────────┬─────────────────────┘
                                            │
                                            ▼
                        ┌─────────────────────────────────────────┐
                        │           BEVY GAME ENGINE              │
                        │    (ECS Architecture, Rendering, UI)    │
                        └──────────┬───────────────────┬──────────┘
                                   │                   │
                                   ▼                   ▼
                        ┌───────────────────────┐  ┌────────────────────────────────┐
                        │ GAME LOGIC (RUST)     │  │ VERIFICATION LAYER (Z3)        │
                        │ - Puzzle State        │  │ - SMT Solver Integration       │
                        │ - ECS Systems         │  │ - Constraint Satisfaction      │
                        │ - Player Interaction  │  │ - Mathematical Proof           │
                        └──────────┬────────────┘  └──────────┬─────────────────────┘
                                   │                          │
                                   └────────────┬─────────────┘
                                                ▼
                        ┌─────────────────────────────────────────┐
                        │             LEVEL DATA                  │
                        │  ┌───────────┐  ┌───────────────────┐  │
                        │  │ Puzzle    │  │  Custom Levels    │  │
                        │  │ Specs     │  │  (User-gen)       │  │
                        │  └───────────┘  └───────────────────┘  │
                        └───────────────────┬─────────────────────┘
                                            │
                                            ▼
                        ┌─────────────────────────────────────────┐
                        │          EXTERNAL INTEGRATION           │
                        │      (Steam SDK, Achievements)          │
                        └─────────────────────────────────────────┘

                        ┌─────────────────────────────────────────┐
                        │          REPO INFRASTRUCTURE            │
                        │  Justfile / Cargo   .machine_readable/  │
                        │  Nix / Wolfi        0-AI-MANIFEST.a2ml  │
                        └─────────────────────────────────────────┘
```

## Completion Dashboard

```
COMPONENT                          STATUS              NOTES
─────────────────────────────────  ──────────────────  ─────────────────────────────────
CORE GAME
  Bevy Engine Implementation        ██████████ 100%    ECS framework stable
  Puzzle State Management           ████████░░  80%    Undo/Redo logic refining
  UI Components (bevy_egui)         ██████████ 100%    Functional interface active

VERIFICATION & DATA
  Z3 SMT Integration                ██████████ 100%    Mathematical proof stable
  Constraint Specification          ████████░░  80%    Puzzle DSL verified
  Level Data (.yaml/.json)          ██████████ 100%    Default levels active

REPO INFRASTRUCTURE
  Justfile Automation               ██████████ 100%    Standard build/run tasks
  .machine_readable/                ██████████ 100%    STATE tracking active
  Test Suite (PropTest)             ████████░░  80%    Logic coverage expanding

─────────────────────────────────────────────────────────────────────────────
OVERALL:                            ████████░░  ~80%   Core playable, refining levels
```

## Key Dependencies

```
Puzzle Spec ──────► Z3 Solver ──────► Logic Check ──────► Completion
     │                 │                 │                  │
     ▼                 ▼                 ▼                  ▼
Player Input ────► Bevy ECS ───────► Game State ────────► Renderer
```

## Update Protocol

This file is maintained by both humans and AI agents. When updating:

1. **After completing a component**: Change its bar and percentage
2. **After adding a component**: Add a new row in the appropriate section
3. **After architectural changes**: Update the ASCII diagram
4. **Date**: Update the `Last updated` comment at the top of this file

Progress bars use: `█` (filled) and `░` (empty), 10 characters wide.
Percentages: 0%, 10%, 20%, ... 100% (in 10% increments).
