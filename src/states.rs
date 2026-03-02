// SPDX-License-Identifier: PMPL-1.0-or-later
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
