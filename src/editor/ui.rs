// SPDX-License-Identifier: MIT OR Apache-2.0
//! Editor UI systems using egui.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::{EditorEntity, EditorPieceType, EditorState, EditorTool, SaveLevelEvent, TestLevelEvent};
use crate::game::{GoalCondition, LogicPiece};
use crate::levels::LevelPackManager;
use crate::states::GameState;

/// Render the editor UI
pub fn editor_ui_system(
    mut contexts: EguiContexts,
    mut editor: ResMut<EditorState>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut test_events: MessageWriter<TestLevelEvent>,
    mut save_events: MessageWriter<SaveLevelEvent>,
    pack_manager: Option<Res<LevelPackManager>>,
) {
    // ESC to exit editor
    if keyboard.just_pressed(KeyCode::Escape) {
        if editor.dirty {
            editor.status_message = "Unsaved changes! Press ESC again to discard.".to_string();
            // TODO: Add confirmation dialog
        } else {
            next_state.set(GameState::MainMenu);
            return;
        }
    }

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    // Top toolbar
    egui::TopBottomPanel::top("editor_toolbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Level Editor");
            ui.separator();

            // File operations
            if ui.button("New").clicked() {
                *editor = EditorState::default();
            }

            if ui.button("Test").clicked() {
                test_events.write(TestLevelEvent);
            }

            if ui.button("Save").clicked() {
                match editor.validate() {
                    Ok(_) => {
                        save_events.write(SaveLevelEvent {
                            to_pack_id: editor.pack_id.clone(),
                        });
                    }
                    Err(errors) => {
                        editor.status_message = format!("Cannot save: {}", errors.join(", "));
                    }
                }
            }

            ui.separator();

            if ui.button("Exit").clicked() {
                next_state.set(GameState::MainMenu);
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if editor.dirty {
                    ui.label(egui::RichText::new("*Unsaved*").color(egui::Color32::YELLOW));
                }
                ui.label(&editor.status_message);
            });
        });
    });

    // Left panel - piece palette and tools
    egui::SidePanel::left("editor_palette")
        .min_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Tools");
            ui.separator();

            // Tool selection
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(editor.tool == EditorTool::Select, "Select")
                    .clicked()
                {
                    editor.tool = EditorTool::Select;
                }
                if ui
                    .selectable_label(editor.tool == EditorTool::Place, "Place")
                    .clicked()
                {
                    editor.tool = EditorTool::Place;
                }
                if ui
                    .selectable_label(editor.tool == EditorTool::Delete, "Delete")
                    .clicked()
                {
                    editor.tool = EditorTool::Delete;
                }
            });

            ui.add_space(10.0);
            ui.heading("Pieces");
            ui.separator();

            // Piece type selection
            let piece_types = [
                EditorPieceType::Assumption,
                EditorPieceType::Goal,
                EditorPieceType::AndIntro,
                EditorPieceType::OrIntro,
                EditorPieceType::ImpliesIntro,
                EditorPieceType::NotIntro,
                EditorPieceType::ForallIntro,
                EditorPieceType::ExistsIntro,
            ];

            for piece_type in piece_types {
                let selected = editor.selected_piece == Some(piece_type);
                if ui
                    .selectable_label(selected, piece_type.name())
                    .clicked()
                {
                    editor.selected_piece = Some(piece_type);
                    editor.tool = EditorTool::Place;
                }
            }

            ui.add_space(10.0);

            // Formula/variable input based on selected piece
            if let Some(piece_type) = editor.selected_piece {
                if piece_type.needs_formula() {
                    ui.label("Formula:");
                    ui.text_edit_singleline(&mut editor.formula_input);
                }
                if piece_type.needs_variable() {
                    ui.label("Variable:");
                    ui.text_edit_singleline(&mut editor.variable_input);
                }
            }

            ui.add_space(20.0);
            ui.heading("Legend");
            ui.separator();

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
                ui.label("Logic Gate");
            });
        });

    // Right panel - level properties
    egui::SidePanel::right("editor_properties")
        .min_width(250.0)
        .show(ctx, |ui| {
            ui.heading("Level Properties");
            ui.separator();

            ui.label("Name:");
            if ui
                .text_edit_singleline(&mut editor.level.name)
                .changed()
            {
                editor.dirty = true;
            }

            ui.add_space(5.0);
            ui.label("Description:");
            if ui
                .text_edit_multiline(&mut editor.level.description)
                .changed()
            {
                editor.dirty = true;
            }

            ui.add_space(5.0);
            ui.label("Theorem (SMT-LIB2):");
            if ui
                .text_edit_singleline(&mut editor.level.theorem)
                .changed()
            {
                editor.dirty = true;
            }

            ui.add_space(10.0);
            ui.separator();
            ui.heading("Grid Size");

            let mut width = editor.grid_width as i32;
            let mut height = editor.grid_height as i32;

            ui.horizontal(|ui| {
                ui.label("Width:");
                if ui.add(egui::DragValue::new(&mut width).range(3..=20)).changed() {
                    editor.set_grid_size(width as u32, height as u32);
                }
            });

            ui.horizontal(|ui| {
                ui.label("Height:");
                if ui.add(egui::DragValue::new(&mut height).range(3..=20)).changed() {
                    editor.set_grid_size(width as u32, height as u32);
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.heading("Goal Condition");

            let mut goal_formula = match &editor.level.goal_state {
                GoalCondition::ProveFormula { formula } => formula.clone(),
                _ => String::new(),
            };

            ui.label("Goal Formula:");
            if ui.text_edit_singleline(&mut goal_formula).changed() {
                editor.level.goal_state = GoalCondition::ProveFormula {
                    formula: goal_formula,
                };
                editor.dirty = true;
            }

            ui.add_space(10.0);
            ui.separator();
            ui.heading("Pieces");

            ui.label(format!(
                "Total: {} pieces",
                editor.level.initial_state.pieces.len()
            ));

            let assumption_count = editor
                .level
                .initial_state
                .pieces
                .iter()
                .filter(|p| matches!(p, LogicPiece::Assumption { .. }))
                .count();
            let goal_count = editor
                .level
                .initial_state
                .pieces
                .iter()
                .filter(|p| matches!(p, LogicPiece::Goal { .. }))
                .count();

            ui.label(format!("Assumptions: {}", assumption_count));
            ui.label(format!("Goals: {}", goal_count));

            // Validation status
            ui.add_space(10.0);
            match editor.validate() {
                Ok(_) => {
                    ui.colored_label(egui::Color32::GREEN, "✓ Valid level");
                }
                Err(errors) => {
                    ui.colored_label(egui::Color32::RED, "✗ Invalid:");
                    for error in errors {
                        ui.label(format!("  - {}", error));
                    }
                }
            }

            // Pack selection
            if let Some(pack_manager) = &pack_manager {
                ui.add_space(10.0);
                ui.separator();
                ui.heading("Save To Pack");

                egui::ComboBox::from_label("Pack")
                    .selected_text(
                        editor
                            .pack_id
                            .as_ref()
                            .map(|id| {
                                pack_manager
                                    .packs
                                    .iter()
                                    .find(|p| &p.id == id)
                                    .map(|p| p.name.as_str())
                                    .unwrap_or(id.as_str())
                            })
                            .unwrap_or("Select pack..."),
                    )
                    .show_ui(ui, |ui| {
                        for pack in &pack_manager.packs {
                            if ui
                                .selectable_label(
                                    editor.pack_id.as_ref() == Some(&pack.id),
                                    &pack.name,
                                )
                                .clicked()
                            {
                                editor.pack_id = Some(pack.id.clone());
                            }
                        }
                    });
            }
        });

    // Bottom panel - instructions
    egui::TopBottomPanel::bottom("editor_help").show(ctx, |ui| {
        ui.horizontal_centered(|ui| {
            ui.label("Left-click: Place/Select");
            ui.separator();
            ui.label("Right-click: Delete");
            ui.separator();
            ui.label("Middle-click: Pan");
            ui.separator();
            ui.label("Scroll: Zoom");
            ui.separator();
            ui.label("ESC: Exit");
        });
    });
}

/// Handle editor input (piece placement, selection, deletion)
pub fn editor_input_system(
    mut editor: ResMut<EditorState>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    // Get mouse position in world space
    if let Some(screen_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
            // Convert to grid coordinates
            let grid_x = ((world_pos.x / 80.0).round() as i32 + (editor.grid_width as i32 / 2))
                .clamp(0, editor.grid_width as i32 - 1) as u32;
            let grid_y = ((world_pos.y / 80.0).round() as i32 + (editor.grid_height as i32 / 2))
                .clamp(0, editor.grid_height as i32 - 1) as u32;

            // Left click - place or select
            if mouse.just_pressed(MouseButton::Left) {
                match editor.tool {
                    EditorTool::Place => {
                        if let Some(piece_type) = editor.selected_piece {
                            // Validate formula input for pieces that need it
                            if piece_type.needs_formula() && editor.formula_input.trim().is_empty()
                            {
                                editor.status_message =
                                    "Enter a formula before placing".to_string();
                            } else {
                                let piece = piece_type.to_logic_piece(
                                    (grid_x, grid_y),
                                    &editor.formula_input,
                                    &editor.variable_input,
                                );
                                editor.add_piece(piece);
                            }
                        } else {
                            editor.status_message = "Select a piece type first".to_string();
                        }
                    }
                    EditorTool::Delete => {
                        editor.remove_piece_at((grid_x, grid_y));
                    }
                    EditorTool::Select => {
                        if let Some(piece) = editor.get_piece_at((grid_x, grid_y)) {
                            editor.status_message = format!("Selected: {}", piece.label());
                        }
                    }
                    _ => {}
                }
            }

            // Right click - delete
            if mouse.just_pressed(MouseButton::Right) {
                editor.remove_piece_at((grid_x, grid_y));
            }
        }
    }
}

/// Spawn editor grid visualization
pub fn spawn_editor_grid(
    mut commands: Commands,
    editor: Res<EditorState>,
) {
    let half_width = editor.grid_width as f32 / 2.0;
    let half_height = editor.grid_height as f32 / 2.0;

    // Spawn grid cells
    for x in 0..editor.grid_width {
        for y in 0..editor.grid_height {
            commands.spawn((
                Sprite {
                    color: if (x + y) % 2 == 0 {
                        Color::srgba(0.15, 0.15, 0.2, 1.0)
                    } else {
                        Color::srgba(0.12, 0.12, 0.17, 1.0)
                    },
                    custom_size: Some(Vec2::new(78.0, 78.0)),
                    ..default()
                },
                Transform::from_xyz(
                    (x as f32 - half_width + 0.5) * 80.0,
                    (y as f32 - half_height + 0.5) * 80.0,
                    -1.0,
                ),
                EditorEntity,
            ));
        }
    }

    // Spawn existing pieces
    for piece in &editor.level.initial_state.pieces {
        let (x, y) = piece.position();
        commands.spawn((
            Sprite {
                color: piece.color(),
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            Transform::from_xyz(
                (x as f32 - half_width + 0.5) * 80.0,
                (y as f32 - half_height + 0.5) * 80.0,
                0.0,
            ),
            piece.clone(),
            EditorEntity,
        ));
    }
}

/// Update editor grid when pieces change
pub fn update_editor_pieces(
    mut commands: Commands,
    editor: Res<EditorState>,
    existing_pieces: Query<(Entity, &LogicPiece), With<EditorEntity>>,
) {
    if !editor.is_changed() {
        return;
    }

    let half_width = editor.grid_width as f32 / 2.0;
    let half_height = editor.grid_height as f32 / 2.0;

    // Find pieces to remove (in ECS but not in editor state)
    for (entity, piece) in existing_pieces.iter() {
        let pos = piece.position();
        if !editor
            .level
            .initial_state
            .pieces
            .iter()
            .any(|p| p.position() == pos)
        {
            commands.entity(entity).despawn();
        }
    }

    // Find pieces to add (in editor state but not in ECS)
    for piece in &editor.level.initial_state.pieces {
        let pos = piece.position();
        if !existing_pieces.iter().any(|(_, p)| p.position() == pos) {
            commands.spawn((
                Sprite {
                    color: piece.color(),
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                Transform::from_xyz(
                    (pos.0 as f32 - half_width + 0.5) * 80.0,
                    (pos.1 as f32 - half_height + 0.5) * 80.0,
                    0.0,
                ),
                piece.clone(),
                EditorEntity,
            ));
        }
    }
}

/// Cleanup editor entities
pub fn cleanup_editor(mut commands: Commands, entities: Query<Entity, With<EditorEntity>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}

/// Handle test level event
pub fn handle_test_level(
    mut events: MessageReader<TestLevelEvent>,
    editor: Res<EditorState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in events.read() {
        match editor.validate() {
            Ok(_) => {
                // TODO: Store the level for testing
                info!("Testing level: {}", editor.level.name);
                next_state.set(GameState::Playing);
            }
            Err(errors) => {
                warn!("Cannot test level: {:?}", errors);
            }
        }
    }
}

/// Handle save level event
pub fn handle_save_level(
    mut events: MessageReader<SaveLevelEvent>,
    mut editor: ResMut<EditorState>,
    mut pack_manager: Option<ResMut<LevelPackManager>>,
) {
    for event in events.read() {
        let level = editor.build_level();

        if let Some(pack_manager) = &mut pack_manager {
            if let Some(pack_id) = &event.to_pack_id {
                // Find the pack index first
                let pack_idx = pack_manager.packs.iter().position(|p| &p.id == pack_id);

                if let Some(idx) = pack_idx {
                    // Update existing or add new level
                    let pack = &mut pack_manager.packs[idx];
                    if let Some(existing) = pack.levels.iter_mut().find(|l| l.id == level.id) {
                        *existing = level.clone();
                        editor.status_message = format!("Updated level in {}", pack.name);
                    } else {
                        pack.levels.push(level.clone());
                        editor.status_message = format!("Added level to {}", pack.name);
                    }

                    // Clone pack for saving
                    let pack_to_save = pack_manager.packs[idx].clone();

                    // Save pack to disk
                    match pack_manager.save_pack(&pack_to_save) {
                        Ok(path) => {
                            info!("Saved pack to {:?}", path);
                            editor.dirty = false;
                        }
                        Err(e) => {
                            editor.status_message = format!("Save failed: {}", e);
                        }
                    }
                } else {
                    editor.status_message = format!("Pack '{}' not found", pack_id);
                }
            } else {
                editor.status_message = "Select a pack to save to".to_string();
            }
        } else {
            editor.status_message = "Level pack manager not available".to_string();
        }
    }
}
