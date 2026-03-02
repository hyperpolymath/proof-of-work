// SPDX-License-Identifier: PMPL-1.0-or-later
use steamworks::*;
use std::sync::Arc;

pub struct SteamManager {
    client: Arc<Client>,
    #[allow(dead_code)]
    single: SingleClient,
}

impl SteamManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (client, single) = Client::init()?;
        Ok(Self {
            client: Arc::new(client),
           single,
        })
    }

    pub fn get_username(&self) -> String {
        self.client.friends().name()
    }

    pub fn get_steam_id(&self) -> SteamId {
        self.client.user().steam_id()
    }

    pub fn unlock_achievement(&self, achievement: &str) {
        let stats = self.client.user_stats();
        stats.set_achievement(achievement);

        // Must call store_stats to persist
        if stats.store_stats() {
            println!("Achievement unlocked: {}", achievement);
        } else {
            eprintln!("Failed to store achievement: {}", achievement);
        }
    }

    pub fn update_stat(&self, stat_name: &str, value: i32) {
        let stats = self.client.user_stats();
        stats.set_stat_i32(stat_name, value);

        if stats.store_stats() {
            println!("Stat updated: {} = {}", stat_name, value);
        } else {
            eprintln!("Failed to store stat: {}", stat_name);
        }
    }

    pub fn get_stat(&self, stat_name: &str) -> Option<i32> {
        let stats = self.client.user_stats();
        stats.get_stat_i32(stat_name).ok()
    }

    pub fn is_achievement_unlocked(&self, achievement: &str) -> bool {
        let stats = self.client.user_stats();
        stats.achievement(achievement).ok().unwrap_or(false)
    }

    pub fn run_callbacks(&self) {
        self.single.run_callbacks();
    }

    pub fn request_stats(&self) {
        let stats = self.client.user_stats();
        stats.request_current_stats();
    }
}

// Achievement IDs (must match Steam Partner settings)
pub const ACHIEVEMENT_FIRST_PROOF: &str = "FIRST_PROOF";
pub const ACHIEVEMENT_10_PROOFS: &str = "TEN_PROOFS";
pub const ACHIEVEMENT_100_PROOFS: &str = "HUNDRED_PROOFS";
pub const ACHIEVEMENT_SPEEDRUN: &str = "SPEEDRUN";
pub const ACHIEVEMENT_PERFECT_LEVEL: &str = "PERFECT_LEVEL";

// Stat names (must match Steam Partner settings)
pub const STAT_PROOFS_COMPLETED: &str = "proofs_completed";
pub const STAT_TOTAL_TIME: &str = "total_time_seconds";
pub const STAT_LEVELS_COMPLETED: &str = "levels_completed";
