pub mod board;
pub mod pieces;
pub mod validation;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

pub use board::*;
pub use pieces::*;
pub use validation::*;

// Level definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub theorem: String,
    pub initial_state: BoardState,
    pub goal_state: GoalCondition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardState {
    pub width: u32,
    pub height: u32,
    pub pieces: Vec<LogicPiece>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalCondition {
    ConnectNodes { start: (u32, u32), end: (u32, u32) },
    ProveFormula { formula: String },
    BuildProofTree { depth: u32 },
}

// Components
#[derive(Component)]
pub struct CurrentLevel(pub Level);

#[derive(Component)]
pub struct PlayerCursor {
    pub position: Vec2,
    pub selected_piece: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct PlayerStats {
    pub proofs_completed: u32,
    pub levels_completed: u32,
    pub total_playtime_secs: u64,
    pub last_level_time_secs: u64,
    pub level_start_time: Option<Instant>,
}

impl PlayerStats {
    pub fn start_level(&mut self) {
        self.level_start_time = Some(Instant::now());
    }

    pub fn complete_level(&mut self) {
        if let Some(start_time) = self.level_start_time {
            self.last_level_time_secs = start_time.elapsed().as_secs();
            self.level_start_time = None;
        }
    }
}

// Marker component for cleanup
#[derive(Component)]
pub struct GameEntity;

// Load level system
pub fn load_level(
    mut commands: Commands,
    mut stats: ResMut<PlayerStats>,
) {
    info!("Loading level...");

    // Start timing
    stats.start_level();

    // Create a simple test level
    let level = Level {
        id: 1,
        name: "First Steps".to_string(),
        description: "Connect P and Q to prove R".to_string(),
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
                        position: (4, 5),
                },
                LogicPiece::Goal {
                    formula: "R".to_string(),
                        position: (8, 5),
                },
            ],
        },
        goal_state: GoalCondition::ConnectNodes {
            start: (2, 5),
            end: (8, 5),
        },
    };

    info!("  Level: {}", level.name);
    info!("  Pieces: {}", level.initial_state.pieces.len());

    commands.spawn((CurrentLevel(level), GameEntity));
}

// Spawn pieces system
pub fn spawn_pieces(
    mut commands: Commands,
    level_query: Query<&CurrentLevel, Added<CurrentLevel>>,
    asset_server: Res<AssetServer>,
) {
    let Ok(current_level) = level_query.get_single() else { return };

    info!("Spawning {} pieces", current_level.0.initial_state.pieces.len());

    // Spawn each piece
    for piece in &current_level.0.initial_state.pieces {
        commands.spawn((
            PieceBundle::new(piece.clone(), &asset_server),
                        GameEntity,
        ));
    }

    // Spawn player cursor
    commands.spawn((
        PlayerCursor {
            position: Vec2::ZERO,
            selected_piece: None,
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.5, 0.8, 1.0, 0.5),
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
                    ..default()
        },
        GameEntity,
    ));

    info!("✓ All pieces spawned");
}

// Input handling system
pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut cursor_query: Query<(&mut PlayerCursor, &mut Transform)>,
                    windows: Query<&Window>,
                    camera_query: Query<(&Camera, &GlobalTransform)>,
                    piece_query: Query<(Entity, &LogicPiece, &Transform), Without<PlayerCursor>>,
) {
    let Ok((mut cursor, mut cursor_transform)) = cursor_query.get_single_mut() else { return };
    let Ok(window) = windows.get_single() else { return };
    let Ok((camera, camera_transform)) = camera_query.get_single() else { return };

    // Get mouse position in world space
    if let Some(screen_pos) = window.cursor_position() {
        if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
            cursor.position = world_pos;
            cursor_transform.translation = world_pos.extend(10.0);
        }
    }

    // Handle piece selection on click
    if mouse.just_pressed(MouseButton::Left) {
        let cursor_pos = cursor.position;

        // Check if we clicked on a piece
        for (entity, _piece, transform) in piece_query.iter() {
            let piece_pos = transform.translation.truncate();
            let distance = cursor_pos.distance(piece_pos);

            if distance < 32.0 { // Within 32 pixels
                if cursor.selected_piece == Some(entity) {
                    // Deselect
                    cursor.selected_piece = None;
                    info!("Piece deselected");
                } else {
                    // Select
                    cursor.selected_piece = Some(entity);
                    info!("Piece selected: {:?}", entity);
                }
                break;
            }
        }
    }

    // Keyboard movement for precision
    let mut move_delta = Vec2::ZERO;
    let move_speed = 80.0; // Grid size

    if keyboard.just_pressed(KeyCode::ArrowUp) { move_delta.y += move_speed; }
    if keyboard.just_pressed(KeyCode::ArrowDown) { move_delta.y -= move_speed; }
    if keyboard.just_pressed(KeyCode::ArrowLeft) { move_delta.x -= move_speed; }
    if keyboard.just_pressed(KeyCode::ArrowRight) { move_delta.x += move_speed; }

    if move_delta != Vec2::ZERO {
        cursor.position += move_delta;
        cursor_transform.translation = cursor.position.extend(10.0);
    }
}

// Update board system (placeholder)
pub fn update_board() {
    // This is where pieces move, connections are made, etc.
}

// Update piece positions system
pub fn update_piece_positions(
    cursor_query: Query<&PlayerCursor>,
    mut piece_query: Query<(&mut Transform, &mut LogicPiece)>,
) {
    let Ok(cursor) = cursor_query.get_single() else { return };

    if let Some(selected_entity) = cursor.selected_piece {
        if let Ok((mut transform, mut piece)) = piece_query.get_mut(selected_entity) {
            // Snap to grid
            let grid_pos = (
                (cursor.position.x / 80.0).round() as u32,
                            (cursor.position.y / 80.0).round() as u32,
            );

            // Update piece position
            piece.set_position(grid_pos);

            // Update visual position
            transform.translation.x = grid_pos.0 as f32 * 80.0;
            transform.translation.y = grid_pos.1 as f32 * 80.0;
        }
    }
}

// Check connections system (placeholder)
pub fn check_connections() {
    // Check if pieces are properly connected
}

// Check solution system
pub fn check_solution(
    level_query: Query<&CurrentLevel>,
    piece_query: Query<&LogicPiece>,
    mut next_state: ResMut<NextState<crate::GameState>>,
    mut stats: ResMut<PlayerStats>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok(current_level) = level_query.get_single() else { return };

    // Manual trigger for testing (Space bar)
    if keyboard.just_pressed(KeyCode::Space) {
        info!("Checking solution...");

        // Collect all pieces
        let pieces: Vec<LogicPiece> = piece_query.iter().cloned().collect();

        // Use Z3 to verify
        if crate::verification::verify_level_solution(&current_level.0) {
            info!("✓ Solution is CORRECT!");
            stats.complete_level();
            next_state.set(crate::GameState::LevelComplete);
        } else {
            warn!("✗ Solution is incorrect - keep trying!");
        }
    }
}

// Cleanup system
pub fn cleanup_level(
    mut commands: Commands,
    entities: Query<Entity, With<GameEntity>>,
) {
    info!("Cleaning up level...");

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    info!("✓ Level cleaned up");
}
