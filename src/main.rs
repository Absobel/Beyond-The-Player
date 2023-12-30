use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // Set commands
        .add_systems(Update,
            (
                move_good_level_wall,
                conveyor_move.after(move_good_level_wall),
            )
        )
        // Process commands
        .add_systems(Update, process_move.after(conveyor_move))
        // Display
        .add_systems(Update, update_display_position.after(process_move))
        // The rest
        .insert_resource(MoveCommands { commands: vec![] })
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

/// RESOURCES

struct MoveCommand {
    entities: Vec<Entity>,
    delta: (i16, i16),
}

#[derive(Resource)]
struct MoveCommands {
    commands: Vec<MoveCommand>,
}

/// MARKER COMPONENTS

#[derive(Component)]
struct Movable;

#[derive(Component)]
struct Immovable;

#[derive(Component)]
struct GoodLevelWall;

#[derive(Component)]
struct BadLevelWall;

#[derive(Component)]
struct BoxBlock;

#[derive(Component)]
struct Conveyor;

/// UTILS COMPONENTS

#[derive(Component, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: u16,
    y: u16,
}

impl Position {
    fn to_tuple(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}

#[derive(Component, Debug)]
struct Direction(Dir);

impl Direction {
    fn to_delta(&self) -> (i16, i16) {
        self.0.to_delta()
    }
}

/// BUNDLES

#[derive(Bundle)]
struct GoodLevelWallBundle {
    good_level_wall: GoodLevelWall,
    movable: Movable,
    position: Position,
    sprite: SpriteBundle,
}

#[derive(Bundle)]
struct BadLevelWallBundle {
    bad_level_wall: BadLevelWall,
    immovable: Immovable,
    position: Position,
    sprite: SpriteBundle,
}

#[derive(Bundle)]
struct BoxBundle {
    box_block: BoxBlock,
    movable: Movable,
    position: Position,
    sprite: SpriteBundle,
}

#[derive(Bundle)]
struct ConveyorBundle {
    conveyor: Conveyor,
    position: Position,
    direction: Direction,
    sprite: SpriteBundle,
}

/// SYSTEMS

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let level_pxl_size = level_coords_to_pxl_coords(LEVEL_X_MAX, LEVEL_Y_MAX);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: BACKGROUND_COLOR,
            custom_size: Some(Vec2::new(level_pxl_size.0, level_pxl_size.1)),
            ..Default::default()
        },
        transform: Transform::from_xyz(level_pxl_size.0 / 2.0, level_pxl_size.1 / 2.0, -1.0),
        ..Default::default()
    });

    let good_level_wall_pos = Position {
        x: LEVEL_X_MAX / 2,
        y: LEVEL_Y_MAX / 2,
    };

    commands.spawn(GoodLevelWallBundle {
        good_level_wall: GoodLevelWall,
        movable: Movable,
        position: good_level_wall_pos,
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::MIDNIGHT_BLUE,
                custom_size: Some(Vec2::new(GRID_SQUARE_SIZE as f32, GRID_SQUARE_SIZE as f32)),
                ..Default::default()
            },
            transform: Transform::default().with_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        },
    });

    commands.spawn(GoodLevelWallBundle {
        good_level_wall: GoodLevelWall,
        movable: Movable,
        position: Position { x: 0, y: 0 },
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::MIDNIGHT_BLUE,
                custom_size: Some(Vec2::new(GRID_SQUARE_SIZE as f32, GRID_SQUARE_SIZE as f32)),
                ..Default::default()
            },
            transform: Transform::default().with_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        },
    });

    commands.spawn(BadLevelWallBundle {
        bad_level_wall: BadLevelWall,
        immovable: Immovable,
        position: Position { x: 3, y: 6 },
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(GRID_SQUARE_SIZE as f32, GRID_SQUARE_SIZE as f32)),
                ..Default::default()
            },
            transform: Transform::default().with_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        },
    });

    commands.spawn(BoxBundle {
        box_block: BoxBlock,
        movable: Movable,
        position: Position { x: 4, y: 6 },
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(GRID_SQUARE_SIZE as f32, GRID_SQUARE_SIZE as f32)),
                ..Default::default()
            },
            transform: Transform::default().with_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        },
    });

    commands.spawn(ConveyorBundle {
        conveyor: Conveyor,
        position: Position { x: 5, y: 6 },
        direction: Direction(Dir::Down),
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
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

fn process_move(mut move_commands: ResMut<MoveCommands>, mut movable_query: Query<(Entity, &mut Position), With<Movable>>, immovable_query: Query<&Position, (Without<Movable>, With<Immovable>)>) {
    for command in move_commands.commands.iter() {
        let movables = {
            let mut tmp = HashMap::new();
            for (entity, position) in movable_query.iter_mut() {
                tmp.insert(position.to_tuple(), entity);
            }
            tmp
        };

        let mut to_move = HashSet::new();
        for entity in command.entities.iter() {
            let mut tmp_to_move = HashSet::new();
            let mut current_pos = movable_query.get(*entity).expect("Entity that moves should be movable").1.to_tuple();
            while let Some(&entity) = movables.get(&current_pos) {
                tmp_to_move.insert(entity);
                current_pos = add_delta(current_pos, command.delta);
            }
            if immovable_query.iter().any(|pos| pos.to_tuple() == current_pos) {
                continue;
            }
            to_move.extend(tmp_to_move);
        }
        for entity in to_move.iter() {
            let mut position = movable_query.get_mut(*entity).expect("Entity that moves should be movable").1;
            let new_pos = add_delta(position.to_tuple(), command.delta);
            position.x = new_pos.0;
            position.y = new_pos.1;
        }
    }
    move_commands.commands.clear();
}

fn move_good_level_wall(keyboard_input: Res<Input<KeyCode>>, query: Query<(Entity, &Position), With<GoodLevelWall>>, mut move_commands: ResMut<MoveCommands>) {
    let delta = {
        if keyboard_input.just_pressed(KeyCode::Left) {
            (-1, 0)
        } else if keyboard_input.just_pressed(KeyCode::Right) {
            (1, 0)
        } else if keyboard_input.just_pressed(KeyCode::Up) {
            (0, 1)
        } else if keyboard_input.just_pressed(KeyCode::Down) {
            (0, -1)
        } else {
            return;
        }
    };

    move_commands.commands.push(MoveCommand {
        entities: query.iter().map(|good_level_wall| good_level_wall.0).collect(),
        delta,
    });
}

fn conveyor_move(
    conveyor_query: Query<(&Position, &Direction), With<Conveyor>>,
    movable_query: Query<(Entity, &Position), With<Movable>>,
    mut move_commands: ResMut<MoveCommands>,
) {
    for (conveyor_pos, direction) in conveyor_query.iter() {
        for (movable_entity, movable_pos) in movable_query.iter() {
            if conveyor_pos.to_tuple() == movable_pos.to_tuple() {
                move_commands.commands.push(MoveCommand {
                    entities: vec![movable_entity],
                    delta: direction.to_delta(),
                });
            }
        }
    }
}


/// UTILS

fn add_delta(pos: (u16, u16), delta: (i16, i16)) -> (u16, u16) {
    let mut to_check = (pos.0 as i16 + delta.0, pos.1 as i16 + delta.1);

    if to_check.0 < 0 {
        to_check.0 = LEVEL_X_MAX as i16 - 1;
    } else if to_check.0 >= LEVEL_X_MAX as i16 {
        to_check.0 = 0;
    }

    if to_check.1 < 0 {
        to_check.1 = LEVEL_Y_MAX as i16 - 1;
    } else if to_check.1 >= LEVEL_Y_MAX as i16 {
        to_check.1 = 0;
    }

    (to_check.0 as u16, to_check.1 as u16)
}

#[allow(dead_code)]
#[derive(Debug)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn to_delta(&self) -> (i16, i16) {
        match self {
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Up => (0, 1),
            Dir::Down => (0, -1),
        }
    }
}