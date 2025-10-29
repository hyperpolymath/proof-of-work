use crate::game::Level;

pub mod z3_integration;
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
            proof_smt2: format!("(assert true)"), // TODO: real proof
            proof_isabelle: None,
            solution_steps: vec![],
            time_taken_secs: solution_time,
        }
    }
}

pub fn verify_level_solution(_level: &Level) -> bool {
    // TODO: actual verification
    true
}
