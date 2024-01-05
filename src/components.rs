use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

// COMPONENTS UTILS

#[allow(dead_code)]
#[derive(Debug)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    pub fn to_delta(&self) -> (i16, i16) {
        match self {
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Up => (0, 1),
            Dir::Down => (0, -1),
        }
    }
}

pub type PositionTuple = (u16, u16);

/// MARKER COMPONENTS

#[derive(Component)]
pub struct Movable;

#[derive(Component)]
pub struct Immovable;

#[derive(Component)]
pub struct GoodLevelWall;

#[derive(Component)]
pub struct BadLevelWall;

#[derive(Component)]
pub struct BoxBlock;

#[derive(Component)]
pub struct Funnel;

/// OTHER COMPONENTS

#[derive(Component, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn to_tuple(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}

#[derive(Component, Debug)]
pub struct Orientation(pub Dir);

impl Orientation {
    pub fn to_delta(&self) -> (i16, i16) {
        self.0.to_delta()
    }
}

/// BUNDLES

#[derive(Bundle)]
pub struct GoodLevelWallBundle {
    pub good_level_wall: GoodLevelWall,
    pub movable: Movable,
    pub position: Position,
    pub sprite: SpriteBundle,
}

#[derive(Bundle)]
pub struct BadLevelWallBundle {
    pub bad_level_wall: BadLevelWall,
    pub immovable: Immovable,
    pub position: Position,
    pub sprite: SpriteBundle,
}

#[derive(Bundle)]
pub struct BoxBundle {
    pub box_block: BoxBlock,
    pub movable: Movable,
    pub position: Position,
    pub sprite: SpriteBundle,
}

#[derive(Bundle)]
pub struct FunnelBundle {
    pub funnel: Funnel,
    pub position: Position,
    pub direction: Orientation,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}
