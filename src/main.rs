// SPDX-License-Identifier: MIT OR Apache-2.0

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod game;
mod game_systems;
mod ui;
mod verification;

#[cfg(feature = "network")]
mod network;

#[cfg(feature = "steam")]
mod steam;

use game::{CurrentLevel, PlayerStats, SelectedPieceType};
use verification::ExportedProof;

#[cfg(feature = "steam")]
use steam::SteamManager;

fn main() {
    // Initialize Steam first (before Bevy) - only if feature enabled
    #[cfg(feature = "steam")]
    let steam_manager: Option<SteamManager> = match SteamManager::new() {
        Ok(steam) => {
            info!("Steam initialized successfully");
            info!("  Username: {}", steam.get_username());
            info!("  Steam ID: {:?}", steam.get_steam_id());
            Some(steam)
        }
        Err(e) => {
            warn!("Steam not available: {}", e);
            warn!("  Running in offline mode");
            None
        }
    };

    // Build and run the app
    let mut app = App::new();

    app
    // Core Bevy plugins
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Proof of Work - Logic Puzzle Game".into(),
            resolution: (1280, 720).into(),
            resizable: true,
            ..default()
        }),
        ..default()
    }))

    // Egui plugin for UI
    .add_plugins(EguiPlugin::default())

    // Initialize game state
    .init_state::<GameState>()

    // Player stats resource
    .insert_resource(PlayerStats::default())

    // Selected piece type resource
    .insert_resource(SelectedPieceType::default());

    // Insert Steam as a resource (if available)
    #[cfg(feature = "steam")]
    app.insert_resource(steam_manager);

    // Network client resource
    #[cfg(feature = "network")]
    {
        #[cfg(feature = "steam")]
        {
            let steam: Option<&SteamManager> = app.world().get_resource();
            if let Some(steam) = steam {
                let steam_id = steam.get_steam_id();
                let api_key = format!("steam_{}", steam_id.raw());
                app.insert_resource(network::NetworkClient::new(api_key));
            } else {
                app.insert_resource(network::NetworkClient::new("offline_mode".to_string()));
            }
        }
        #[cfg(not(feature = "steam"))]
        app.insert_resource(network::NetworkClient::new("offline_mode".to_string()));
    }

    app
    // Startup systems (run once at launch)
    .add_systems(Startup, setup_camera)

    // Systems that run every frame in MainMenu state
    .add_systems(Update, (
        ui::main_menu_system,
        ui::handle_menu_input,
    ).run_if(in_state(GameState::MainMenu)))

    // Systems when entering Playing state
    .add_systems(OnEnter(GameState::Playing), (
        game_systems::load_level,
        game_systems::spawn_pieces,
    ).chain())

    // Systems that run every frame in Playing state
    .add_systems(Update, (
        game_systems::handle_input,
        game_systems::update_board,
        game_systems::update_piece_positions,
        game_systems::check_connections,
        game_systems::check_solution,
        ui::update_hud,
    ).run_if(in_state(GameState::Playing)));

    // Steam callbacks (if available)
    #[cfg(feature = "steam")]
    app.add_systems(Update, steam_callbacks.run_if(in_state(GameState::Playing)));

    app
    // Systems when entering LevelComplete state
    .add_systems(OnEnter(GameState::LevelComplete), on_level_complete)

    // Systems that run in LevelComplete state
    .add_systems(Update, (
        ui::show_completion_screen,
        ui::handle_completion_input,
    ).run_if(in_state(GameState::LevelComplete)))

    // Systems when exiting Playing state
    .add_systems(OnExit(GameState::Playing), game_systems::cleanup_level)

    // Run the app
    .run();
}

// Game states
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    LevelComplete,
    Settings,
    Leaderboard,
}

// Startup systems
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Camera spawned");
}

// Steam callback system (must run every frame)
#[cfg(feature = "steam")]
fn steam_callbacks(steam: Option<Res<SteamManager>>) {
    if let Some(steam) = steam {
        steam.run_callbacks();
    }
}

// Level completion handler
fn on_level_complete(
    #[cfg(feature = "steam")] steam: Option<Res<SteamManager>>,
    mut stats: ResMut<PlayerStats>,
    level_query: Query<&CurrentLevel>,
    #[cfg(feature = "network")] network: Res<network::NetworkClient>,
) {
    let Ok(current_level) = level_query.single() else {
        error!("No current level found!");
        return;
    };

    // Update player stats
    stats.proofs_completed += 1;
    stats.levels_completed += 1;
    stats.total_playtime_secs += stats.last_level_time_secs;

    info!("========================================");
    info!("  LEVEL COMPLETE!");
    info!("  Level: {}", current_level.0.name);
    info!("  Time: {}s", stats.last_level_time_secs);
    info!("  Total proofs: {}", stats.proofs_completed);
    info!("========================================");

    // Steam integration
    #[cfg(feature = "steam")]
    if let Some(steam) = steam {
        // Update Steam stats
        steam.update_stat(steam::STAT_PROOFS_COMPLETED, stats.proofs_completed as i32);
        steam.update_stat(steam::STAT_LEVELS_COMPLETED, stats.levels_completed as i32);

        // Check and unlock achievements
        match stats.proofs_completed {
            1 => {
                info!("Achievement unlocked: First Proof!");
                steam.unlock_achievement(steam::ACHIEVEMENT_FIRST_PROOF);
            }
            10 => {
                info!("Achievement unlocked: Ten Proofs!");
                steam.unlock_achievement(steam::ACHIEVEMENT_10_PROOFS);
            }
            100 => {
                info!("Achievement unlocked: Hundred Proofs!");
                steam.unlock_achievement(steam::ACHIEVEMENT_100_PROOFS);
            }
            _ => {}
        }

        // Check for speedrun achievement (level completed in < 60 seconds)
        if stats.last_level_time_secs < 60 {
            info!("Achievement unlocked: Speedrunner!");
            steam.unlock_achievement(steam::ACHIEVEMENT_SPEEDRUN);
        }
    }

    // Export proof
    let proof = ExportedProof::from_level(&current_level.0, stats.last_level_time_secs);
    info!("Proof exported: {} bytes SMT-LIB2", proof.proof_smt2.len());

    // Submit proof to server (async, non-blocking)
    #[cfg(feature = "network")]
    {
        let network_clone = network.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                match network_clone.submit_proof(proof).await {
                    Ok(response) => {
                        info!("Proof submitted successfully!");
                        info!("  Points awarded: {}", response.points_awarded);
                        if let Some(rank) = response.global_rank {
                            info!("  Global rank: #{}", rank);
                        }
                    }
                    Err(e) => {
                        warn!("Failed to submit proof: {}", e);
                        warn!("  (Will retry later)");
                    }
                }
            });
        });
    }
}
