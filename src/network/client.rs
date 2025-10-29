use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::verification::ExportedProof;
use super::{ProofSubmission, ServerResponse, LeaderboardEntry};

const SERVER_URL: &str = "https://api.proofofwork.game";

#[derive(Clone)]
pub struct NetworkClient {
    client: Client,
    api_key: String,
}

impl NetworkClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client"),
            api_key,
        }
    }

    pub async fn submit_proof(&self, proof: ExportedProof) -> Result<ServerResponse, Box<dyn std::error::Error>> {
        let submission = ProofSubmission {
            proof: proof.clone(),
            signature: Self::sign_proof(&proof, &self.api_key),
        };

        let response = self.client
        .post(&format!("{}/api/v1/proofs", SERVER_URL))
        .header("Authorization", format!("Bearer {}", self.api_key))
        .json(&submission)
        .send()
        .await?;

        if !response.status().is_success() {
            return Err(format!("Server returned error: {}", response.status()).into());
        }

        let server_response = response.json::<ServerResponse>().await?;
        Ok(server_response)
    }

    pub async fn get_leaderboard(&self, limit: Option<u32>) -> Result<Vec<LeaderboardEntry>, Box<dyn std::error::Error>> {
        let limit = limit.unwrap_or(100);

        let response = self.client
        .get(&format!("{}/api/v1/leaderboard", SERVER_URL))
        .query(&[("limit", limit)])
        .send()
        .await?;

        if !response.status().is_success() {
            return Err(format!("Server returned error: {}", response.status()).into());
        }

        let leaderboard = response.json::<Vec<LeaderboardEntry>>().await?;
        Ok(leaderboard)
    }

    pub async fn get_player_stats(&self) -> Result<PlayerStatsResponse, Box<dyn std::error::Error>> {
        let response = self.client
        .get(&format!("{}/api/v1/player/stats", SERVER_URL))
        .header("Authorization", format!("Bearer {}", self.api_key))
        .send()
        .await?;

        if !response.status().is_success() {
            return Err(format!("Server returned error: {}", response.status()).into());
        }

        let stats = response.json::<PlayerStatsResponse>().await?;
        Ok(stats)
    }

    fn sign_proof(proof: &ExportedProof, api_key: &str) -> String {
        use sha2::{Sha256, Digest};

        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(proof).unwrap());
        hasher.update(api_key.as_bytes());

        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStatsResponse {
    pub total_proofs: u32,
    pub total_points: u32,
    pub global_rank: u32,
    pub levels_completed: u32,
}
