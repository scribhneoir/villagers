//linting rules
#![warn(
    clippy::unwrap_used,
    clippy::perf,
    clippy::nursery,
    clippy::expect_used,
    clippy::complexity
)]

//modules
mod camera;
pub mod chunk;
mod fps;
mod mouse;
pub mod pathfinding;
mod physics;
mod villager;
mod world;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            world::WorldPlugin,
            chunk::ChunkPlugin,
            villager::VillagerPlugin,
            mouse::MousePlugin,
            fps::FpsPlugin,
            camera::CameraPlugin,
        ))
        .run();
}
