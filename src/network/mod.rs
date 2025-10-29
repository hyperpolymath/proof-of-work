pub mod client;

pub use client::NetworkClient;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofSubmission {
    pub proof: crate::verification::ExportedProof,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerResponse {
    pub accepted: bool,
    pub points_awarded: u32,
    pub global_rank: Option<u32>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub player_name: String,
    pub steam_id: Option<String>,
    pub proofs_completed: u32,
    pub total_points: u32,
    pub rank: u32,
}
