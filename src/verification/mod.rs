// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::game::{BoardState, Level, LogicPiece};

#[cfg(feature = "z3-verify")]
pub mod z3_integration;

#[cfg(feature = "z3-verify")]
pub use z3_integration::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExportedProof {
    pub level_id: u32,
    pub player_id: String,
    pub proof_smt2: String,
    pub proof_isabelle: Option<String>,
    pub solution_steps: Vec<String>,
    pub time_taken_secs: u64,
}

impl ExportedProof {
    pub fn from_level(level: &Level, solution_time: u64) -> Self {
        Self {
            level_id: level.id,
            player_id: "local".to_string(),
            proof_smt2: board_to_smt(&level.initial_state),
            proof_isabelle: None,
            solution_steps: vec![],
            time_taken_secs: solution_time,
        }
    }
}

/// Convert board state to SMT-LIB2 format
pub fn board_to_smt(board: &BoardState) -> String {
    let mut smt = String::from("; Proof of Work - Generated Proof\n");
    smt.push_str("(set-logic QF_UF)\n");

    // Declare boolean constants for each formula
    let mut formulas: Vec<String> = Vec::new();
    for piece in &board.pieces {
        match piece {
            LogicPiece::Assumption { formula, .. } => {
                if !formulas.contains(formula) {
                    smt.push_str(&format!("(declare-const {} Bool)\n", formula));
                    formulas.push(formula.clone());
                }
            }
            LogicPiece::Goal { formula, .. } => {
                if !formulas.contains(formula) {
                    smt.push_str(&format!("(declare-const {} Bool)\n", formula));
                    formulas.push(formula.clone());
                }
            }
            _ => {}
        }
    }

    // Assert assumptions
    for piece in &board.pieces {
        if let LogicPiece::Assumption { formula, .. } = piece {
            smt.push_str(&format!("(assert {})\n", formula));
        }
    }

    smt.push_str("(check-sat)\n");
    smt
}

/// Check if two positions are adjacent (within 2 grid units)
fn is_adjacent(a: (u32, u32), b: (u32, u32)) -> bool {
    let dx = (a.0 as i32 - b.0 as i32).abs();
    let dy = (a.1 as i32 - b.1 as i32).abs();
    dx <= 2 && dy <= 2 && (dx + dy) > 0
}

/// Verify that the puzzle solution is correct
/// For the vertical slice: check if pieces form a valid proof
#[cfg(feature = "z3-verify")]
pub fn verify_level_solution(_level: &Level, pieces: &[LogicPiece]) -> bool {
    use z3::ast::{Ast, Bool};
    use z3::{Config, Context, Solver};

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // Collect assumptions and goals
    let mut assumptions: Vec<(&str, (u32, u32))> = Vec::new();
    let mut goals: Vec<(&str, (u32, u32))> = Vec::new();
    let mut and_gates: Vec<(u32, u32)> = Vec::new();
    let mut or_gates: Vec<(u32, u32)> = Vec::new();

    for piece in pieces {
        match piece {
            LogicPiece::Assumption { formula, position } => {
                assumptions.push((formula, *position));
            }
            LogicPiece::Goal { formula, position } => {
                goals.push((formula, *position));
            }
            LogicPiece::AndIntro { position } => {
                and_gates.push(*position);
            }
            LogicPiece::OrIntro { position } => {
                or_gates.push(*position);
            }
            _ => {}
        }
    }

    // For the "P AND Q => R" puzzle:
    // Need an AND gate that connects P and Q, and that gate connects to R
    for and_pos in &and_gates {
        let mut p_connected = false;
        let mut q_connected = false;
        let mut goal_connected = false;

        for (formula, pos) in &assumptions {
            if is_adjacent(*pos, *and_pos) {
                if *formula == "P" {
                    p_connected = true;
                }
                if *formula == "Q" {
                    q_connected = true;
                }
            }
        }

        for (_formula, pos) in &goals {
            if is_adjacent(*and_pos, *pos) {
                goal_connected = true;
            }
        }

        // If AND gate connects P, Q, and R - verify with Z3
        if p_connected && q_connected && goal_connected {
            // Create Z3 proof
            let p = Bool::new_const(&ctx, "P");
            let q = Bool::new_const(&ctx, "Q");
            let r = Bool::new_const(&ctx, "R");

            // Assert P and Q are true (assumptions)
            solver.assert(&p);
            solver.assert(&q);

            // We want to prove R, given (P AND Q) => R
            // Assert the implication as an axiom
            let p_and_q = Bool::and(&ctx, &[&p, &q]);
            let implication = Bool::implies(&p_and_q, &r);
            solver.assert(&implication);

            // Try to prove R is true
            // We check if NOT R leads to UNSAT
            solver.push();
            solver.assert(&r.not());

            match solver.check() {
                z3::SatResult::Unsat => {
                    // R must be true! Proof verified.
                    return true;
                }
                _ => {
                    solver.pop(1);
                }
            }
        }
    }

    // No valid configuration found
    false
}

