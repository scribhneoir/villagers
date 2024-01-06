use bevy::prelude::*;

mod systems;
mod resources;

use systems::*;
use resources::*;

pub struct MousePlugin;
impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseState>()
            .add_systems(Update, update_mouse_state);
    }
}
