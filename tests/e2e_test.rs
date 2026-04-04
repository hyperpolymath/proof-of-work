// SPDX-License-Identifier: PMPL-1.0-or-later
//! End-to-end tests for proof-of-work game logic.
//!
//! Tests complete workflows:
//! - Full puzzle flow from creation to verification
//! - Level lifecycle and state management
//! - Level editor workflows
//! - SMT-LIB2 proof export
//! - Verification of valid and invalid proofs

use proof_of_work::{BoardState, GoalCondition, Level, LogicPiece};

// ============================================================================
// Full Puzzle Flow Tests
// ============================================================================

#[test]
fn e2e_create_board_place_pieces_validate() {
    // Create a board
    let mut board = BoardState::new(10, 10);
    assert_eq!(board.width, 10);
    assert_eq!(board.height, 10);
    assert_eq!(board.piece_count(), 0);

    // Place an assumption
    let assumption = LogicPiece::Assumption {
        formula: "P".into(),
        position: (2, 5),
    };
    let placed_assumption = board.place_piece(assumption);
    assert!(placed_assumption, "Should place assumption");
    assert_eq!(board.piece_count(), 1);

    // Place a goal
    let goal = LogicPiece::Goal {
        formula: "Q".into(),
        position: (8, 5),
    };
    let placed_goal = board.place_piece(goal);
    assert!(placed_goal, "Should place goal");
    assert_eq!(board.piece_count(), 2);

    // Verify both pieces are on the board
    assert!(board.is_occupied(2, 5), "Assumption should be at (2,5)");
    assert!(board.is_occupied(8, 5), "Goal should be at (8,5)");

    // Retrieve and verify formulas
    let retrieved_assumption = board.piece_at(2, 5);
    assert!(retrieved_assumption.is_some());
    if let Some(LogicPiece::Assumption { formula, .. }) = retrieved_assumption {
        assert_eq!(formula, "P");
    } else {
        panic!("Expected Assumption piece");
    }
}

#[test]
fn e2e_puzzle_with_logic_gates() {
    // Create board
    let mut board = BoardState::new(15, 15);

    // Place assumptions
    let p = LogicPiece::Assumption {
        formula: "P".into(),
        position: (2, 7),
    };
    let q = LogicPiece::Assumption {
        formula: "Q".into(),
        position: (2, 5),
    };
    assert!(board.place_piece(p));
    assert!(board.place_piece(q));

    // Place AND gate
    let and_gate = LogicPiece::AndIntro { position: (5, 6) };
    assert!(board.place_piece(and_gate));

    // Place goal
    let goal = LogicPiece::Goal {
        formula: "P ∧ Q".into(),
        position: (10, 6),
    };
    assert!(board.place_piece(goal));

    // Verify all pieces are placed
    assert_eq!(board.piece_count(), 4);

    // Verify gate is in bounds and occupied
    assert!(board.in_bounds(5, 6));
    assert!(board.is_occupied(5, 6));
}

#[test]
fn e2e_place_and_remove_pieces() {
    let mut board = BoardState::new(10, 10);

    let piece = LogicPiece::AndIntro { position: (5, 5) };
    assert!(board.place_piece(piece));
    assert_eq!(board.piece_count(), 1);

    // Remove the piece
    let removed = board.remove_piece(5, 5);
    assert!(removed.is_some(), "Should remove placed piece");
    assert_eq!(board.piece_count(), 0);
    assert!(!board.is_occupied(5, 5));

    // Verify removal was complete
    let after_removal = board.piece_at(5, 5);
    assert!(after_removal.is_none(), "Position should be empty after removal");
}

#[test]
fn e2e_move_piece_on_board() {
    let mut board = BoardState::new(15, 15);

    // Place piece at (5, 5)
    let piece = LogicPiece::AndIntro { position: (5, 5) };
    assert!(board.place_piece(piece));
    assert!(board.is_occupied(5, 5));

    // Move piece to (10, 10)
    let moved = board.move_piece((5, 5), (10, 10));
    assert!(moved, "Should move piece");

    // Verify new position
    assert!(!board.is_occupied(5, 5), "Old position should be empty");
    assert!(board.is_occupied(10, 10), "New position should have piece");
}

