// SPDX-License-Identifier: PMPL-1.0-or-later
//! Level editor for creating and modifying puzzles.

pub mod ui;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::{BoardState, GoalCondition, Level, LogicPiece};
use crate::levels::LevelPack;

/// The state of the level being edited
#[allow(dead_code)]
#[derive(Debug, Clone, Resource)]
pub struct EditorState {
    /// Current level being edited
    pub level: Level,
    /// Level pack this level belongs to (if any)
    pub pack_id: Option<String>,
    /// Whether we're editing an existing level or creating new
    pub is_new_level: bool,
    /// Currently selected tool
    pub tool: EditorTool,
    /// Grid size for the level
    pub grid_width: u32,
    pub grid_height: u32,
    /// Whether to show grid
    pub show_grid: bool,
    /// Currently selected piece type for placement
    pub selected_piece: Option<EditorPieceType>,
    /// Formula input for assumption/goal
    pub formula_input: String,
    /// Variable input for quantifiers
    pub variable_input: String,
    /// Status message
    pub status_message: String,
    /// Whether level has unsaved changes
    pub dirty: bool,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            level: Level {
                id: 1,
                name: "New Level".to_string(),
                description: "Enter description here".to_string(),
                theorem: "".to_string(),
                initial_state: BoardState {
                    width: 10,
                    height: 10,
                    pieces: vec![],
                },
                goal_state: GoalCondition::ProveFormula {
                    formula: "Goal".to_string(),
                },
            },
            pack_id: None,
            is_new_level: true,
            tool: EditorTool::Select,
            grid_width: 10,
            grid_height: 10,
            show_grid: true,
            selected_piece: None,
            formula_input: String::new(),
            variable_input: "x".to_string(),
            status_message: "Ready".to_string(),
            dirty: false,
        }
    }
}

#[allow(dead_code)]
impl EditorState {
    /// Create a new editor state for a blank level
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an editor state from an existing level
    pub fn from_level(level: Level, pack_id: Option<String>) -> Self {
        let grid_width = level.initial_state.width;
        let grid_height = level.initial_state.height;
        Self {
            level,
            pack_id,
            is_new_level: false,
            grid_width,
            grid_height,
            ..Default::default()
        }
    }

    /// Add a piece at the specified position
    pub fn add_piece(&mut self, piece: LogicPiece) {
        // Check if position is already occupied
        let pos = piece.position();
        if !self.is_position_occupied(pos) {
            self.level.initial_state.pieces.push(piece);
            self.dirty = true;
            self.status_message = format!("Added piece at ({}, {})", pos.0, pos.1);
        } else {
            self.status_message = format!("Position ({}, {}) is occupied", pos.0, pos.1);
        }
    }

    /// Remove a piece at the specified position
    pub fn remove_piece_at(&mut self, pos: (u32, u32)) -> bool {
        let initial_len = self.level.initial_state.pieces.len();
        self.level
            .initial_state
            .pieces
            .retain(|p| p.position() != pos);

        if self.level.initial_state.pieces.len() < initial_len {
            self.dirty = true;
            self.status_message = format!("Removed piece at ({}, {})", pos.0, pos.1);
            true
        } else {
            self.status_message = format!("No piece at ({}, {})", pos.0, pos.1);
            false
        }
    }

    /// Check if a position is occupied
    pub fn is_position_occupied(&self, pos: (u32, u32)) -> bool {
        self.level
            .initial_state
            .pieces
            .iter()
            .any(|p| p.position() == pos)
    }

    /// Get piece at position
    pub fn get_piece_at(&self, pos: (u32, u32)) -> Option<&LogicPiece> {
        self.level
            .initial_state
            .pieces
            .iter()
            .find(|p| p.position() == pos)
    }

    /// Update grid size
    pub fn set_grid_size(&mut self, width: u32, height: u32) {
        self.grid_width = width;
        self.grid_height = height;
        self.level.initial_state.width = width;
        self.level.initial_state.height = height;
        self.dirty = true;

        // Remove pieces outside the new grid
        self.level.initial_state.pieces.retain(|p| {
            let (x, y) = p.position();
            x < width && y < height
        });
    }

    /// Validate the level for playability
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = vec![];

        // Check for at least one assumption
        let has_assumption = self
            .level
            .initial_state
            .pieces
            .iter()
            .any(|p| matches!(p, LogicPiece::Assumption { .. }));
        if !has_assumption {
            errors.push("Level needs at least one assumption".to_string());
        }

