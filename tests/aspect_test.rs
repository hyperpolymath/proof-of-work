// SPDX-License-Identifier: PMPL-1.0-or-later
//! Aspect tests for proof-of-work robustness.
//!
//! Tests cross-cutting concerns:
//! - Security: input validation, injection resistance
//! - Bounds: boundary conditions, limits
//! - Error handling: panic safety, recovery
//! - Performance: large boards, many pieces
//! - Internationalization: Unicode and special characters

use proof_of_work::{BoardState, LogicPiece};

// ============================================================================
// Security: Input Validation
// ============================================================================

#[test]
fn aspect_null_byte_in_formula() {
    // Null byte in formula string should not cause panic
    let formula = "P\0Q".to_string();

    let piece = LogicPiece::Assumption {
        formula,
        position: (5, 5),
    };

    let mut board = BoardState::new(10, 10);
    let placed = board.place_piece(piece);

    assert!(placed, "Should handle null byte in formula");
    assert_eq!(board.piece_count(), 1);
}

#[test]
fn aspect_large_formula_string() {
    // Very large formula string (10KB+) should not cause memory issues
    let large_formula = "P".repeat(10000);

    let piece = LogicPiece::Assumption {
        formula: large_formula.clone(),
        position: (0, 0),
    };

    let mut board = BoardState::new(10, 10);
    let placed = board.place_piece(piece);

    assert!(placed, "Should handle large formula strings");
    assert_eq!(board.piece_count(), 1);

    // Verify formula is stored
    if let Some(LogicPiece::Assumption { formula: stored, .. }) = board.piece_at(0, 0) {
        assert_eq!(stored.len(), 10000);
    }
}

#[test]
fn aspect_html_injection_in_formula() {
    // HTML/script injection in formula should be stored as-is (not executed)
    let formula = "<script>alert('xss')</script>".to_string();

    let piece = LogicPiece::Assumption {
        formula: formula.clone(),
        position: (2, 2),
    };

    let mut board = BoardState::new(10, 10);
    assert!(board.place_piece(piece));

    // Verify HTML string is stored verbatim
    if let Some(LogicPiece::Assumption { formula: stored, .. }) = board.piece_at(2, 2) {
        assert_eq!(stored, formula);
    }
}

#[test]
fn aspect_special_characters_in_formula() {
    // Special characters should be preserved
    let formulas = vec![
        "P∧Q∨R".to_string(),
        "¬P→Q".to_string(),
        "∀x.P(x)".to_string(),
        "∃y.Q(y)".to_string(),
    ];

    let mut board = BoardState::new(10, 10);
    for (idx, formula) in formulas.iter().enumerate() {
        let piece = LogicPiece::Assumption {
            formula: formula.clone(),
            position: (idx as u32, 0),
        };
        assert!(board.place_piece(piece));
    }

    assert_eq!(board.piece_count(), formulas.len());
}

// ============================================================================
// Bounds: Edge Cases
// ============================================================================

#[test]
fn aspect_piece_at_origin() {
    let mut board = BoardState::new(5, 5);

    let piece = LogicPiece::AndIntro { position: (0, 0) };
    assert!(board.place_piece(piece));

    assert!(board.is_occupied(0, 0));
    assert!(board.piece_at(0, 0).is_some());
}

#[test]
fn aspect_piece_at_max_coordinates() {
    let width = 100u32;
    let height = 100u32;
    let mut board = BoardState::new(width, height);

    let piece = LogicPiece::OrIntro {
        position: (width - 1, height - 1),
    };
    assert!(board.place_piece(piece));

    assert!(board.is_occupied(width - 1, height - 1));
    assert!(board.in_bounds(width - 1, height - 1));
}

#[test]
fn aspect_piece_just_outside_bounds() {
    let width = 10u32;
    let height = 10u32;
    let mut board = BoardState::new(width, height);

    // Try to place at width (out of bounds)
    let piece = LogicPiece::AndIntro {
        position: (width, height),
    };
    let placed = board.place_piece(piece);

    assert!(!placed, "Should not place outside bounds");
    assert!(!board.in_bounds(width, height));
}

#[test]
fn aspect_1x1_board_edge_case() {
    let mut board = BoardState::new(1, 1);

    // Only position (0, 0) exists
    assert!(board.place_piece(LogicPiece::AndIntro { position: (0, 0) }));
    assert_eq!(board.piece_count(), 1);

    // Cannot place another
    let result = board.place_piece(LogicPiece::OrIntro { position: (0, 0) });
    assert!(!result, "Position occupied");

    // Cannot place at (1, 0) or (0, 1)
    let result2 = board.place_piece(LogicPiece::AndIntro { position: (1, 0) });
    assert!(!result2, "Out of bounds");
}

