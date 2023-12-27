use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // Display
        .add_systems(Update, update_display_position.after(move_player))
        // Input
        .add_systems(Update, move_player)
        // The rest
        .run();
}

/// CONSTANTS

const LEVEL_X_MAX: u16 = 10;
const LEVEL_Y_MAX: u16 = 10;
const GRID_SQUARE_SIZE: u16 = 32;

const BACKGROUND_COLOR: Color = Color::WHITE;

const fn level_coords_to_pxl_coords(x: u16, y: u16) -> (f32, f32) {
    ((x * GRID_SQUARE_SIZE) as f32, (y * GRID_SQUARE_SIZE) as f32)
}

/// THE REST

#[derive(Component)]
struct Player;

#[derive(Component)]
struct BoxBlock;

#[derive(Component)]
struct Position {
    x: u16,
    y: u16,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    position: Position,
    sprite: SpriteBundle,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let level_pxl_size = level_coords_to_pxl_coords(LEVEL_X_MAX, LEVEL_Y_MAX);

    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: BACKGROUND_COLOR,
                custom_size: Some(Vec2::new(level_pxl_size.0, level_pxl_size.1)),
                ..Default::default()
            },
            transform: Transform::from_xyz(level_pxl_size.0 / 2.0, level_pxl_size.1 / 2.0, -1.0),
            ..Default::default()
        }
    );

    let player_pos = Position { x: LEVEL_X_MAX / 2, y: LEVEL_Y_MAX / 2 };

    commands.spawn(PlayerBundle {
        player: Player,
        position: player_pos,
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::MIDNIGHT_BLUE,
                custom_size: Some(Vec2::new(GRID_SQUARE_SIZE as f32, GRID_SQUARE_SIZE as f32)),
                ..Default::default()
            },
            ..Default::default()
        },
    });

    commands.spawn(PlayerBundle {
        player: Player,
        position: Position { x: 0, y: 0 },
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::MIDNIGHT_BLUE,
                custom_size: Some(Vec2::new(GRID_SQUARE_SIZE as f32, GRID_SQUARE_SIZE as f32)),
                ..Default::default()
            },
            ..Default::default()
        },
    });
}

fn update_display_position(mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in query.iter_mut() {
        let pxl_coords = level_coords_to_pxl_coords(position.x, position.y);
        transform.translation.x = pxl_coords.0 + GRID_SQUARE_SIZE as f32 / 2.0;
        transform.translation.y = pxl_coords.1 + GRID_SQUARE_SIZE as f32 / 2.0;
    }
}

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Position, With<Player>>) {
    for mut position in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Left) {
            if position.x > 0 {
                position.x -= 1;
            } else {
                position.x = LEVEL_X_MAX - 1;
            }
        }
        if keyboard_input.just_pressed(KeyCode::Right) {
            if position.x < LEVEL_X_MAX - 1 {
                position.x += 1;
            } else {
                position.x = 0;
            }
        }
        if keyboard_input.just_pressed(KeyCode::Up) {
            if position.y < LEVEL_Y_MAX - 1 {
                position.y += 1;
            } else {
                position.y = 0;
            }
        }
        if keyboard_input.just_pressed(KeyCode::Down) {
            if position.y > 0 {
                position.y -= 1;
            } else {
                position.y = LEVEL_Y_MAX - 1;
            }
        }
    }
}