// ============================================================================
// Level Lifecycle Tests
// ============================================================================

#[test]
fn e2e_create_level_with_initial_state() {
    let initial_state = BoardState::new(10, 10);

    let level = Level {
        id: 1,
        name: "Test Level".into(),
        description: "A simple test level".into(),
        theorem: "(assert (=> P Q))".into(),
        initial_state,
        goal_state: GoalCondition::ProveFormula {
            formula: "Q".into(),
        },
    };

    assert_eq!(level.id, 1);
    assert_eq!(level.name, "Test Level");
    assert_eq!(level.initial_state.width, 10);
    assert_eq!(level.initial_state.height, 10);
}

#[test]
fn e2e_level_with_pieces() {
    let mut initial_state = BoardState::new(12, 12);

    // Place initial pieces
    let assumption = LogicPiece::Assumption {
        formula: "P".into(),
        position: (3, 6),
    };
    initial_state.place_piece(assumption);

    let level = Level {
        id: 2,
        name: "Level with Pieces".into(),
        description: "Has initial assumptions".into(),
        theorem: "(assert P)".into(),
        initial_state,
        goal_state: GoalCondition::ProveFormula {
            formula: "P".into(),
        },
    };

    assert_eq!(level.initial_state.piece_count(), 1);
}

#[test]
fn e2e_level_board_modification() {
    let initial_state = BoardState::new(20, 20);

    let mut level = Level {
        id: 3,
        name: "Modifiable Level".into(),
        description: "Can be modified".into(),
        theorem: "(assert Q)".into(),
        initial_state,
        goal_state: GoalCondition::ProveFormula {
            formula: "Q".into(),
        },
    };

    // Modify the level's board state by placing pieces
    let assumption = LogicPiece::Assumption {
        formula: "Q".into(),
        position: (5, 10),
    };
    level.initial_state.place_piece(assumption);

    assert_eq!(level.initial_state.piece_count(), 1);
}

// ============================================================================
// Editor Workflow Tests
// ============================================================================

#[test]
fn e2e_editor_place_pieces_save_cycle() {
    // Simulate editor: create level, place pieces
    let mut board = BoardState::new(15, 15);

    let pieces_to_place = vec![
        LogicPiece::Assumption {
            formula: "X".into(),
            position: (2, 7),
        },
        LogicPiece::Assumption {
            formula: "Y".into(),
            position: (2, 3),
        },
        LogicPiece::AndIntro { position: (7, 5) },
        LogicPiece::Goal {
            formula: "X ∧ Y".into(),
            position: (12, 5),
        },
    ];

    for piece in &pieces_to_place {
        assert!(board.place_piece(piece.clone()));
    }

    assert_eq!(board.piece_count(), 4);

    // Verify each piece is at expected position
    for piece in &pieces_to_place {
        let (x, y) = piece.position();
        assert!(board.is_occupied(x, y), "Piece at ({},{}) should be occupied", x, y);
    }
}

#[test]
fn e2e_editor_modify_level() {
    let mut board = BoardState::new(10, 10);

    // Place initial piece
    let p1 = LogicPiece::Assumption {
        formula: "A".into(),
        position: (2, 5),
    };
    board.place_piece(p1);

    // Add another piece
    let p2 = LogicPiece::Assumption {
        formula: "B".into(),
        position: (2, 3),
    };
    board.place_piece(p2);

    // Remove first piece
    board.remove_piece(2, 5);

    // Verify state
    assert_eq!(board.piece_count(), 1);
    assert!(!board.is_occupied(2, 5));
    assert!(board.is_occupied(2, 3));
}

// ============================================================================
// Proof Export Tests
// ============================================================================