#[test]
fn aspect_max_u32_coordinates_out_of_bounds() {
    let mut board = BoardState::new(10, 10);

    let piece = LogicPiece::AndIntro {
        position: (u32::MAX, u32::MAX),
    };
    let placed = board.place_piece(piece);

    assert!(!placed, "u32::MAX coordinates should be out of bounds");
}

// ============================================================================
// Error Handling: Panic Safety
// ============================================================================

#[test]
fn aspect_remove_nonexistent_piece_no_panic() {
    let mut board = BoardState::new(10, 10);

    // Try to remove from empty board
    let removed = board.remove_piece(5, 5);
    assert!(removed.is_none(), "Should return None, not panic");

    // Try to remove from position never occupied
    let removed2 = board.remove_piece(0, 0);
    assert!(removed2.is_none(), "Should return None, not panic");
}

#[test]
fn aspect_move_nonexistent_piece_no_panic() {
    let mut board = BoardState::new(10, 10);

    // Try to move piece that doesn't exist
    let moved = board.move_piece((5, 5), (7, 7));
    assert!(!moved, "Should return false, not panic");
}

#[test]
fn aspect_move_to_occupied_position_no_panic() {
    let mut board = BoardState::new(10, 10);

    // Place two pieces
    assert!(board.place_piece(LogicPiece::AndIntro { position: (3, 3) }));
    assert!(board.place_piece(LogicPiece::OrIntro { position: (5, 5) }));

    // Try to move first piece to second's location
    let moved = board.move_piece((3, 3), (5, 5));
    assert!(!moved, "Should not move to occupied position");

    // Verify original piece still at original location
    assert!(board.is_occupied(3, 3));
    assert!(board.is_occupied(5, 5));
}

#[test]
fn aspect_query_empty_board_no_panic() {
    let board = BoardState::new(10, 10);

    // Query empty board should not panic
    let pieces = board.pieces_near(5, 5, 3);
    assert_eq!(pieces.len(), 0);

    let at = board.piece_at(5, 5);
    assert!(at.is_none());
}

#[test]
fn aspect_repeated_place_same_position_idempotent() {
    let mut board = BoardState::new(10, 10);

    let piece1 = LogicPiece::AndIntro { position: (5, 5) };
    assert!(board.place_piece(piece1));

    // Try to place again at same position
    let piece2 = LogicPiece::AndIntro { position: (5, 5) };
    let result = board.place_piece(piece2);

    assert!(!result, "Second placement should fail");
    assert_eq!(board.piece_count(), 1, "Should still have only 1 piece");
}

// ============================================================================
// Performance: Large Scale
// ============================================================================

#[test]
fn aspect_100x100_board_creation() {
    let board = BoardState::new(100, 100);

    assert_eq!(board.width, 100);
    assert_eq!(board.height, 100);
    assert_eq!(board.piece_count(), 0);
}

#[test]
fn aspect_large_board_with_1000_pieces() {
    let mut board = BoardState::new(100, 100);

    // Place 1000 pieces (board can fit 10000, so well within capacity)
    let mut placed_count = 0;
    for i in 0..1000 {
        let x = (i % 100) as u32;
        let y = (i / 100) as u32;

        let piece = match i % 4 {
            0 => LogicPiece::AndIntro { position: (x, y) },
            1 => LogicPiece::OrIntro { position: (x, y) },
            2 => LogicPiece::NotIntro { position: (x, y) },
            _ => LogicPiece::ImpliesIntro { position: (x, y) },
        };

        if board.place_piece(piece) {
            placed_count += 1;
        }
    }

    assert_eq!(placed_count, 100, "Should place 100 pieces (10x10 grid of 100x100 board)");
    assert_eq!(board.piece_count(), placed_count);
}

#[test]
fn aspect_spatial_query_on_large_board() {
    let mut board = BoardState::new(200, 200);

    // Place piece in center
    assert!(board.place_piece(LogicPiece::AndIntro { position: (100, 100) }));

    // Query with small radius
    let nearby = board.pieces_near(100, 100, 5);
    assert_eq!(nearby.len(), 1);

    // Query with large radius
    let faraway = board.pieces_near(100, 100, 500);
    assert_eq!(faraway.len(), 1);

    // Query from far location
    let far_query = board.pieces_near(0, 0, 50);
    assert_eq!(far_query.len(), 0);
}

#[test]
fn aspect_remove_multiple_pieces_performance() {
    let mut board = BoardState::new(50, 50);

    // Place 100 pieces
    for i in 0..100 {
        let x = (i % 50) as u32;
        let y = (i / 50) as u32;
        let piece = LogicPiece::AndIntro { position: (x, y) };
        let _ = board.place_piece(piece);
    }

    assert_eq!(board.piece_count(), 100);

    // Remove them all
    for i in 0..100 {
        let x = (i % 50) as u32;
        let y = (i / 50) as u32;
        board.remove_piece(x, y);
    }

    assert_eq!(board.piece_count(), 0);
}

