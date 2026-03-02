// SPDX-License-Identifier: PMPL-1.0-or-later
//! Level pack management - loading, saving, and organizing levels.

pub mod ui;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::game::{BoardState, GoalCondition, Level, LogicPiece};

/// A collection of levels bundled together
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelPack {
    /// Unique identifier for the pack
    pub id: String,
    /// Display name
    pub name: String,
    /// Pack author
    pub author: String,
    /// Description of the pack
    pub description: String,
    /// Version string (semantic versioning recommended)
    pub version: String,
    /// Difficulty rating (1-5)
    pub difficulty: u8,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// The levels in this pack
    pub levels: Vec<Level>,
}

impl Default for LevelPack {
    fn default() -> Self {
        Self {
            id: "untitled".to_string(),
            name: "Untitled Pack".to_string(),
            author: "Unknown".to_string(),
            description: "A new level pack".to_string(),
            version: "1.0.0".to_string(),
            difficulty: 1,
            tags: vec![],
            levels: vec![],
        }
    }
}

#[allow(dead_code)]
impl LevelPack {
    /// Create a new empty level pack
    pub fn new(id: &str, name: &str, author: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            author: author.to_string(),
            ..Default::default()
        }
    }

    /// Add a level to the pack
    pub fn add_level(&mut self, level: Level) {
        self.levels.push(level);
    }

    /// Get the number of levels
    pub fn level_count(&self) -> usize {
        self.levels.len()
    }

    /// Save the pack to a file
    pub fn save(&self, path: &Path) -> Result<(), LevelPackError> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| LevelPackError::SerializationError(e.to_string()))?;
        fs::write(path, json).map_err(|e| LevelPackError::IoError(e.to_string()))?;
        Ok(())
    }

    /// Load a pack from a file
    pub fn load(path: &Path) -> Result<Self, LevelPackError> {
        let content =
            fs::read_to_string(path).map_err(|e| LevelPackError::IoError(e.to_string()))?;
        serde_json::from_str(&content)
            .map_err(|e| LevelPackError::DeserializationError(e.to_string()))
    }
}

/// Errors that can occur when working with level packs
#[allow(dead_code)]
#[derive(Debug)]
pub enum LevelPackError {
    IoError(String),
    SerializationError(String),
    DeserializationError(String),
    NotFound(String),
}

impl std::fmt::Display for LevelPackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
            Self::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            Self::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for LevelPackError {}

/// Progress tracking for a level pack
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackProgress {
    /// Levels completed (by level id)
    pub completed: HashMap<u32, LevelCompletion>,
}

/// Completion data for a single level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelCompletion {
    /// Best time in seconds
    pub best_time_secs: u64,
    /// Number of times completed
    pub times_completed: u32,
}

/// Manager for loading and tracking level packs
#[derive(Debug, Default, bevy::prelude::Resource)]
pub struct LevelPackManager {
    /// Directory where level packs are stored
    pub packs_dir: PathBuf,
    /// Loaded level packs
    pub packs: Vec<LevelPack>,
    /// Progress for each pack (by pack id)
    pub progress: HashMap<String, PackProgress>,
    /// Currently selected pack index
    pub current_pack_index: Option<usize>,
    /// Currently selected level index within the pack
    pub current_level_index: Option<usize>,
}

#[allow(dead_code)]
impl LevelPackManager {
    /// Create a new manager with the specified packs directory
    pub fn new(packs_dir: PathBuf) -> Self {
        Self {
            packs_dir,
            packs: vec![],
            progress: HashMap::new(),
            current_pack_index: None,
            current_level_index: None,
        }
    }

