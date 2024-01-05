use bevy::prelude::*;
pub mod resources;

use resources::World;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<World>();
    }
}
