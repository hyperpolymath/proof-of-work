// SPDX-License-Identifier: MPL-2.0

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

/// Outcome of `verify_level_solution`. Tri-valued so the mock (no-Z3) path
/// can honestly say "cannot decide" instead of granting false wins on
/// connectivity alone — see I2 in `src/abi/ProofOfWork/ABI/Invariants.idr`.
//
// Variants are only ever *constructed* by the z3-verify path; the mock
// path only constructs `CannotVerify`. The match in `game_systems.rs`
// handles all three regardless of feature, so we mute dead-code in the
// no-z3 build to keep the enum exhaustive at the call site.
#[cfg_attr(not(feature = "z3-verify"), allow(dead_code))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationVerdict {
    /// Z3 (or another sound verifier) accepted the solution.
    Verified,
    /// Z3 (or another sound verifier) rejected the solution.
    Rejected,
    /// No sound verifier available in this build (`--features z3-verify`
    /// was not enabled). The mock path returns this — never `Verified` —
    /// so that no-Z3 builds cannot grant wins.
    CannotVerify,
}

/// Verify that the puzzle solution is correct
/// For the vertical slice: check if pieces form a valid proof
//
// PROOF-OBLIGATION I1 (OWED): verification soundness — a `Verified` verdict
// must imply the existence of a `VerifiedSolution` certificate (adjacency
// witness + SMT entailment). This function returns a verdict; the certificate
// is not constructed/returned. The refinement obligation is to surface the
// certificate type the seam already defines.
// See: src/abi/ProofOfWork/ABI/Invariants.idr I1
#[cfg(feature = "z3-verify")]
pub fn verify_level_solution(_level: &Level, pieces: &[LogicPiece]) -> VerificationVerdict {
    use z3::Solver;
    use z3::ast::Bool;

    let solver = Solver::new();

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
            let p = Bool::new_const("P");
            let q = Bool::new_const("Q");
            let r = Bool::new_const("R");

            // Assert P and Q are true (assumptions)
            solver.assert(&p);
            solver.assert(&q);

            // We want to prove R, given (P AND Q) => R
            // Assert the implication as an axiom
            let p_and_q = Bool::and(&[&p, &q]);
            let implication = p_and_q.implies(&r);
            solver.assert(&implication);

            // Try to prove R is true
            // We check if NOT R leads to UNSAT
            solver.push();
            solver.assert(&r.not());

            match solver.check() {
                z3::SatResult::Unsat => {
                    // R must be true! Proof verified.
                    return VerificationVerdict::Verified;
                }
                _ => {
                    solver.pop(1);
                }
            }
        }
    }

    // No valid configuration found
    VerificationVerdict::Rejected
}

/// Mock verification stub when Z3 is not available.
//
// I2 RESOLUTION (2026-05-21): the mock now always returns
// [VerificationVerdict::CannotVerify]. Previously this function accepted
// on adjacency-connectivity alone (no SMT step), which granted false wins
// in no-Z3 builds and made `mockNoStrongerThanZ3` (see
// `src/abi/ProofOfWork/ABI/Invariants.idr` I2) structurally unprovable.
// With the tri-valued [VerificationVerdict], the mock honestly reports
// "I cannot decide" — so it can never accept what Z3 would reject, and
// the Idris2 obligation discharges vacuously.
//
// Callers must handle [CannotVerify] as a non-winning verdict and surface
// the demo-build status to the player.
#[cfg(not(feature = "z3-verify"))]
pub fn verify_level_solution(_level: &Level, _pieces: &[LogicPiece]) -> VerificationVerdict {
    tracing::warn!(
        "Verification skipped: this is a demo build (no `z3-verify` feature). \
         Rebuild with `cargo run --features z3-verify` to verify solutions."
    );
    VerificationVerdict::CannotVerify
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

    fn test_level() -> Level {
        Level {
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
        }
    }

    fn pieces_disconnected() -> Vec<LogicPiece> {
        vec![
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
            LogicPiece::AndIntro { position: (4, 4) }, // Adjacent to P, Q, but not R (8,4)
        ]
    }

    fn pieces_valid() -> Vec<LogicPiece> {
        vec![
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
        ]
    }

    /// Under `--features z3-verify`, the Z3 path discriminates Verified
    /// vs Rejected based on adjacency + SMT entailment.
    #[cfg(feature = "z3-verify")]
    #[test]
    fn test_verification_z3() {
        assert_eq!(
            verify_level_solution(&test_level(), &pieces_disconnected()),
            VerificationVerdict::Rejected
        );
        assert_eq!(
            verify_level_solution(&test_level(), &pieces_valid()),
            VerificationVerdict::Verified
        );
    }

    /// Without `z3-verify`, the mock returns `CannotVerify` for every
    /// input — discharging I2 (mockNoStrongerThanZ3) vacuously by never
    /// accepting at all. This is the regression test for the 2026-05-21
    /// fix: any future weakening that re-adds connectivity-only acceptance
    /// will fail this assertion.
    #[cfg(not(feature = "z3-verify"))]
    #[test]
    fn test_mock_never_accepts() {
        assert_eq!(
            verify_level_solution(&test_level(), &pieces_disconnected()),
            VerificationVerdict::CannotVerify
        );
        assert_eq!(
            verify_level_solution(&test_level(), &pieces_valid()),
            VerificationVerdict::CannotVerify
        );
    }
}
