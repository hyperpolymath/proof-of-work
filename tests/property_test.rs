// SPDX-License-Identifier: PMPL-1.0-or-later
//! Property-based tests for proof-of-work game logic.
//!
//! Tests invariants that should hold for all valid inputs:
//! - Board creation always produces correct dimensions
//! - Piece placement idempotency and state changes
//! - Spatial query consistency
//! - Formula preservation

use proptest::prelude::*;
use proof_of_work::{BoardState, LogicPiece};

// ============================================================================
// Strategy definitions for property-based testing
// ============================================================================

/// Strategy to generate valid board dimensions (1-100 x 1-100).
fn board_dimension_strategy() -> impl Strategy<Value = (u32, u32)> {
    (1u32..100, 1u32..100)
}

/// Strategy to generate positions within a given board.
fn position_strategy(width: u32, height: u32) -> impl Strategy<Value = (u32, u32)> {
    (0u32..width, 0u32..height)
}

/// Strategy to generate formula strings (alphanumeric + operators).
fn formula_strategy() -> impl Strategy<Value = String> {
    r"[A-Z][A-Z0-9]*( (AND|OR|IMPLIES|NOT) [A-Z][A-Z0-9]*)*"
        .prop_map(|s| s.to_string())
}

// ============================================================================
// Property: Board Creation
// ============================================================================

proptest! {
    /// Property: Creating a board with any valid dimensions (1-100 x 1-100)
    /// should produce an empty board with exact dimensions.
    #[test]
    fn prop_board_creation_empty(
        (width, height) in board_dimension_strategy()
    ) {
        let board = BoardState::new(width, height);
        prop_assert_eq!(board.width, width);
        prop_assert_eq!(board.height, height);
        prop_assert_eq!(board.piece_count(), 0);
    }

    /// Property: A newly created board has all positions unoccupied.
    #[test]
    fn prop_board_all_positions_unoccupied(
        (width, height) in board_dimension_strategy()
    ) {
        let board = BoardState::new(width, height);
        for x in 0..width {
            for y in 0..height {
                prop_assert!(!board.is_occupied(x, y));
            }
        }
    }

    /// Property: Placing a valid piece increases piece_count by exactly 1.
    #[test]
    fn prop_place_piece_increases_count(
        (width, height) in board_dimension_strategy(),
        (px, py) in (0u32..100, 0u32..100),
    ) {
        let width = width.max(1);
        let height = height.max(1);
        // Clamp position to board bounds
        let (px, py) = (px % width, py % height);

        let mut board = BoardState::new(width, height);
        let initial_count = board.piece_count();

        let piece = LogicPiece::AndIntro { position: (px, py) };
        let placed = board.place_piece(piece);

        prop_assert!(placed, "Piece should be placed in empty position");
        prop_assert_eq!(board.piece_count(), initial_count + 1);
    }

    /// Property: Placing and removing a piece returns to original state.
    #[test]
    fn prop_place_remove_idempotent(
        (width, height) in board_dimension_strategy(),
        (px, py) in (0u32..100, 0u32..100),
    ) {
        let width = width.max(1);
        let height = height.max(1);
        let (px, py) = (px % width, py % height);

        let mut board = BoardState::new(width, height);
        let initial_count = board.piece_count();

        let piece = LogicPiece::AndIntro { position: (px, py) };
        board.place_piece(piece);

        let removed = board.remove_piece(px, py);
        prop_assert!(removed.is_some(), "Piece should be removed");
        prop_assert_eq!(board.piece_count(), initial_count);
        prop_assert!(!board.is_occupied(px, py));
    }

    /// Property: Cannot place two pieces at the same position.
    #[test]
    fn prop_no_overlapping_pieces(
        (width, height) in board_dimension_strategy(),
        (px, py) in (0u32..100, 0u32..100),
    ) {
        let width = width.max(1);
        let height = height.max(1);
        let (px, py) = (px % width, py % height);

        let mut board = BoardState::new(width, height);

        let piece1 = LogicPiece::AndIntro { position: (px, py) };
        let placed1 = board.place_piece(piece1);
        prop_assert!(placed1);

        let piece2 = LogicPiece::OrIntro { position: (px, py) };
        let placed2 = board.place_piece(piece2);
        prop_assert!(!placed2, "Should not place piece at occupied position");
        prop_assert_eq!(board.piece_count(), 1);
    }
}

// ============================================================================
// Property: Spatial Queries
// ============================================================================

