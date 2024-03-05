use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct MouseState {
    pub position: Vec2,
    pub left_button_pressed: bool,
    pub right_button_pressed: bool,
}
