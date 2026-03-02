// SPDX-License-Identifier: PMPL-1.0-or-later
//! Game systems that run within the Bevy game loop.
//! These are separated from the core game module because they depend on GameState.

use bevy::prelude::*;

use crate::game::{
    CurrentLevel, GameEntity, Level, LogicPiece, PlaceablePiece, PlayerCursor, PlayerPlaced,
    PlayerStats, SelectedPieceType, BoardState, GoalCondition, PieceBundle,
};
use crate::states::GameState;

// Load level system
pub fn load_level(mut commands: Commands, mut stats: ResMut<PlayerStats>) {
    info!("Loading level...");

    // Start timing
    stats.start_level();

    // Create the vertical slice puzzle: P AND Q => R
    let level = Level {
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
    };

    info!("  Level: {}", level.name);
    info!("  Pieces: {}", level.initial_state.pieces.len());
    info!("  Hint: Place an AND gate between the assumptions and the goal!");

    commands.spawn((CurrentLevel(level), GameEntity));
}

// Spawn pieces system
pub fn spawn_pieces(
    mut commands: Commands,
    level_query: Query<&CurrentLevel, Added<CurrentLevel>>,
    asset_server: Res<AssetServer>,
) {
    let Ok(current_level) = level_query.single() else {
        return;
    };

    info!(
        "Spawning {} pieces",
        current_level.0.initial_state.pieces.len()
    );

    // Spawn grid background
    for x in 0..10 {
        for y in 0..10 {
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
                    (x as f32 - 4.5) * 80.0,
                    (y as f32 - 4.5) * 80.0,
                    -1.0,
                ),
                GameEntity,
            ));
        }
    }

    // Spawn each piece
    for piece in &current_level.0.initial_state.pieces {
        let (x, y) = piece.position();
        let bundle = PieceBundle::new(piece.clone(), &asset_server);
        let mut entity = commands.spawn((bundle, GameEntity));

        // Offset to center the grid
        entity.insert(Transform::from_xyz(
            (x as f32 - 4.5) * 80.0,
            (y as f32 - 4.5) * 80.0,
            0.0,
        ));
    }

    // Spawn player cursor
    commands.spawn((
        PlayerCursor {
            position: Vec2::ZERO,
            selected_piece: None,
        },
        Sprite {
            color: Color::srgba(0.5, 0.8, 1.0, 0.3),
            custom_size: Some(Vec2::new(76.0, 76.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 5.0),
        GameEntity,
    ));

    info!("Pieces spawned successfully");
}

// Input handling system
pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut cursor_query: Query<(&mut PlayerCursor, &mut Transform)>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    piece_query: Query<
        (
            Entity,
            &LogicPiece,
            &Transform,
            Option<&PlayerPlaced>,
        ),
        Without<PlayerCursor>,
    >,
    mut commands: Commands,
    selected_piece_type: Option<Res<SelectedPieceType>>,
) {
    let Ok((mut cursor, mut cursor_transform)) = cursor_query.single_mut() else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    // Get mouse position in world space
    if let Some(screen_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
            cursor.position = world_pos;

            // Snap cursor to grid
            let grid_x = ((world_pos.x / 80.0).round() as i32).clamp(-4, 5);
            let grid_y = ((world_pos.y / 80.0).round() as i32).clamp(-4, 5);
            cursor_transform.translation =
                Vec3::new(grid_x as f32 * 80.0, grid_y as f32 * 80.0, 5.0);
        }
    }

    // Handle right-click to place new piece
    if mouse.just_pressed(MouseButton::Right) {
        if let Some(selected) = &selected_piece_type {
            if let Some(piece_type) = &selected.piece_type {
                let grid_x = ((cursor.position.x / 80.0).round() as i32 + 4) as u32;
                let grid_y = ((cursor.position.y / 80.0).round() as i32 + 4) as u32;

                // Check if position is empty
                let mut occupied = false;
                for (_entity, piece, _transform, _) in piece_query.iter() {
                    if piece.position() == (grid_x, grid_y) {
                        occupied = true;
                        break;
                    }
                }

                if !occupied && grid_x < 10 && grid_y < 10 {
                    let new_piece = match piece_type {
                        PlaceablePiece::AndGate => LogicPiece::AndIntro {
                            position: (grid_x, grid_y),
                        },
                        PlaceablePiece::OrGate => LogicPiece::OrIntro {
                            position: (grid_x, grid_y),
                        },
                        PlaceablePiece::Wire => LogicPiece::Wire {
                            from: (grid_x, grid_y),
                            to: (grid_x + 1, grid_y),
                        },
                    };

                    let color = match piece_type {
                        PlaceablePiece::AndGate => Color::srgb(0.5, 0.5, 0.9),
                        PlaceablePiece::OrGate => Color::srgb(0.9, 0.5, 0.5),
                        PlaceablePiece::Wire => Color::srgb(0.6, 0.6, 0.6),
                    };

                    commands.spawn((
                        new_piece,
                        Sprite {
                            color,
                            custom_size: Some(Vec2::new(64.0, 64.0)),
                            ..default()
                        },
                        Transform::from_xyz(
                            (grid_x as f32 - 4.5) * 80.0,
                            (grid_y as f32 - 4.5) * 80.0,
                            0.0,
                        ),
                        GameEntity,
                        PlayerPlaced,
                    ));

                    info!("Placed {:?} at ({}, {})", piece_type, grid_x, grid_y);
                }
            }
        }
    }

    // Handle left-click for piece selection
    if mouse.just_pressed(MouseButton::Left) {
        let cursor_pos = cursor.position;

        // Check if we clicked on a movable piece (player-placed only)
        for (entity, _piece, transform, player_placed) in piece_query.iter() {
            if player_placed.is_some() {
                let piece_pos = transform.translation.truncate();
                let distance = cursor_pos.distance(piece_pos);

                if distance < 40.0 {
                    if cursor.selected_piece == Some(entity) {
                        cursor.selected_piece = None;
                        info!("Piece deselected");
                    } else {
                        cursor.selected_piece = Some(entity);
                        info!("Piece selected: {:?}", entity);
                    }
                    break;
                }
            }
        }
    }

    // Delete selected piece with Delete/Backspace
    if keyboard.just_pressed(KeyCode::Delete) || keyboard.just_pressed(KeyCode::Backspace) {
        if let Some(selected_entity) = cursor.selected_piece {
            // Only delete player-placed pieces
            if let Ok((_, _, _, player_placed)) = piece_query.get(selected_entity) {
                if player_placed.is_some() {
                    commands.entity(selected_entity).despawn();
                    cursor.selected_piece = None;
                    info!("Piece deleted");
                }
            }
        }
    }
}