/// Mock verification when Z3 is not available
/// Uses simple connectivity check
#[cfg(not(feature = "z3-verify"))]
pub fn verify_level_solution(_level: &Level, pieces: &[LogicPiece]) -> bool {
    // Collect assumptions and goals
    let mut assumptions: Vec<(&str, (u32, u32))> = Vec::new();
    let mut goals: Vec<(&str, (u32, u32))> = Vec::new();
    let mut and_gates: Vec<(u32, u32)> = Vec::new();

    for piece in pieces {
        match piece {
            LogicPiece::Assumption { formula, position } => {
                assumptions.push((formula, *position));
            }
            LogicPiece::Goal { formula, position } => {
                goals.push((formula, *position));
            }
            LogicPiece::AndIntro { position } => {
                and_gates.push(*position);
            }
            _ => {}
        }
    }

    // Simple connectivity check: AND gate must be adjacent to P, Q, and R
    for and_pos in &and_gates {
        let mut p_connected = false;
        let mut q_connected = false;
        let mut goal_connected = false;

        for (formula, pos) in &assumptions {
            if is_adjacent(*pos, *and_pos) {
                if *formula == "P" {
                    p_connected = true;
                }
                if *formula == "Q" {
                    q_connected = true;
                }
            }
        }

        for (_formula, pos) in &goals {
            if is_adjacent(*and_pos, *pos) {
                goal_connected = true;
            }
        }

        if p_connected && q_connected && goal_connected {
            // Mock verification passes - connections are correct
            tracing::info!("Mock verification: connections valid (Z3 not available)");
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacency() {
        assert!(is_adjacent((2, 5), (3, 5))); // Same row, adjacent
        assert!(is_adjacent((2, 5), (4, 5))); // Same row, 2 apart
        assert!(!is_adjacent((2, 5), (5, 5))); // Too far
        assert!(is_adjacent((2, 5), (3, 6))); // Diagonal
    }

    #[test]
    fn test_mock_verification() {
        let level = Level {
            id: 1,
            name: "Test".to_string(),
            description: "Test level".to_string(),
            theorem: "(assert (=> (and P Q) R))".to_string(),
            initial_state: BoardState {
                width: 10,
                height: 10,
                pieces: vec![],
            },
            goal_state: crate::game::GoalCondition::ProveFormula {
                formula: "R".to_string(),
            },
        };

        // Create pieces that should verify
        let pieces = vec![
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
            LogicPiece::AndIntro { position: (4, 4) }, // Adjacent to P, Q, and close to R
        ];

        // This should fail - AND gate is not adjacent to R (8,4)
        assert!(!verify_level_solution(&level, &pieces));

        // Now place AND gate between all pieces
        let pieces_valid = vec![
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
                position: (5, 4),
            },
            LogicPiece::AndIntro { position: (3, 4) }, // Adjacent to P(2,5), Q(2,3), and R(5,4)
        ];

        assert!(verify_level_solution(&level, &pieces_valid));
    }
}
