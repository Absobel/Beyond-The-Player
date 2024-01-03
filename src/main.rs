use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .configure_sets(Update, (MoveSystems, ProcessSystems, Display))
        /////////
        // Setup
        .add_systems(Startup, setup)
        // Player Move
        .add_systems(Update, move_good_level_wall)
        // Set commands
        .add_systems(
            Update,
            funnel_move
                .in_set(MoveSystems)
                .after(move_good_level_wall)
        )
        // Process commands
        .add_systems(
            Update,
            (process_move, undo)
                .in_set(ProcessSystems)
                .after(MoveSystems),
        )
        // Display
        .add_systems(
            Update,
            update_display_position
                .in_set(Display)
                .after(ProcessSystems),
        )
        /////////
        .insert_resource(MoveCommands { commands: vec![] })
        .insert_resource(MoveHistory { moves: vec![] })
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

/// SYSTEM SETS

#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Clone)]
struct MoveSystems;

#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Clone)]
struct ProcessSystems;

#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Clone)]
struct Display;

/// CONDITIONS

// Nothing for now...

/// RESOURCES UTILS

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MoveCause {
    UserMove,
    FunnelMove(PositionTuple),
}

struct MoveCommand {
    entities: Vec<(Entity, PositionTuple)>,
    cause: MoveCause,
    delta: (i16, i16),
}

impl MoveCommand {
    fn keep_still_valid(&mut self) {
        match self.cause {
            MoveCause::UserMove => {}
            MoveCause::FunnelMove(funnel_pos) => {
                self.entities.retain(|(_, pos)| *pos == funnel_pos);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ActualMove {
    entities: Vec<Entity>,
    cause: MoveCause,
    delta: (i16, i16),
}

impl ActualMove {
    fn undo_delta(&self) -> (i16, i16) {
        (-self.delta.0, -self.delta.1)
    }
}

/// RESOURCES

#[derive(Resource)]
struct MoveCommands {
    commands: Vec<MoveCommand>,
}

#[derive(Resource)]
struct MoveHistory {
    moves: Vec<Vec<ActualMove>>, // Vec<AcutalMove> is all the moves in a single frame
}

impl MoveHistory {
    fn last_actual_move(&self) -> Option<&ActualMove> {
        self.moves
            .last()
            .and_then(|last_game_loop_move| last_game_loop_move.last())
    }

    fn last_move_cause(&self) -> Option<MoveCause> {
        self.last_actual_move()
            .map(|last_actual_move| last_actual_move.cause)
    }
}

// COMPONENTS UTILS

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

type PositionTuple = (u16, u16);

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
struct Funnel;

/// OTHER COMPONENTS

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
struct FunnelBundle {
    funnel: Funnel,
    position: Position,
    direction: Direction,
    sprite: SpriteBundle,
}

/// SYSTEMS UTILS

fn spawn_good_level_wall(commands: &mut Commands, position: Position) {
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

fn spawn_bad_level_wall(commands: &mut Commands, position: Position) {
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

fn spawn_box(commands: &mut Commands, position: Position) {
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

fn spawn_funnel(commands: &mut Commands, position: Position, direction: Direction) {
    commands.spawn(FunnelBundle {
        funnel: Funnel,
        position,
        direction,
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

    spawn_good_level_wall(&mut commands, good_level_wall_pos);
    spawn_good_level_wall(&mut commands, Position { x: 0, y: 0 });

    spawn_bad_level_wall(&mut commands, Position { x: 3, y: 6 });

    spawn_box(&mut commands, Position { x: 4, y: 6 });

    spawn_funnel(&mut commands, Position { x: 5, y: 6 }, Direction(Dir::Down));
    spawn_funnel(
        &mut commands,
        Position { x: 5, y: 5 },
        Direction(Dir::Right),
    );
    spawn_funnel(&mut commands, Position { x: 3, y: 5 }, Direction(Dir::Up));
}

fn update_display_position(mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in query.iter_mut() {
        let pxl_coords = level_coords_to_pxl_coords(position.x, position.y);
        transform.translation.x = pxl_coords.0 + GRID_SQUARE_SIZE as f32 / 2.0;
        transform.translation.y = pxl_coords.1 + GRID_SQUARE_SIZE as f32 / 2.0;
    }
}

fn process_move(
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

fn undo(
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

fn move_good_level_wall(
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

fn funnel_move(
    funnel_query: Query<(&Position, &Direction), With<Funnel>>,
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
