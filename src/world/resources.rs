use bevy::prelude::*;
use noise::Perlin;

const WORLD_SEED: u32 = 10;

#[derive(Resource)]
pub struct World {
    pub perlin: Perlin,
}

impl Default for World {
    fn default() -> World {
        World {
            perlin: Perlin::new(WORLD_SEED),
        }
    }
}
