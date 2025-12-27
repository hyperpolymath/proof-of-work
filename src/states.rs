// SPDX-License-Identifier: MIT OR Apache-2.0
//! Game states for the application.

use bevy::prelude::*;

/// Game states
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    LevelSelect,
    Playing,
    LevelComplete,
    Editor,
    Settings,
    Leaderboard,
}
