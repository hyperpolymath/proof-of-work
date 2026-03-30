// SPDX-License-Identifier: PMPL-1.0-or-later
//! Benchmarks for proof-of-work core game logic.
//!
//! Exercises real operations: board manipulation, piece placement and removal,
//! spatial queries, board validation, level verification, and SMT generation.

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use proof_of_work::game::validation;
use proof_of_work::verification;
use proof_of_work::{BoardState, GoalCondition, Level, LogicPiece};

// ---------------------------------------------------------------------------
// Helpers: build test fixtures once, clone per iteration
// ---------------------------------------------------------------------------

/// Create a populated board with assumptions, gates, goals, and wires.
fn populated_board() -> BoardState {
    let pieces = vec![
        LogicPiece::Assumption { formula: "P".into(), position: (2, 5) },
        LogicPiece::Assumption { formula: "Q".into(), position: (2, 3) },
        LogicPiece::Assumption { formula: "R".into(), position: (1, 7) },
        LogicPiece::Goal { formula: "S".into(), position: (15, 5) },
        LogicPiece::Goal { formula: "T".into(), position: (15, 10) },
        LogicPiece::AndIntro { position: (5, 4) },
        LogicPiece::OrIntro { position: (8, 6) },
        LogicPiece::ImpliesIntro { position: (10, 4) },
        LogicPiece::NotIntro { position: (12, 8) },
        LogicPiece::Wire { from: (3, 5), to: (5, 4) },
        LogicPiece::Wire { from: (3, 3), to: (5, 4) },
        LogicPiece::Wire { from: (6, 4), to: (8, 6) },
        LogicPiece::Wire { from: (9, 6), to: (10, 4) },
        LogicPiece::Wire { from: (11, 4), to: (15, 5) },
    ];
    BoardState::with_pieces(20, 20, pieces)
}

/// Create a level suitable for verification benchmarking.
fn verifiable_level() -> Level {
    Level {
        id: 1,
        name: "Bench Level".into(),
        description: "P AND Q implies R".into(),
        theorem: "(assert (=> (and P Q) R))".into(),
        initial_state: BoardState::new(10, 10),
        goal_state: GoalCondition::ProveFormula { formula: "R".into() },
    }
}

/// Pieces that form a valid proof for the verifiable level (AND gate adjacent to
/// both assumptions and the goal).
fn valid_proof_pieces() -> Vec<LogicPiece> {
    vec![
        LogicPiece::Assumption { formula: "P".into(), position: (2, 5) },
        LogicPiece::Assumption { formula: "Q".into(), position: (2, 3) },
        LogicPiece::Goal { formula: "R".into(), position: (5, 4) },
        LogicPiece::AndIntro { position: (3, 4) },
    ]
}

// ---------------------------------------------------------------------------
// Board operation benchmarks
// ---------------------------------------------------------------------------

/// Benchmark placing 100 pieces onto a board sequentially.
fn bench_board_place_pieces(c: &mut Criterion) {
    c.bench_function("board_place_100_pieces", |b| {
        b.iter(|| {
            let mut board = BoardState::new(20, 20);
            for i in 0..10u32 {
                for j in 0..10u32 {
                    board.place_piece(LogicPiece::AndIntro { position: (i, j) });
                }
            }
            black_box(board.piece_count())
        });
    });
}

/// Benchmark removing all pieces from a populated board.
fn bench_board_remove_pieces(c: &mut Criterion) {
    let template = populated_board();

    c.bench_function("board_remove_all_pieces", |b| {
        b.iter(|| {
            let mut board = template.clone();
            let positions: Vec<(u32, u32)> = board.pieces.iter().map(|p| p.position()).collect();
            for (x, y) in &positions {
                board.remove_piece(*x, *y);
            }
            black_box(board.piece_count())
        });
    });
}

/// Benchmark the spatial query (pieces_near) on a populated board.
fn bench_board_pieces_near(c: &mut Criterion) {
    let board = populated_board();

    c.bench_function("board_pieces_near_radius_3", |b| {
        b.iter(|| {
            let near = board.pieces_near(black_box(8), black_box(5), black_box(3));
            black_box(near.len())
        });
    });
}

/// Benchmark filtering board into assumptions, goals, gates, and wires.
fn bench_board_filters(c: &mut Criterion) {
    let board = populated_board();

    c.bench_function("board_filter_all_categories", |b| {
        b.iter(|| {
            let a = board.assumptions().len();
            let g = board.goals().len();
            let gates = board.gates().len();
            let w = board.wires().len();
            black_box(a + g + gates + w)
        });
    });
}

/// Benchmark piece movement on a board.
fn bench_board_move_piece(c: &mut Criterion) {
    c.bench_function("board_move_piece_round_trip", |b| {
        b.iter(|| {
            let mut board = BoardState::new(20, 20);
            board.place_piece(LogicPiece::AndIntro { position: (5, 5) });
            board.move_piece((5, 5), (10, 10));
            board.move_piece((10, 10), (15, 15));
            board.move_piece((15, 15), (0, 0));
            black_box(board.piece_at(0, 0).is_some())
        });
    });
}

// ---------------------------------------------------------------------------
// Validation benchmarks
// ---------------------------------------------------------------------------

