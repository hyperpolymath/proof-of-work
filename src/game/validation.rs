// SPDX-License-Identifier: MIT OR Apache-2.0
//! Validation for game board state and piece placement.
//!
//! Provides rules for validating piece placement, wire connections,
//! and overall board state correctness before proof verification.

use super::{BoardState, GoalCondition, Level, LogicPiece};

/// Validation error types for piece placement and board state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// Piece is placed outside board boundaries.
    OutOfBounds { x: u32, y: u32, max_x: u32, max_y: u32 },
    /// Two pieces occupy the same position.
    OverlappingPieces { position: (u32, u32) },
    /// Wire endpoints are invalid.
    InvalidWire { from: (u32, u32), to: (u32, u32), reason: String },
    /// No goals defined on the board.
    NoGoals,
    /// No assumptions defined on the board.
    NoAssumptions,
    /// Gate has no inputs connected.
    DisconnectedGate { position: (u32, u32) },
    /// Goal has no path from assumptions.
    UnreachableGoal { formula: String },
    /// Formula syntax is invalid.
    InvalidFormula { formula: String, reason: String },
}

/// Result of board validation.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    /// Create a valid result with no errors.
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Create an invalid result with errors.
    pub fn invalid(errors: Vec<ValidationError>) -> Self {
        Self {
            is_valid: false,
            errors,
            warnings: Vec::new(),
        }
    }

    /// Add a warning to the result.
    pub fn with_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }
}

/// Validate a piece placement on the board.
pub fn validate_piece_placement(board: &BoardState, piece: &LogicPiece) -> Result<(), ValidationError> {
    let (x, y) = piece.position();

    // Check bounds
    if x >= board.width || y >= board.height {
        return Err(ValidationError::OutOfBounds {
            x,
            y,
            max_x: board.width - 1,
            max_y: board.height - 1,
        });
    }

    // Check for overlap
    if board.is_occupied(x, y) {
        return Err(ValidationError::OverlappingPieces { position: (x, y) });
    }

    // Validate wire-specific rules
    if let LogicPiece::Wire { from, to } = piece {
        // Wire must connect different positions
        if from == to {
            return Err(ValidationError::InvalidWire {
                from: *from,
                to: *to,
                reason: "Wire cannot connect a position to itself".to_string(),
            });
        }

        // Wire endpoints must be in bounds
        if from.0 >= board.width || from.1 >= board.height {
            return Err(ValidationError::InvalidWire {
                from: *from,
                to: *to,
                reason: "Wire start position out of bounds".to_string(),
            });
        }
        if to.0 >= board.width || to.1 >= board.height {
            return Err(ValidationError::InvalidWire {
                from: *from,
                to: *to,
                reason: "Wire end position out of bounds".to_string(),
            });
        }
    }

    // Validate formula syntax for assumptions and goals
    match piece {
        LogicPiece::Assumption { formula, .. } | LogicPiece::Goal { formula, .. } => {
            if formula.is_empty() {
                return Err(ValidationError::InvalidFormula {
                    formula: formula.clone(),
                    reason: "Formula cannot be empty".to_string(),
                });
            }
            // Basic formula validation: must start with alphanumeric or parenthesis
            if !formula.chars().next().map_or(false, |c| c.is_alphanumeric() || c == '(') {
                return Err(ValidationError::InvalidFormula {
                    formula: formula.clone(),
                    reason: "Formula must start with identifier or parenthesis".to_string(),
                });
            }
        }
        _ => {}
    }

    Ok(())
}

/// Validate the entire board state.
pub fn validate_board(board: &BoardState) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Check each piece for basic validity
    for piece in &board.pieces {
        let (x, y) = piece.position();
        if x >= board.width || y >= board.height {
            errors.push(ValidationError::OutOfBounds {
                x,
                y,
                max_x: board.width - 1,
                max_y: board.height - 1,
            });
        }
    }

    // Check for overlapping pieces
    let mut positions: Vec<(u32, u32)> = Vec::new();
    for piece in &board.pieces {
        let pos = piece.position();
        if positions.contains(&pos) {
            errors.push(ValidationError::OverlappingPieces { position: pos });
        } else {
            positions.push(pos);
        }
    }

    // Check for at least one assumption and one goal
    let has_assumptions = board.pieces.iter().any(|p| matches!(p, LogicPiece::Assumption { .. }));
    let has_goals = board.pieces.iter().any(|p| matches!(p, LogicPiece::Goal { .. }));

    if !has_assumptions {
        errors.push(ValidationError::NoAssumptions);
    }
    if !has_goals {
        errors.push(ValidationError::NoGoals);
    }

    // Check for disconnected gates (warning only)
    for piece in &board.pieces {
        if let LogicPiece::AndIntro { position }
        | LogicPiece::OrIntro { position }
        | LogicPiece::ImpliesIntro { position }
        | LogicPiece::NotIntro { position } = piece
        {
            let nearby = board.pieces_near(position.0, position.1, 2);
            let has_input = nearby.iter().any(|p| {
                matches!(
                    p,
                    LogicPiece::Assumption { .. }
                        | LogicPiece::AndIntro { .. }
                        | LogicPiece::OrIntro { .. }
                )
            });
            if !has_input {
                warnings.push(format!(
                    "Gate at ({}, {}) has no nearby input pieces",
                    position.0, position.1
                ));
            }
        }
    }

    if errors.is_empty() {
        let mut result = ValidationResult::valid();
        result.warnings = warnings;
        result
    } else {
        let mut result = ValidationResult::invalid(errors);
        result.warnings = warnings;
        result
    }
}

