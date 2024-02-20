use bevy::prelude::*;
use noise::Perlin;

const WORLD_SEED: u32 = 10;
pub const WORLD_SIZE: usize = 8; //chunks^2

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<World>();
    }
}

#[derive(Resource)]
pub struct World {
    pub perlin: Perlin,
}

impl Default for World {
    fn default() -> Self {
        Self {
            perlin: Perlin::new(WORLD_SEED),
        }
    }
}

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component, Eq, PartialEq, Hash, Clone, Debug)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl GridPosition {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn to_grid(&self) -> GridPosition {
        GridPosition {
            x: (self.x.round()) as usize,
            y: (self.y.round()) as usize,
            z: (self.z.round()) as usize,
        }
    }
}