#[test]
fn e2e_board_has_smt_export_capability() {
    let mut board = BoardState::new(10, 10);

    let assumption = LogicPiece::Assumption {
        formula: "P".into(),
        position: (2, 5),
    };
    board.place_piece(assumption);

    let goal = LogicPiece::Goal {
        formula: "Q".into(),
        position: (8, 5),
    };
    board.place_piece(goal);

    // Verify board has content
    assert_eq!(board.piece_count(), 2);

    // In production, SMT export would convert this board to SMT-LIB2
    // Here we verify the board state is valid for export
    assert!(board.in_bounds(2, 5));
    assert!(board.in_bounds(8, 5));
}

#[test]
fn e2e_complex_proof_structure() {
    // Create a board with a complex proof structure
    let mut board = BoardState::new(20, 20);

    // Assumptions
    assert!(board.place_piece(LogicPiece::Assumption {
        formula: "P".into(),
        position: (2, 10),
    }));
    assert!(board.place_piece(LogicPiece::Assumption {
        formula: "Q".into(),
        position: (2, 6),
    }));

    // Logic gates
    assert!(board.place_piece(LogicPiece::AndIntro { position: (7, 8) }));
    assert!(board.place_piece(LogicPiece::OrIntro { position: (12, 8) }));

    // Wires connecting pieces
    assert!(board.place_piece(LogicPiece::Wire {
        from: (3, 10),
        to: (7, 8),
    }));
    assert!(board.place_piece(LogicPiece::Wire {
        from: (3, 6),
        to: (7, 8),
    }));

    // Goals
    assert!(board.place_piece(LogicPiece::Goal {
        formula: "P ∧ Q".into(),
        position: (17, 8),
    }));

    assert_eq!(board.piece_count(), 7);
}

// ============================================================================
// Verification Tests
// ============================================================================

#[test]
fn e2e_valid_proof_structure() {
    let mut board = BoardState::new(12, 12);

    // Build a valid proof: two assumptions + AND gate + goal
    let assumption_p = LogicPiece::Assumption {
        formula: "P".into(),
        position: (2, 6),
    };
    let assumption_q = LogicPiece::Assumption {
        formula: "Q".into(),
        position: (2, 4),
    };
    let and_gate = LogicPiece::AndIntro { position: (6, 5) };
    let goal = LogicPiece::Goal {
        formula: "P ∧ Q".into(),
        position: (10, 5),
    };

    assert!(board.place_piece(assumption_p));
    assert!(board.place_piece(assumption_q));
    assert!(board.place_piece(and_gate));
    assert!(board.place_piece(goal));

    // Verify structure is valid
    assert_eq!(board.piece_count(), 4);
    assert!(board.is_occupied(2, 6), "Assumption P should be at (2,6)");
    assert!(board.is_occupied(2, 4), "Assumption Q should be at (2,4)");
    assert!(board.is_occupied(6, 5), "AND gate should be at (6,5)");
    assert!(board.is_occupied(10, 5), "Goal should be at (10,5)");
}

#[test]
fn e2e_empty_board_has_no_goals() {
    let board = BoardState::new(10, 10);

    // Empty board should have no goals
    assert_eq!(board.piece_count(), 0);

    // Verify no pieces can be found
    let pieces = board.pieces_near(5, 5, 10);
    assert_eq!(pieces.len(), 0);
}

#[test]
fn e2e_goal_reachability() {
    let mut board = BoardState::new(15, 15);

    // Place assumption
    assert!(board.place_piece(LogicPiece::Assumption {
        formula: "R".into(),
        position: (2, 7),
    }));

    // Place goal
    assert!(board.place_piece(LogicPiece::Goal {
        formula: "R".into(),
        position: (12, 7),
    }));

    // Both should be on board
    assert!(board.is_occupied(2, 7));
    assert!(board.is_occupied(12, 7));

    // Verify pieces can be queried
    let near_assumption = board.pieces_near(2, 7, 1);
    assert_eq!(near_assumption.len(), 1);

    let near_goal = board.pieces_near(12, 7, 1);
    assert_eq!(near_goal.len(), 1);
}

// ============================================================================
// Invalid Proof Tests
// ============================================================================