    /// Load all level packs from the packs directory
    pub fn load_all(&mut self) -> Result<(), LevelPackError> {
        self.packs.clear();

        // Create directory if it doesn't exist
        if !self.packs_dir.exists() {
            fs::create_dir_all(&self.packs_dir)
                .map_err(|e| LevelPackError::IoError(e.to_string()))?;
        }

        // Add built-in tutorial pack
        self.packs.push(create_builtin_tutorial_pack());

        // Load packs from directory
        if let Ok(entries) = fs::read_dir(&self.packs_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "json").unwrap_or(false) {
                    match LevelPack::load(&path) {
                        Ok(pack) => {
                            self.packs.push(pack);
                        }
                        Err(e) => {
                            eprintln!("Failed to load pack {:?}: {}", path, e);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Get the current level based on selected indices
    pub fn current_level(&self) -> Option<&Level> {
        let pack_idx = self.current_pack_index?;
        let level_idx = self.current_level_index?;
        self.packs.get(pack_idx)?.levels.get(level_idx)
    }

    /// Select a pack by index
    pub fn select_pack(&mut self, index: usize) {
        if index < self.packs.len() {
            self.current_pack_index = Some(index);
            self.current_level_index = Some(0);
        }
    }

    /// Select a level by index within the current pack
    pub fn select_level(&mut self, index: usize) {
        if let Some(pack_idx) = self.current_pack_index {
            if let Some(pack) = self.packs.get(pack_idx) {
                if index < pack.levels.len() {
                    self.current_level_index = Some(index);
                }
            }
        }
    }

    /// Advance to the next level, returns true if successful
    pub fn next_level(&mut self) -> bool {
        if let (Some(pack_idx), Some(level_idx)) =
            (self.current_pack_index, self.current_level_index)
        {
            if let Some(pack) = self.packs.get(pack_idx) {
                if level_idx + 1 < pack.levels.len() {
                    self.current_level_index = Some(level_idx + 1);
                    return true;
                }
            }
        }
        false
    }

    /// Mark current level as completed
    pub fn mark_completed(&mut self, time_secs: u64) {
        if let (Some(pack_idx), Some(level_idx)) =
            (self.current_pack_index, self.current_level_index)
        {
            if let Some(pack) = self.packs.get(pack_idx) {
                if let Some(level) = pack.levels.get(level_idx) {
                    let pack_progress = self.progress.entry(pack.id.clone()).or_default();
                    let completion =
                        pack_progress
                            .completed
                            .entry(level.id)
                            .or_insert(LevelCompletion {
                                best_time_secs: u64::MAX,
                                times_completed: 0,
                            });

                    completion.times_completed += 1;
                    if time_secs < completion.best_time_secs {
                        completion.best_time_secs = time_secs;
                    }
                }
            }
        }
    }

    /// Check if a level is completed
    pub fn is_level_completed(&self, pack_id: &str, level_id: u32) -> bool {
        self.progress
            .get(pack_id)
            .map(|p| p.completed.contains_key(&level_id))
            .unwrap_or(false)
    }

    /// Save a user-created pack
    pub fn save_pack(&self, pack: &LevelPack) -> Result<PathBuf, LevelPackError> {
        let filename = format!("{}.json", pack.id);
        let path = self.packs_dir.join(filename);
        pack.save(&path)?;
        Ok(path)
    }

    /// Save progress to disk
    pub fn save_progress(&self, path: &Path) -> Result<(), LevelPackError> {
        let json = serde_json::to_string_pretty(&self.progress)
            .map_err(|e| LevelPackError::SerializationError(e.to_string()))?;
        fs::write(path, json).map_err(|e| LevelPackError::IoError(e.to_string()))?;
        Ok(())
    }

    /// Load progress from disk
    pub fn load_progress(&mut self, path: &Path) -> Result<(), LevelPackError> {
        if path.exists() {
            let content =
                fs::read_to_string(path).map_err(|e| LevelPackError::IoError(e.to_string()))?;
            self.progress = serde_json::from_str(&content)
                .map_err(|e| LevelPackError::DeserializationError(e.to_string()))?;
        }
        Ok(())
    }
}

/// Create the built-in tutorial level pack
pub fn create_builtin_tutorial_pack() -> LevelPack {
    LevelPack {
        id: "tutorial".to_string(),
        name: "Tutorial".to_string(),
        author: "Proof of Work Team".to_string(),
        description: "Learn the basics of logical proofs".to_string(),
        version: "1.0.0".to_string(),
        difficulty: 1,
        tags: vec!["tutorial".to_string(), "beginner".to_string()],
        levels: vec![
            Level {
                id: 1,
                name: "First Steps".to_string(),
                description: "Place an AND gate to connect P and Q, then connect to R".to_string(),
                theorem: "(assert (=> (and P Q) R))".to_string(),
                initial_state: BoardState {
                    width: 10,
                    height: 10,
                    pieces: vec![
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
                    ],
                },
                goal_state: GoalCondition::ProveFormula {
                    formula: "R".to_string(),
                },
            },
            Level {
                id: 2,
                name: "Either Way".to_string(),
                description: "Use OR introduction to prove A ∨ B from A".to_string(),
                theorem: "(assert (=> A (or A B)))".to_string(),
                initial_state: BoardState {
                    width: 10,
                    height: 10,
                    pieces: vec![
                        LogicPiece::Assumption {
                            formula: "A".to_string(),
                            position: (2, 5),
                        },
                        LogicPiece::Goal {
                            formula: "A ∨ B".to_string(),
                            position: (8, 5),
                        },
                    ],
                },
                goal_state: GoalCondition::ProveFormula {
                    formula: "(or A B)".to_string(),
                },
            },
            Level {
                id: 3,
                name: "Conjunction Junction".to_string(),
                description: "Combine X, Y, and Z using multiple AND gates".to_string(),
                theorem: "(assert (=> (and (and X Y) Z) Result))".to_string(),
                initial_state: BoardState {
                    width: 10,
                    height: 10,
                    pieces: vec![
                        LogicPiece::Assumption {
                            formula: "X".to_string(),
                            position: (1, 7),
                        },
                        LogicPiece::Assumption {
                            formula: "Y".to_string(),
                            position: (1, 5),
                        },
                        LogicPiece::Assumption {
                            formula: "Z".to_string(),
                            position: (1, 3),
                        },
                        LogicPiece::Goal {
                            formula: "Result".to_string(),
                            position: (9, 5),
                        },
                    ],
                },
                goal_state: GoalCondition::ProveFormula {
                    formula: "Result".to_string(),
                },
            },
            Level {
                id: 4,
                name: "Chain of Logic".to_string(),
                description: "Build a chain: A → (A ∧ B) → Goal".to_string(),
                theorem: "(assert (=> (and A B) Goal))".to_string(),
                initial_state: BoardState {
                    width: 10,
                    height: 10,
                    pieces: vec![
                        LogicPiece::Assumption {
                            formula: "A".to_string(),
                            position: (1, 6),
                        },
                        LogicPiece::Assumption {
                            formula: "B".to_string(),
                            position: (1, 4),
                        },
                        LogicPiece::Goal {
                            formula: "Goal".to_string(),
                            position: (9, 5),
                        },
                    ],
                },
                goal_state: GoalCondition::ProveFormula {
                    formula: "Goal".to_string(),
                },
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_pack_creation() {
        let pack = LevelPack::new("test", "Test Pack", "Test Author");
        assert_eq!(pack.id, "test");
        assert_eq!(pack.name, "Test Pack");
        assert_eq!(pack.author, "Test Author");
        assert_eq!(pack.level_count(), 0);
    }

    #[test]
    fn test_add_level() {
        let mut pack = LevelPack::new("test", "Test Pack", "Test Author");
        pack.add_level(Level {
            id: 1,
            name: "Test Level".to_string(),
            description: "Test".to_string(),
            theorem: "".to_string(),
            initial_state: BoardState {
                width: 10,
                height: 10,
                pieces: vec![],
            },
            goal_state: GoalCondition::ProveFormula {
                formula: "X".to_string(),
            },
        });
        assert_eq!(pack.level_count(), 1);
    }

    #[test]
    fn test_builtin_pack() {
        let pack = create_builtin_tutorial_pack();
        assert_eq!(pack.id, "tutorial");
        assert!(!pack.levels.is_empty());
    }
}