        // Check for exactly one goal
        let goal_count = self
            .level
            .initial_state
            .pieces
            .iter()
            .filter(|p| matches!(p, LogicPiece::Goal { .. }))
            .count();
        if goal_count == 0 {
            errors.push("Level needs a goal".to_string());
        } else if goal_count > 1 {
            errors.push("Level should have exactly one goal".to_string());
        }

        // Check for name
        if self.level.name.trim().is_empty() {
            errors.push("Level needs a name".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Build the final level
    pub fn build_level(&self) -> Level {
        self.level.clone()
    }
}

/// Editor tools
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditorTool {
    #[default]
    Select,
    Place,
    Delete,
    Move,
}

/// Piece types that can be placed in the editor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditorPieceType {
    Assumption,
    Goal,
    AndIntro,
    OrIntro,
    ImpliesIntro,
    NotIntro,
    ForallIntro,
    ExistsIntro,
}

impl EditorPieceType {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Assumption => "Assumption",
            Self::Goal => "Goal",
            Self::AndIntro => "AND Gate",
            Self::OrIntro => "OR Gate",
            Self::ImpliesIntro => "Implies",
            Self::NotIntro => "NOT",
            Self::ForallIntro => "For All",
            Self::ExistsIntro => "Exists",
        }
    }

    /// Whether this piece type requires a formula input
    pub fn needs_formula(&self) -> bool {
        matches!(self, Self::Assumption | Self::Goal)
    }

    /// Whether this piece type requires a variable input
    pub fn needs_variable(&self) -> bool {
        matches!(self, Self::ForallIntro | Self::ExistsIntro)
    }

    /// Create a LogicPiece from this type at the given position
    pub fn to_logic_piece(&self, pos: (u32, u32), formula: &str, variable: &str) -> LogicPiece {
        match self {
            Self::Assumption => LogicPiece::Assumption {
                formula: formula.to_string(),
                position: pos,
            },
            Self::Goal => LogicPiece::Goal {
                formula: formula.to_string(),
                position: pos,
            },
            Self::AndIntro => LogicPiece::AndIntro { position: pos },
            Self::OrIntro => LogicPiece::OrIntro { position: pos },
            Self::ImpliesIntro => LogicPiece::ImpliesIntro { position: pos },
            Self::NotIntro => LogicPiece::NotIntro { position: pos },
            Self::ForallIntro => LogicPiece::ForallIntro {
                position: pos,
                variable: variable.to_string(),
            },
            Self::ExistsIntro => LogicPiece::ExistsIntro {
                position: pos,
                variable: variable.to_string(),
            },
        }
    }
}

/// Marker component for editor entities
#[derive(Component)]
pub struct EditorEntity;

/// Event for testing a level from the editor
#[derive(bevy::prelude::Message, Clone)]
pub struct TestLevelEvent;

/// Event for saving a level
#[derive(bevy::prelude::Message, Clone)]
pub struct SaveLevelEvent {
    pub to_pack_id: Option<String>,
}

/// Create a new level pack for user-created levels
#[allow(dead_code)]
pub fn create_user_pack(name: &str, author: &str) -> LevelPack {
    let id = name
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect::<String>();

    LevelPack::new(&id, name, author)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_state_default() {
        let state = EditorState::default();
        assert!(state.is_new_level);
        assert_eq!(state.grid_width, 10);
        assert_eq!(state.grid_height, 10);
    }

    #[test]
    fn test_add_remove_piece() {
        let mut state = EditorState::default();

        let piece = LogicPiece::Assumption {
            formula: "P".to_string(),
            position: (5, 5),
        };
        state.add_piece(piece);

        assert_eq!(state.level.initial_state.pieces.len(), 1);
        assert!(state.dirty);

        assert!(state.remove_piece_at((5, 5)));
        assert_eq!(state.level.initial_state.pieces.len(), 0);
    }

    #[test]
    fn test_validate_level() {
        let mut state = EditorState::default();

        // Empty level should fail
        assert!(state.validate().is_err());

        // Add assumption
        state.add_piece(LogicPiece::Assumption {
            formula: "P".to_string(),
            position: (0, 0),
        });
        assert!(state.validate().is_err()); // Still no goal

        // Add goal
        state.add_piece(LogicPiece::Goal {
            formula: "Q".to_string(),
            position: (5, 5),
        });
        assert!(state.validate().is_ok());
    }
}
