// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod board;
pub mod pieces;
pub mod validation;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

pub use pieces::*;

// Level definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub theorem: String,
    pub initial_state: BoardState,
    pub goal_state: GoalCondition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardState {
    pub width: u32,
    pub height: u32,
    pub pieces: Vec<LogicPiece>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalCondition {
    ConnectNodes { start: (u32, u32), end: (u32, u32) },
    ProveFormula { formula: String },
    BuildProofTree { depth: u32 },
}

// Components
#[derive(Component)]
pub struct CurrentLevel(pub Level);

#[derive(Component)]
pub struct PlayerCursor {
    pub position: Vec2,
    pub selected_piece: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct PlayerStats {
    pub proofs_completed: u32,
    pub levels_completed: u32,
    pub total_playtime_secs: u64,
    pub last_level_time_secs: u64,
    pub level_start_time: Option<Instant>,
}

impl PlayerStats {
    pub fn start_level(&mut self) {
        self.level_start_time = Some(Instant::now());
    }

    pub fn complete_level(&mut self) {
        if let Some(start_time) = self.level_start_time {
            self.last_level_time_secs = start_time.elapsed().as_secs();
            self.level_start_time = None;
        }
    }
}

// Marker component for cleanup
#[derive(Component)]
pub struct GameEntity;

// Marker for player-placed pieces
#[derive(Component)]
pub struct PlayerPlaced;

/// Placeable piece types for the UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceablePiece {
    AndGate,
    OrGate,
    Wire,
}

/// Resource to track selected piece type for placement
#[derive(Resource, Default)]
pub struct SelectedPieceType {
    pub piece_type: Option<PlaceablePiece>,
}