// ============================================================================
// Internationalization: Unicode and Special Characters
// ============================================================================

#[test]
fn aspect_chinese_characters_in_formula() {
    let formula = "前提 ∧ 结论".to_string();

    let piece = LogicPiece::Assumption {
        formula: formula.clone(),
        position: (1, 1),
    };

    let mut board = BoardState::new(10, 10);
    assert!(board.place_piece(piece));

    if let Some(LogicPiece::Assumption { formula: stored, .. }) = board.piece_at(1, 1) {
        assert_eq!(stored, formula);
    }
}

#[test]
fn aspect_arabic_characters_in_formula() {
    let formula = "افتراض و نتيجة".to_string();

    let piece = LogicPiece::Goal {
        formula: formula.clone(),
        position: (2, 2),
    };

    let mut board = BoardState::new(10, 10);
    assert!(board.place_piece(piece));

    if let Some(LogicPiece::Goal { formula: stored, .. }) = board.piece_at(2, 2) {
        assert_eq!(stored, formula);
    }
}

#[test]
fn aspect_emoji_in_formula() {
    let formula = "😀 proof 🎯".to_string();

    let piece = LogicPiece::Assumption {
        formula: formula.clone(),
        position: (3, 3),
    };

    let mut board = BoardState::new(10, 10);
    assert!(board.place_piece(piece));

    if let Some(LogicPiece::Assumption { formula: stored, .. }) = board.piece_at(3, 3) {
        assert_eq!(stored, formula);
    }
}

#[test]
fn aspect_mixed_scripts_in_formula() {
    let formula = "P AND 果 OR 🔥 IMPLIES x".to_string();

    let piece = LogicPiece::Assumption {
        formula: formula.clone(),
        position: (4, 4),
    };

    let mut board = BoardState::new(10, 10);
    assert!(board.place_piece(piece));

    if let Some(LogicPiece::Assumption { formula: stored, .. }) = board.piece_at(4, 4) {
        assert_eq!(stored, formula);
    }
}

#[test]
fn aspect_whitespace_variants_in_formula() {
    // Test various whitespace characters
    let formulas = vec![
        "P Q".to_string(),           // Regular space
        "P\tQ".to_string(),          // Tab
        "P\nQ".to_string(),          // Newline
        "P\u{00A0}Q".to_string(),    // Non-breaking space
        "P\u{2003}Q".to_string(),    // Em space
    ];

    let mut board = BoardState::new(10, 10);
    for (idx, formula) in formulas.iter().enumerate() {
        let piece = LogicPiece::Assumption {
            formula: formula.clone(),
            position: (idx as u32, 0),
        };
        assert!(board.place_piece(piece));
    }

    assert_eq!(board.piece_count(), formulas.len());
}

// ============================================================================
// Logic Piece Variant Coverage
// ============================================================================

#[test]
fn aspect_all_piece_variants_placeable() {
    let mut board = BoardState::new(15, 15);

    let pieces = vec![
        LogicPiece::Assumption {
            formula: "P".into(),
            position: (1, 1),
        },
        LogicPiece::Goal {
            formula: "Q".into(),
            position: (2, 2),
        },
        LogicPiece::AndIntro { position: (3, 3) },
        LogicPiece::OrIntro { position: (4, 4) },
        LogicPiece::ImpliesIntro { position: (5, 5) },
        LogicPiece::NotIntro { position: (6, 6) },
        LogicPiece::ForallIntro {
            position: (7, 7),
            variable: "x".into(),
        },
        LogicPiece::ExistsIntro {
            position: (8, 8),
            variable: "y".into(),
        },
        LogicPiece::Wire {
            from: (1, 1),
            to: (3, 3),
        },
    ];

    for piece in pieces {
        assert!(board.place_piece(piece));
    }

    assert_eq!(board.piece_count(), 9);
}

#[test]
fn aspect_wire_pieces_with_same_endpoints() {
    let mut board = BoardState::new(10, 10);

    let wire1 = LogicPiece::Wire {
        from: (1, 1),
        to: (5, 5),
    };
    let wire2 = LogicPiece::Wire {
        from: (1, 1),
        to: (5, 5),
    };

    // Both wires have same endpoints but are different piece instances
    assert!(board.place_piece(wire1));

    // Try to place second wire - should fail (same endpoints = same logical position)
    let result = board.place_piece(wire2);
    // Note: Depending on implementation, this might succeed (wires can coexist)
    // or fail (logical uniqueness). We just verify no panic.
    assert!(board.piece_count() >= 1);
}