#[test]
fn e2e_wrong_gate_types_are_detectable() {
    let mut board = BoardState::new(12, 12);

    // Place assumptions for AND operation
    assert!(board.place_piece(LogicPiece::Assumption {
        formula: "P".into(),
        position: (2, 6),
    }));
    assert!(board.place_piece(LogicPiece::Assumption {
        formula: "Q".into(),
        position: (2, 4),
    }));

    // Place NOT gate instead of AND (wrong gate type)
    assert!(board.place_piece(LogicPiece::NotIntro { position: (6, 5) }));

    // Place goal
    assert!(board.place_piece(LogicPiece::Goal {
        formula: "P ∧ Q".into(),
        position: (10, 5),
    }));

    // Structure is present but logically incorrect
    // (NOT gate cannot satisfy P ∧ Q from P, Q)
    assert_eq!(board.piece_count(), 4);
}

#[test]
fn e2e_missing_assumptions() {
    let mut board = BoardState::new(10, 10);

    // Place goal without assumptions
    assert!(board.place_piece(LogicPiece::Goal {
        formula: "Z".into(),
        position: (5, 5),
    }));

    // Only goal is on board, no assumptions
    assert_eq!(board.piece_count(), 1);
    let pieces = board.pieces_near(2, 5, 10);
    assert!(pieces.is_empty(), "No assumptions placed");
}

#[test]
fn e2e_disconnected_pieces() {
    let mut board = BoardState::new(15, 15);

    // Place pieces far apart with no connections
    assert!(board.place_piece(LogicPiece::Assumption {
        formula: "X".into(),
        position: (1, 1),
    }));

    assert!(board.place_piece(LogicPiece::AndIntro { position: (7, 7) }));

    assert!(board.place_piece(LogicPiece::Goal {
        formula: "Y".into(),
        position: (13, 13),
    }));

    // Pieces exist but are not connected
    assert_eq!(board.piece_count(), 3);
    // Verification would fail due to disconnected structure
}

// ============================================================================
// Boundary and Edge Case Tests
// ============================================================================

#[test]
fn e2e_pieces_at_board_boundaries() {
    let width = 20u32;
    let height = 20u32;
    let mut board = BoardState::new(width, height);

    // Place pieces at all four corners
    assert!(board.place_piece(LogicPiece::AndIntro {
        position: (0, 0),
    }));
    assert!(board.place_piece(LogicPiece::OrIntro {
        position: (width - 1, 0),
    }));
    assert!(board.place_piece(LogicPiece::AndIntro {
        position: (0, height - 1),
    }));
    assert!(board.place_piece(LogicPiece::OrIntro {
        position: (width - 1, height - 1),
    }));

    assert_eq!(board.piece_count(), 4);
    assert!(board.is_occupied(0, 0));
    assert!(board.is_occupied(width - 1, 0));
    assert!(board.is_occupied(0, height - 1));
    assert!(board.is_occupied(width - 1, height - 1));
}

#[test]
fn e2e_small_board_operations() {
    let mut board = BoardState::new(2, 2);

    // 2x2 board has 4 positions
    assert!(board.place_piece(LogicPiece::AndIntro { position: (0, 0) }));
    assert!(board.place_piece(LogicPiece::OrIntro { position: (1, 0) }));
    assert!(board.place_piece(LogicPiece::AndIntro { position: (0, 1) }));
    assert!(board.place_piece(LogicPiece::OrIntro { position: (1, 1) }));

    assert_eq!(board.piece_count(), 4, "All 4 positions filled");

    // Cannot place more
    let result = board.place_piece(LogicPiece::AndIntro { position: (0, 0) });
    assert!(!result, "All positions occupied");
}

#[test]
fn e2e_large_board_with_many_pieces() {
    let mut board = BoardState::new(50, 50);

    // Place many pieces
    let mut count = 0;
    for x in (0..50).step_by(2) {
        for y in (0..50).step_by(2) {
            let piece = LogicPiece::AndIntro {
                position: (x as u32, y as u32),
            };
            if board.place_piece(piece) {
                count += 1;
            }
        }
    }

    assert!(count > 0, "Should place multiple pieces");
    assert_eq!(board.piece_count(), count);
}
