// SPDX-License-Identifier: MIT OR Apache-2.0

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::game::{CurrentLevel, PlaceablePiece, PlayerStats, SelectedPieceType};
use crate::GameState;

/// Main menu system - renders the start screen
pub fn main_menu_system(mut contexts: EguiContexts, mut next_state: ResMut<NextState<GameState>>) {
    let Ok(ctx) = contexts.ctx_mut() else { return };

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(150.0);

            ui.heading(egui::RichText::new("PROOF OF WORK").size(48.0).strong());
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new("A Logic Puzzle Game")
                    .size(20.0)
                    .italics(),
            );

            ui.add_space(60.0);

            if ui
                .add_sized(
                    [200.0, 50.0],
                    egui::Button::new(egui::RichText::new("Play").size(24.0)),
                )
                .clicked()
            {
                next_state.set(GameState::Playing);
            }

            ui.add_space(20.0);

            ui.label(egui::RichText::new("Connect assumptions to prove the goal").size(14.0));
            ui.label(
                egui::RichText::new("Verified with Z3 SMT Solver")
                    .size(12.0)
                    .weak(),
            );

            ui.add_space(100.0);

            ui.label(egui::RichText::new("Controls:").size(14.0).strong());
            ui.label("Click pieces to select, drag to move");
            ui.label("Press SPACE to verify your solution");
            ui.label("Press ESC to return to menu");
        });
    });
}

/// Handle menu input (keyboard shortcuts)
pub fn handle_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

/// Game HUD - shows level info, piece palette, and controls
pub fn update_hud(
    mut contexts: EguiContexts,
    level_query: Query<&CurrentLevel>,
    stats: Res<PlayerStats>,
    mut selected: ResMut<SelectedPieceType>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // ESC to return to menu
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
        return;
    }

    let Ok(ctx) = contexts.ctx_mut() else { return };

    // Top panel - level info
    egui::TopBottomPanel::top("hud_top").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if let Ok(level) = level_query.single() {
                ui.heading(&level.0.name);
                ui.separator();
                ui.label(&level.0.description);
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("Proofs: {}", stats.proofs_completed));
            });
        });
    });

    // Left panel - piece palette
    egui::SidePanel::left("palette")
        .min_width(150.0)
        .show(ctx, |ui| {
            ui.heading("Pieces");
            ui.separator();

            ui.label("Click to select, then click on grid to place:");
            ui.add_space(10.0);

            let and_selected = selected.piece_type == Some(PlaceablePiece::AndGate);
            if ui
                .add_sized(
                    [130.0, 40.0],
                    egui::SelectableLabel::new(
                        and_selected,
                        egui::RichText::new("AND Gate").size(16.0),
                    ),
                )
                .clicked()
            {
                selected.piece_type = if and_selected {
                    None
                } else {
                    Some(PlaceablePiece::AndGate)
                };
            }

            let or_selected = selected.piece_type == Some(PlaceablePiece::OrGate);
            if ui
                .add_sized(
                    [130.0, 40.0],
                    egui::SelectableLabel::new(
                        or_selected,
                        egui::RichText::new("OR Gate").size(16.0),
                    ),
                )
                .clicked()
            {
                selected.piece_type = if or_selected {
                    None
                } else {
                    Some(PlaceablePiece::OrGate)
                };
            }

            ui.add_space(20.0);
            ui.separator();

            if let Some(piece) = &selected.piece_type {
                ui.label(format!("Selected: {:?}", piece));
                ui.label("Right-click grid to place");
            } else {
                ui.label("No piece selected");
            }

            ui.add_space(20.0);
            ui.separator();
            ui.heading("Legend");
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::from_rgb(76, 204, 76), "■");
                ui.label("Assumption");
            });
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::from_rgb(204, 76, 76), "■");
                ui.label("Goal");
            });
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::from_rgb(128, 128, 204), "■");
                ui.label("AND Gate");
            });
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::from_rgb(204, 128, 128), "■");
                ui.label("OR Gate");
            });
        });

    // Bottom panel - controls
    egui::TopBottomPanel::bottom("controls").show(ctx, |ui| {
        ui.horizontal_centered(|ui| {
            ui.label("SPACE: Verify Solution");
            ui.separator();
            ui.label("Arrow Keys / Mouse: Move cursor");
            ui.separator();
            ui.label("Left Click: Select piece");
            ui.separator();
            ui.label("Right Click: Place selected piece");
            ui.separator();
            ui.label("ESC: Menu");
        });
    });
}

/// Level completion screen
pub fn show_completion_screen(
    mut contexts: EguiContexts,
    stats: Res<PlayerStats>,
    level_query: Query<&CurrentLevel>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return };

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(150.0);

            ui.heading(egui::RichText::new("PROOF VERIFIED!").size(48.0).strong());
            ui.add_space(20.0);

            if let Ok(level) = level_query.single() {
                ui.label(egui::RichText::new(format!("Level: {}", level.0.name)).size(20.0));
            }

            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(format!("Time: {}s", stats.last_level_time_secs)).size(18.0),
            );
            ui.label(
                egui::RichText::new(format!("Total Proofs: {}", stats.proofs_completed)).size(18.0),
            );

            ui.add_space(40.0);

            ui.label(
                egui::RichText::new("Your solution has been verified by the Z3 SMT solver.")
                    .size(14.0),
            );
            ui.label(
                egui::RichText::new("The proof is mathematically sound!")
                    .size(14.0)
                    .italics(),
            );

            ui.add_space(40.0);

            ui.label("Press ENTER to continue");
            ui.label("Press ESC for menu");
        });
    });
}

/// Handle completion screen input
pub fn handle_completion_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        // Go to next level (for now, replay)
        next_state.set(GameState::Playing);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}
