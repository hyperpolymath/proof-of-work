// SPDX-License-Identifier: MIT OR Apache-2.0
//! Level selection UI for browsing and selecting levels from packs.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::LevelPackManager;
use crate::states::GameState;

/// Render the level selection UI
pub fn level_select_ui_system(
    mut contexts: EguiContexts,
    mut pack_manager: ResMut<LevelPackManager>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // ESC to go back
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
        return;
    }

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    // Track actions to perform after UI rendering
    let mut select_pack: Option<usize> = None;
    let mut select_level: Option<usize> = None;
    let mut play_level = false;

    // Collect pack data for rendering
    let pack_data: Vec<_> = pack_manager
        .packs
        .iter()
        .enumerate()
        .map(|(idx, pack)| {
            let is_selected = pack_manager.current_pack_index == Some(idx);
            let completed_count = pack_manager
                .progress
                .get(&pack.id)
                .map(|p| p.completed.len())
                .unwrap_or(0);
            let total_count = pack.levels.len();
            let completion_pct = if total_count > 0 {
                (completed_count as f32 / total_count as f32 * 100.0) as u32
            } else {
                0
            };
            (
                idx,
                pack.name.clone(),
                pack.author.clone(),
                pack.difficulty,
                is_selected,
                completed_count,
                total_count,
                completion_pct,
            )
        })
        .collect();

    // Collect level data for selected pack
    let level_data: Option<(String, Vec<_>)> = pack_manager
        .current_pack_index
        .and_then(|pack_idx| pack_manager.packs.get(pack_idx))
        .map(|pack| {
            let pack_id = pack.id.clone();
            let levels: Vec<_> = pack
                .levels
                .iter()
                .enumerate()
                .map(|(level_idx, level)| {
                    let is_selected = pack_manager.current_level_index == Some(level_idx);
                    let is_completed = pack_manager.is_level_completed(&pack_id, level.id);
                    let best_time = if is_completed {
                        pack_manager
                            .progress
                            .get(&pack_id)
                            .and_then(|p| p.completed.get(&level.id))
                            .map(|c| (c.best_time_secs, c.times_completed))
                    } else {
                        None
                    };
                    (
                        level_idx,
                        level.id,
                        level.name.clone(),
                        level.description.clone(),
                        is_selected,
                        is_completed,
                        best_time,
                    )
                })
                .collect();
            (pack_id, levels)
        });

    let has_level_selected = pack_manager.current_level_index.is_some();

    // Main panel
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading(egui::RichText::new("Level Select").size(36.0));
            ui.add_space(20.0);
        });

        ui.separator();

        // Two-column layout: packs on left, levels on right
        ui.columns(2, |columns| {
            // Left column: Pack list
            columns[0].heading("Level Packs");
            columns[0].separator();

            egui::ScrollArea::vertical()
                .id_salt("pack_list")
                .show(&mut columns[0], |ui| {
                    for (idx, name, author, difficulty, is_selected, completed, total, pct) in
                        &pack_data
                    {
                        ui.group(|ui| {
                            let response = ui.selectable_label(
                                *is_selected,
                                egui::RichText::new(name).size(18.0),
                            );

                            if response.clicked() {
                                select_pack = Some(*idx);
                            }

                            ui.label(format!("by {}", author));
                            ui.label(format!("{}/{} completed ({}%)", completed, total, pct));

                            // Difficulty stars
                            let stars = "★".repeat(*difficulty as usize)
                                + &"☆".repeat(5 - *difficulty as usize);
                            ui.label(format!("Difficulty: {}", stars));
                        });

                        ui.add_space(5.0);
                    }
                });

            // Right column: Level list for selected pack
            columns[1].heading("Levels");
            columns[1].separator();

            if let Some((_pack_id, levels)) = &level_data {
                egui::ScrollArea::vertical()
                    .id_salt("level_list")
                    .show(&mut columns[1], |ui| {
                        for (level_idx, _level_id, name, description, is_selected, is_completed, best_time) in
                            levels
                        {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    // Completion indicator
                                    if *is_completed {
                                        ui.colored_label(egui::Color32::GREEN, "✓");
                                    } else {
                                        ui.label("○");
                                    }

                                    let response = ui.selectable_label(
                                        *is_selected,
                                        egui::RichText::new(name).size(16.0),
                                    );

                                    if response.clicked() {
                                        select_level = Some(*level_idx);
                                    }

                                    // Double-click to play
                                    if response.double_clicked() {
                                        select_level = Some(*level_idx);
                                        play_level = true;
                                    }
                                });

                                ui.label(description);

                                // Show best time if completed
                                if let Some((best, times)) = best_time {
                                    ui.label(format!("Best: {}s ({} completions)", best, times));
                                }
                            });

                            ui.add_space(3.0);
                        }
                    });

                // Play button for selected level
                columns[1].add_space(10.0);
                if columns[1]
                    .add_sized(
                        [150.0, 40.0],
                        egui::Button::new(egui::RichText::new("Play Selected").size(18.0)),
                    )
                    .clicked()
                {
                    if has_level_selected {
                        play_level = true;
                    }
                }
            } else {
                columns[1].label("Select a pack to see levels");
            }
        });
    });

    // Bottom panel with navigation
    egui::TopBottomPanel::bottom("level_select_nav").show(ctx, |ui| {
        ui.horizontal_centered(|ui| {
            if ui.button("Back to Menu").clicked() {
                next_state.set(GameState::MainMenu);
            }

            ui.separator();

            ui.label("Double-click a level to play");

            ui.separator();

            ui.label("ESC: Back");
        });
    });

    // Apply actions after UI rendering
    if let Some(pack_idx) = select_pack {
        pack_manager.select_pack(pack_idx);
    }
    if let Some(level_idx) = select_level {
        pack_manager.select_level(level_idx);
    }
    if play_level {
        next_state.set(GameState::Playing);
    }
}

/// Initialize level pack manager and load packs
pub fn init_level_packs(mut pack_manager: ResMut<LevelPackManager>) {
    if let Err(e) = pack_manager.load_all() {
        warn!("Failed to load level packs: {}", e);
    }

    // Load progress
    let progress_path = pack_manager.packs_dir.join("progress.json");
    if let Err(e) = pack_manager.load_progress(&progress_path) {
        warn!("Failed to load progress: {}", e);
    }

    // Select first pack if available
    if !pack_manager.packs.is_empty() {
        pack_manager.select_pack(0);
    }

    info!("Loaded {} level packs", pack_manager.packs.len());
}

/// Save progress when exiting level select
pub fn save_level_progress(pack_manager: Res<LevelPackManager>) {
    let progress_path = pack_manager.packs_dir.join("progress.json");
    if let Err(e) = pack_manager.save_progress(&progress_path) {
        warn!("Failed to save progress: {}", e);
    }
}