// Update board system
pub fn update_board() {
    // Reserved for future board state updates
}

// Update piece positions system
pub fn update_piece_positions(
    cursor_query: Query<&PlayerCursor>,
    mut piece_query: Query<(&mut Transform, &mut LogicPiece, Option<&PlayerPlaced>)>,
) {
    let Ok(cursor) = cursor_query.single() else {
        return;
    };

    if let Some(selected_entity) = cursor.selected_piece {
        if let Ok((mut transform, mut piece, player_placed)) = piece_query.get_mut(selected_entity)
        {
            // Only move player-placed pieces
            if player_placed.is_some() {
                // Snap to grid
                let grid_x = ((cursor.position.x / 80.0).round() as i32 + 4).clamp(0, 9) as u32;
                let grid_y = ((cursor.position.y / 80.0).round() as i32 + 4).clamp(0, 9) as u32;

                // Update piece position
                piece.set_position((grid_x, grid_y));

                // Update visual position
                transform.translation.x = (grid_x as f32 - 4.5) * 80.0;
                transform.translation.y = (grid_y as f32 - 4.5) * 80.0;
            }
        }
    }
}

// Check connections system
pub fn check_connections() {
    // Reserved for visual connection feedback
}

// Check solution system
pub fn check_solution(
    level_query: Query<&CurrentLevel>,
    piece_query: Query<&LogicPiece>,
    mut next_state: ResMut<NextState<GameState>>,
    mut stats: ResMut<PlayerStats>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok(current_level) = level_query.single() else {
        return;
    };

    // Manual trigger for verification (Space bar)
    if keyboard.just_pressed(KeyCode::Space) {
        info!("Verifying solution...");

        // Collect all pieces
        let pieces: Vec<LogicPiece> = piece_query.iter().cloned().collect();

        info!("  Pieces on board: {}", pieces.len());
        for piece in &pieces {
            info!("    {:?}", piece);
        }

        // Verify the solution
        if crate::verification::verify_level_solution(&current_level.0, &pieces) {
            info!("PROOF VERIFIED - Solution is correct!");
            stats.complete_level();
            next_state.set(GameState::LevelComplete);
        } else {
            warn!("Solution incomplete - keep trying!");
            warn!("Hint: Place an AND gate adjacent to P and Q, and adjacent to R");
        }
    }
}

// Cleanup system
pub fn cleanup_level(mut commands: Commands, entities: Query<Entity, With<GameEntity>>) {
    info!("Cleaning up level...");

    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }

    info!("Level cleaned up");
}
