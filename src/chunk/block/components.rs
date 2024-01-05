use bevy::prelude::*;

pub const DIRT: usize = 1;
pub const GRASS: usize = 2;
pub const STONE: usize = 3;
pub const SAND: usize = 4;
pub const LOG: usize = 5;
pub const LEAF: usize = 6;
pub const WATER: usize = 24;
pub const WATER_TOP: usize = 25;

pub const BLOCK_TEXTURE_SIZE: f32 = 24.0;

#[derive(Component)]
pub struct Block;