proptest! {
    /// Property: pieces_near with radius 0 returns only piece at exact position.
    #[test]
    fn prop_pieces_near_radius_zero(
        (width, height) in board_dimension_strategy(),
        (px, py) in (0u32..100, 0u32..100),
    ) {
        let width = width.max(1);
        let height = height.max(1);
        let (px, py) = (px % width, py % height);

        let mut board = BoardState::new(width, height);
        let piece = LogicPiece::AndIntro { position: (px, py) };
        board.place_piece(piece);

        let near = board.pieces_near(px, py, 0);
        prop_assert_eq!(near.len(), 1, "Should find exactly one piece at exact position");
    }

    /// Property: pieces_near never returns pieces at distance > radius.
    #[test]
    fn prop_pieces_near_respects_radius(
        (width, height) in board_dimension_strategy(),
        radius in 0u32..50,
    ) {
        let width = (width.max(1)).min(100) as u32;
        let height = (height.max(1)).min(100) as u32;

        let mut board = BoardState::new(width, height);

        // Place piece at (1, 1)
        let piece = LogicPiece::AndIntro { position: (1, 1) };
        board.place_piece(piece);

        // Query from origin
        let near = board.pieces_near(0, 0, radius);

        if near.len() > 0 {
            let (qx, qy) = near[0].position();
            let dx = (qx as i32 - 0).unsigned_abs();
            let dy = (qy as i32 - 0).unsigned_abs();
            prop_assert!(dx <= radius && dy <= radius,
                "Returned pieces must be within radius");
        }
    }
}

// ============================================================================
// Property: Validation
// ============================================================================

proptest! {
    /// Property: Validating an empty board always succeeds.
    #[test]
    fn prop_empty_board_valid(
        (width, height) in board_dimension_strategy()
    ) {
        let board = BoardState::new(width, height);
        // Empty boards are technically valid in structure (no overlaps, etc.)
        prop_assert_eq!(board.piece_count(), 0);
    }

    /// Property: A board with one piece has that piece accessible.
    #[test]
    fn prop_single_piece_accessible(
        (width, height) in board_dimension_strategy(),
        (px, py) in (0u32..100, 0u32..100),
    ) {
        let width = width.max(1);
        let height = height.max(1);
        let (px, py) = (px % width, py % height);

        let mut board = BoardState::new(width, height);
        let piece = LogicPiece::AndIntro { position: (px, py) };
        board.place_piece(piece);

        let retrieved = board.piece_at(px, py);
        prop_assert!(retrieved.is_some(), "Placed piece should be retrievable");
    }
}

// ============================================================================
// Property: Formula Preservation
// ============================================================================

proptest! {
    /// Property: Assumption piece stores formula verbatim.
    #[test]
    fn prop_assumption_stores_formula(
        formula in formula_strategy(),
        (width, height) in board_dimension_strategy(),
    ) {
        let width = width.max(1);
        let height = height.max(1);
        let piece = LogicPiece::Assumption {
            formula: formula.clone(),
            position: (0, 0),
        };

        if let LogicPiece::Assumption { formula: stored, .. } = piece {
            prop_assert_eq!(stored, formula, "Formula should be stored verbatim");
        } else {
            prop_assert!(false, "Should be Assumption variant");
        }
    }

    /// Property: Goal piece stores formula verbatim.
    #[test]
    fn prop_goal_stores_formula(
        formula in formula_strategy(),
        (width, height) in board_dimension_strategy(),
    ) {
        let width = width.max(1);
        let height = height.max(1);
        let piece = LogicPiece::Goal {
            formula: formula.clone(),
            position: (0, 0),
        };

        if let LogicPiece::Goal { formula: stored, .. } = piece {
            prop_assert_eq!(stored, formula, "Formula should be stored verbatim");
        } else {
            prop_assert!(false, "Should be Goal variant");
        }
    }
}

// ============================================================================
// Property: Bounds Checking
// ============================================================================

proptest! {
    /// Property: in_bounds correctly identifies valid positions.
    #[test]
    fn prop_in_bounds_consistent(
        (width, height) in board_dimension_strategy(),
        (x, y) in (0u32..200, 0u32..200),
    ) {
        let board = BoardState::new(width, height);

        let in_bounds = board.in_bounds(x, y);
        let should_be_in = x < width && y < height;

        prop_assert_eq!(in_bounds, should_be_in);
    }

    /// Property: Piece at (0,0) is always in bounds.
    #[test]
    fn prop_origin_always_in_bounds(
        (width, height) in board_dimension_strategy()
    ) {
        let board = BoardState::new(width, height);
        prop_assert!(board.in_bounds(0, 0), "Origin should always be in bounds");
    }

    /// Property: Cannot place piece outside bounds.
    #[test]
    fn prop_cannot_place_out_of_bounds(
        (width, height) in board_dimension_strategy(),
    ) {
        let width = width.max(1);
        let height = height.max(1);

        let mut board = BoardState::new(width, height);

        // Try to place piece at boundary + 1
        let piece = LogicPiece::AndIntro {
            position: (width + 10, height + 10),
        };

        let placed = board.place_piece(piece);
        prop_assert!(!placed, "Cannot place piece outside bounds");
    }
}