/// Benchmark validating a well-formed populated board.
fn bench_validate_board(c: &mut Criterion) {
    let board = populated_board();

    c.bench_function("validate_board_14_pieces", |b| {
        b.iter(|| {
            let result = validation::validate_board(black_box(&board));
            black_box(result.is_valid)
        });
    });
}

/// Benchmark validating individual piece placements.
fn bench_validate_piece_placement(c: &mut Criterion) {
    let board = populated_board();
    let pieces = [
        LogicPiece::Assumption { formula: "X".into(), position: (0, 0) },
        LogicPiece::Wire { from: (3, 3), to: (7, 7) },
        LogicPiece::AndIntro { position: (99, 99) }, // out of bounds
        LogicPiece::Goal { formula: "".into(), position: (4, 4) }, // empty formula
    ];

    c.bench_function("validate_piece_placement_4_pieces", |b| {
        b.iter(|| {
            let mut ok = 0usize;
            for piece in &pieces {
                if validation::validate_piece_placement(&board, piece).is_ok() {
                    ok += 1;
                }
            }
            black_box(ok)
        });
    });
}

/// Benchmark the readiness-for-verification check.
fn bench_is_ready_for_verification(c: &mut Criterion) {
    let board = populated_board();

    c.bench_function("is_ready_for_verification", |b| {
        b.iter(|| {
            black_box(validation::is_ready_for_verification(&board))
        });
    });
}

/// Benchmark validating a complete level definition.
fn bench_validate_level(c: &mut Criterion) {
    let level = Level {
        id: 1,
        name: "Bench Level".into(),
        description: "Test level for benchmarking".into(),
        theorem: "(assert (=> (and P Q) R))".into(),
        initial_state: populated_board(),
        goal_state: GoalCondition::ConnectNodes {
            start: (2, 5),
            end: (15, 5),
        },
    };

    c.bench_function("validate_level_definition", |b| {
        b.iter(|| {
            let result = validation::validate_level(black_box(&level));
            black_box(result.is_valid)
        });
    });
}

// ---------------------------------------------------------------------------
// Verification / SMT benchmarks
// ---------------------------------------------------------------------------

/// Benchmark converting a board state to SMT-LIB2 format.
fn bench_board_to_smt(c: &mut Criterion) {
    let board = populated_board();

    c.bench_function("board_to_smt_14_pieces", |b| {
        b.iter(|| {
            let smt = verification::board_to_smt(black_box(&board));
            black_box(smt.len())
        });
    });
}

/// Benchmark creating an ExportedProof from a level.
fn bench_exported_proof_from_level(c: &mut Criterion) {
    let level = verifiable_level();

    c.bench_function("exported_proof_from_level", |b| {
        b.iter(|| {
            let proof = verification::ExportedProof::from_level(black_box(&level), 42);
            black_box(proof.proof_smt2.len())
        });
    });
}

/// Benchmark solution verification (mock / non-Z3 path).
fn bench_verify_level_solution(c: &mut Criterion) {
    let level = verifiable_level();
    let pieces = valid_proof_pieces();

    c.bench_function("verify_level_solution_valid", |b| {
        b.iter(|| {
            black_box(verification::verify_level_solution(
                black_box(&level),
                black_box(&pieces),
            ))
        });
    });
}

/// Benchmark solution verification with an invalid arrangement.
fn bench_verify_level_solution_invalid(c: &mut Criterion) {
    let level = verifiable_level();
    let pieces = vec![
        LogicPiece::Assumption { formula: "P".into(), position: (0, 0) },
        LogicPiece::Assumption { formula: "Q".into(), position: (0, 19) },
        LogicPiece::Goal { formula: "R".into(), position: (19, 19) },
        LogicPiece::AndIntro { position: (10, 10) }, // too far from everything
    ];

    c.bench_function("verify_level_solution_invalid", |b| {
        b.iter(|| {
            black_box(verification::verify_level_solution(
                black_box(&level),
                black_box(&pieces),
            ))
        });
    });
}

/// Benchmark serialization of a LogicPiece to SMT format.
fn bench_piece_to_smt(c: &mut Criterion) {
    let pieces = [
        LogicPiece::Assumption { formula: "P".into(), position: (2, 5) },
        LogicPiece::Goal { formula: "Q".into(), position: (8, 4) },
        LogicPiece::AndIntro { position: (5, 5) },
        LogicPiece::ImpliesIntro { position: (3, 3) },
        LogicPiece::ForallIntro { position: (1, 1), variable: "x".into() },
    ];

    c.bench_function("piece_to_smt_5_pieces", |b| {
        b.iter(|| {
            let mut total_len = 0usize;
            for piece in &pieces {
                total_len += piece.to_smt().len();
            }
            black_box(total_len)
        });
    });
}

// ---------------------------------------------------------------------------
// Criterion groups
// ---------------------------------------------------------------------------

criterion_group!(
    board_benches,
    bench_board_place_pieces,
    bench_board_remove_pieces,
    bench_board_pieces_near,
    bench_board_filters,
    bench_board_move_piece,
);

criterion_group!(
    validation_benches,
    bench_validate_board,
    bench_validate_piece_placement,
    bench_is_ready_for_verification,
    bench_validate_level,
);

criterion_group!(
    verification_benches,
    bench_board_to_smt,
    bench_exported_proof_from_level,
    bench_verify_level_solution,
    bench_verify_level_solution_invalid,
    bench_piece_to_smt,
);

criterion_main!(board_benches, validation_benches, verification_benches);
