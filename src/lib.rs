// SPDX-License-Identifier: PMPL-1.0-or-later
//! Proof of Work - A logic puzzle game with cryptographic verification
//!
//! This library exposes the core game logic for testing and embedding.

pub mod editor;
pub mod game;
pub mod levels;
pub mod states;
pub mod verification;

// Re-export commonly used types
pub use editor::EditorState;
pub use game::{BoardState, GoalCondition, Level, LogicPiece};
pub use levels::{LevelPack, LevelPackManager};
pub use verification::ExportedProof;