/// Validate a level definition.
pub fn validate_level(level: &Level) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Validate initial board state
    let board_result = validate_board(&level.initial_state);
    errors.extend(board_result.errors);
    warnings.extend(board_result.warnings);

    // Validate goal condition matches board
    match &level.goal_state {
        GoalCondition::ConnectNodes { start, end } => {
            if start.0 >= level.initial_state.width || start.1 >= level.initial_state.height {
                warnings.push(format!("Goal start node {:?} is outside board bounds", start));
            }
            if end.0 >= level.initial_state.width || end.1 >= level.initial_state.height {
                warnings.push(format!("Goal end node {:?} is outside board bounds", end));
            }
        }
        GoalCondition::ProveFormula { formula } => {
            if formula.is_empty() {
                errors.push(ValidationError::InvalidFormula {
                    formula: formula.clone(),
                    reason: "Goal formula cannot be empty".to_string(),
                });
            }
        }
        GoalCondition::BuildProofTree { depth } => {
            if *depth == 0 {
                warnings.push("Proof tree depth of 0 is trivially satisfied".to_string());
            }
        }
    }

    if errors.is_empty() {
        let mut result = ValidationResult::valid();
        result.warnings = warnings;
        result
    } else {
        let mut result = ValidationResult::invalid(errors);
        result.warnings = warnings;
        result
    }
}

/// Check if a board state is ready for proof verification.
/// Returns true if the board has valid structure for verification.
pub fn is_ready_for_verification(board: &BoardState) -> bool {
    let result = validate_board(board);
    result.is_valid && board.piece_count() >= 3 // At least assumption, gate, and goal
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_board() -> BoardState {
        BoardState {
            width: 10,
            height: 10,
            pieces: vec![
                LogicPiece::Assumption {
                    formula: "P".to_string(),
                    position: (2, 5),
                },
                LogicPiece::Assumption {
                    formula: "Q".to_string(),
                    position: (2, 3),
                },
                LogicPiece::Goal {
                    formula: "R".to_string(),
                    position: (8, 4),
                },
                LogicPiece::AndIntro { position: (5, 4) },
            ],
        }
    }

    #[test]
    fn test_valid_board() {
        let board = make_test_board();
        let result = validate_board(&board);
        assert!(result.is_valid);
    }

    #[test]
    fn test_out_of_bounds() {
        let mut board = make_test_board();
        board.pieces.push(LogicPiece::OrIntro { position: (15, 15) });

        let result = validate_board(&board);
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| matches!(e, ValidationError::OutOfBounds { .. })));
    }

    #[test]
    fn test_overlapping_pieces() {
        let mut board = make_test_board();
        board.pieces.push(LogicPiece::OrIntro { position: (2, 5) }); // Same as first assumption

        let result = validate_board(&board);
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| matches!(e, ValidationError::OverlappingPieces { .. })));
    }

    #[test]
    fn test_no_assumptions() {
        let board = BoardState {
            width: 10,
            height: 10,
            pieces: vec![LogicPiece::Goal {
                formula: "R".to_string(),
                position: (5, 5),
            }],
        };

        let result = validate_board(&board);
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| matches!(e, ValidationError::NoAssumptions)));
    }

    #[test]
    fn test_invalid_wire() {
        let board = BoardState::new(10, 10);
        let wire = LogicPiece::Wire {
            from: (5, 5),
            to: (5, 5), // Same position
        };

        let result = validate_piece_placement(&board, &wire);
        assert!(result.is_err());
    }

    #[test]
    fn test_ready_for_verification() {
        let board = make_test_board();
        assert!(is_ready_for_verification(&board));
    }
}
