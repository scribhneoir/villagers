use bevy::prelude::*;

pub mod block;

mod components;
mod helpers;
mod systems;

use systems::*;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_chunks);
    }
}
