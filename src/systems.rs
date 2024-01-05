use bevy::render::mesh;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::sprite::Mesh2dHandle;
use bevy::utils::{HashMap, HashSet};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use crate::utils::*;

/// SYSTEM SETS

#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Clone)]
pub struct MoveSystems;

#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Clone)]
pub struct ProcessSystems;

#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Clone)]
pub struct Display;

/// SYSTEMS UTILS

pub fn spawn_good_level_wall(commands: &mut Commands, position: Position) {
    commands.spawn(GoodLevelWallBundle {
        good_level_wall: GoodLevelWall,
        movable: Movable,
        position,
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
}

pub fn spawn_bad_level_wall(commands: &mut Commands, position: Position) {
    commands.spawn(BadLevelWallBundle {
        bad_level_wall: BadLevelWall,
        immovable: Immovable,
        position,
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
}

pub fn spawn_box(commands: &mut Commands, position: Position) {
    commands.spawn(BoxBundle {
        box_block: BoxBlock,
        movable: Movable,
        position,
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
}

pub fn funnel_mesh_vertices(direction: &Dir) -> Vec<[f32; 3]> {
    let grid_size = GRID_SQUARE_SIZE as f32;
    let semi_grid_size = grid_size / 2.0;
    match direction {
        Dir::Left => vec![
            [grid_size, 0.0, 0.0],       // bottom right
            [grid_size, grid_size, 0.0], // top right
            [0.0, semi_grid_size, 0.0],  // middle left
        ],
        Dir::Right => vec![
            [0.0, 0.0, 0.0],                  // bottom left
            [0.0, grid_size, 0.0],            // top left
            [grid_size, semi_grid_size, 0.0], // middle right
        ],
        Dir::Up => vec![
            [0.0, 0.0, 0.0],                  // bottom left
            [grid_size, 0.0, 0.0],            // bottom right
            [semi_grid_size, grid_size, 0.0], // middle top
        ],
        Dir::Down => vec![
            [0.0, grid_size, 0.0],       // top left
            [grid_size, grid_size, 0.0], // top right
            [semi_grid_size, 0.0, 0.0],  // middle bottom
        ],
    }
}

pub fn spawn_funnel(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Position,
    direction: Orientation,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, funnel_mesh_vertices(&direction.0));
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 3]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; 3]);
    mesh.set_indices(Some(mesh::Indices::U32(vec![0, 1, 2])));
    let handle_mesh = meshes.add(mesh);
    let handle_color = materials.add(Color::GREEN.into());

    commands.spawn(FunnelBundle {
        funnel: Funnel,
        position,
        direction,
        mesh: MaterialMesh2dBundle {
            mesh: Mesh2dHandle(handle_mesh),
            material: handle_color,
            ..Default::default()
        },
    });
}

/// SYSTEMS

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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

    spawn_good_level_wall(&mut commands, good_level_wall_pos);
    spawn_good_level_wall(&mut commands, Position { x: 0, y: 0 });

    spawn_bad_level_wall(&mut commands, Position { x: 3, y: 6 });

    spawn_box(&mut commands, Position { x: 4, y: 6 });

    spawn_funnel(
        &mut commands,
        &mut meshes,
        &mut materials,
        Position { x: 5, y: 6 },
        Orientation(Dir::Down),
    );
    spawn_funnel(
        &mut commands,
        &mut meshes,
        &mut materials,
        Position { x: 5, y: 5 },
        Orientation(Dir::Right),
    );
    spawn_funnel(
        &mut commands,
        &mut meshes,
        &mut materials,
        Position { x: 3, y: 5 },
        Orientation(Dir::Up),
    );
}

pub fn update_display_position(mut query: Query<(&Position, &mut Transform, Option<&Sprite>)>) {
    for (position, mut transform, sprite) in query.iter_mut() {
        let pxl_coords = level_coords_to_pxl_coords(position.x, position.y);
        let offset = if sprite.is_some() {
            GRID_SQUARE_SIZE as f32 / 2.0
        } else {
            0.0
        };
        transform.translation.x = pxl_coords.0 + offset;
        transform.translation.y = pxl_coords.1 + offset;
    }
}

pub fn process_move(
    mut move_commands: ResMut<MoveCommands>,
    mut move_history: ResMut<MoveHistory>,
    mut movable_query: Query<(Entity, &mut Position), With<Movable>>,
    immovable_query: Query<&Position, (Without<Movable>, With<Immovable>)>,
) {
    for command in move_commands.commands.iter_mut() {
        command.keep_still_valid();

        let mut actual_move = ActualMove {
            entities: vec![],
            cause: command.cause,
            delta: command.delta,
        };

        let movables = {
            let mut tmp = HashMap::new();
            for (entity, position) in movable_query.iter_mut() {
                tmp.insert(position.to_tuple(), entity);
            }
            tmp
        };

        let mut to_move = HashSet::new();
        for (_, mut current_pos) in command.entities.iter() {
            let mut tmp_to_move = HashSet::new();
            while let Some(&entity) = movables.get(&current_pos) {
                tmp_to_move.insert(entity);
                current_pos = add_delta(current_pos, command.delta);
            }
            if immovable_query
                .iter()
                .any(|pos| pos.to_tuple() == current_pos)
            {
                continue;
            }
            to_move.extend(tmp_to_move);
        }
        for entity in to_move.iter() {
            actual_move.entities.push(*entity);
            let mut position = movable_query
                .get_mut(*entity)
                .expect("Entity that moves should be movable")
                .1;
            let new_pos = add_delta(position.to_tuple(), command.delta);
            position.x = new_pos.0;
            position.y = new_pos.1;
        }
        dbg!(&actual_move); // DEBUG
        if actual_move.cause == MoveCause::UserMove || move_history.moves.is_empty() {
            move_history.moves.push(vec![actual_move]);
        } else if let Some(last_move) = move_history.moves.last_mut() {
            last_move.push(actual_move);
        } else {
            unreachable!();
        }
    }
    move_commands.commands.clear();
}

pub fn undo(
    mut move_history: ResMut<MoveHistory>,
    keyboard_input: Res<Input<KeyCode>>,
    mut movable_query: Query<(Entity, &mut Position), With<Movable>>,
) {
    if keyboard_input.just_pressed(KeyCode::U) {
        if let Some(last_game_loop_move) = move_history.moves.pop() {
            for last_move in last_game_loop_move.iter().rev() {
                for entity in last_move.entities.iter() {
                    let mut position = movable_query
                        .get_mut(*entity)
                        .expect("Entity that moves should be movable")
                        .1;
                    let new_pos = add_delta(position.to_tuple(), last_move.undo_delta());
                    position.x = new_pos.0;
                    position.y = new_pos.1;
                }
            }
        }
    }
}

pub fn move_good_level_wall(
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(Entity, &Position), With<GoodLevelWall>>,
    mut move_commands: ResMut<MoveCommands>,
) {
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
        entities: query
            .iter()
            .map(|(entity, position)| (entity, position.to_tuple()))
            .collect(),
        cause: MoveCause::UserMove,
        delta,
    });
}

pub fn funnel_move(
    funnel_query: Query<(&Position, &Orientation), With<Funnel>>,
    movable_query: Query<(Entity, &Position), With<Movable>>,
    mut move_commands: ResMut<MoveCommands>,
) {
    for (funnel_pos, direction) in funnel_query.iter() {
        for (movable_entity, movable_pos) in movable_query.iter() {
            if funnel_pos.to_tuple() == movable_pos.to_tuple() {
                move_commands.commands.push(MoveCommand {
                    entities: vec![(movable_entity, movable_pos.to_tuple())],
                    cause: MoveCause::FunnelMove(funnel_pos.to_tuple()),
                    delta: direction.to_delta(),
                });
            }
        }
    }
}
