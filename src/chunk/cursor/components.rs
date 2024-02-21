use bevy::prelude::*;

pub const CURSOR_FULL: usize = 0;
pub const CURSOR_TOP: usize = 1;
pub const CURSOR_LEFT: usize = 2;
pub const CURSOR_RIGHT: usize = 3;

pub const CURSOR_TEXTURE_SIZE: f32 = 24.0;

#[derive(Component)]
pub struct Cursor;
