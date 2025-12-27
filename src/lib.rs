// SPDX-License-Identifier: MIT OR Apache-2.0
//! Proof of Work - A logic puzzle game with cryptographic verification
//!
//! This library exposes the core game logic for testing and embedding.

pub mod game;
pub mod verification;

// Re-export commonly used types
pub use game::{BoardState, GoalCondition, Level, LogicPiece};
pub use verification::ExportedProof;
