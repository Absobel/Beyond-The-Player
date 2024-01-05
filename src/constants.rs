use bevy::prelude::*;

pub const LEVEL_X_MAX: u16 = 10;
pub const LEVEL_Y_MAX: u16 = 10;
pub const GRID_SQUARE_SIZE: u16 = 32;

pub const BACKGROUND_COLOR: Color = Color::WHITE;

pub const fn level_coords_to_pxl_coords(x: u16, y: u16) -> (f32, f32) {
    ((x * GRID_SQUARE_SIZE) as f32, (y * GRID_SQUARE_SIZE) as f32)
}
