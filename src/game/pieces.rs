// SPDX-License-Identifier: MIT OR Apache-2.0

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub enum LogicPiece {
    // Basic building blocks
    Assumption {
        formula: String,
        position: (u32, u32),
    },
    Goal {
        formula: String,
        position: (u32, u32),
    },

    // Logical operators (movable pieces)
    AndIntro {
        position: (u32, u32),
    },
    OrIntro {
        position: (u32, u32),
    },
    ImpliesIntro {
        position: (u32, u32),
    },
    NotIntro {
        position: (u32, u32),
    },

    // Quantifiers
    ForallIntro {
        position: (u32, u32),
        variable: String,
    },
    ExistsIntro {
        position: (u32, u32),
        variable: String,
    },

    // Connectors
    Wire {
        from: (u32, u32),
        to: (u32, u32),
    },
}

impl LogicPiece {
    pub fn position(&self) -> (u32, u32) {
        match self {
            Self::Assumption { position, .. } => *position,
            Self::Goal { position, .. } => *position,
            Self::AndIntro { position } => *position,
            Self::OrIntro { position } => *position,
            Self::ImpliesIntro { position } => *position,
            Self::NotIntro { position } => *position,
            Self::ForallIntro { position, .. } => *position,
            Self::ExistsIntro { position, .. } => *position,
            Self::Wire { from, .. } => *from,
        }
    }

    pub fn set_position(&mut self, new_pos: (u32, u32)) {
        match self {
            Self::Assumption { position, .. } => *position = new_pos,
            Self::Goal { position, .. } => *position = new_pos,
            Self::AndIntro { position } => *position = new_pos,
            Self::OrIntro { position } => *position = new_pos,
            Self::ImpliesIntro { position } => *position = new_pos,
            Self::NotIntro { position } => *position = new_pos,
            Self::ForallIntro { position, .. } => *position = new_pos,
            Self::ExistsIntro { position, .. } => *position = new_pos,
            Self::Wire { from, .. } => *from = new_pos,
        }
    }

    pub fn to_smt(&self) -> String {
        match self {
            Self::Assumption { formula, .. } => format!("(assert {})", formula),
            Self::Goal { formula, .. } => format!("(assert (not {}))", formula),
            Self::AndIntro { .. } => "(and _ _)".to_string(),
            Self::OrIntro { .. } => "(or _ _)".to_string(),
            Self::ImpliesIntro { .. } => "(=> _ _)".to_string(),
            Self::NotIntro { .. } => "(not _)".to_string(),
            Self::ForallIntro { variable, .. } => format!("(forall (({} Int)) _)", variable),
            Self::ExistsIntro { variable, .. } => format!("(exists (({} Int)) _)", variable),
            Self::Wire { .. } => "".to_string(),
        }
    }

    pub fn label(&self) -> String {
        match self {
            Self::Assumption { formula, .. } => formula.clone(),
            Self::Goal { formula, .. } => formula.clone(),
            Self::AndIntro { .. } => "AND".to_string(),
            Self::OrIntro { .. } => "OR".to_string(),
            Self::ImpliesIntro { .. } => "=>".to_string(),
            Self::NotIntro { .. } => "NOT".to_string(),
            Self::ForallIntro { variable, .. } => format!("∀{}", variable),
            Self::ExistsIntro { variable, .. } => format!("∃{}", variable),
            Self::Wire { .. } => "-".to_string(),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            LogicPiece::Assumption { .. } => Color::srgb(0.3, 0.8, 0.3), // Green
            LogicPiece::Goal { .. } => Color::srgb(0.8, 0.3, 0.3),       // Red
            LogicPiece::AndIntro { .. } => Color::srgb(0.5, 0.5, 0.9),   // Blue
            LogicPiece::OrIntro { .. } => Color::srgb(0.9, 0.5, 0.5),    // Light red
            LogicPiece::ImpliesIntro { .. } => Color::srgb(0.9, 0.9, 0.3), // Yellow
            _ => Color::srgb(0.6, 0.6, 0.6),                             // Gray
        }
    }
}

/// Visual representation bundle for pieces
#[derive(Bundle)]
pub struct PieceBundle {
    pub piece: LogicPiece,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl PieceBundle {
    pub fn new(piece: LogicPiece, _asset_server: &AssetServer) -> Self {
        let (x, y) = piece.position();
        let color = piece.color();

        Self {
            piece,
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            transform: Transform::from_xyz(x as f32 * 80.0, y as f32 * 80.0, 0.0),
        }
    }
}
