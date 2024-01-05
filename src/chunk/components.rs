use bevy::prelude::*;

pub const WORLD_SIZE: usize = 8; //chunks^2
pub const CHUNK_SIZE: usize = 20; //blocks^3

#[derive(Component)]
pub struct Chunk {
    pub blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}
