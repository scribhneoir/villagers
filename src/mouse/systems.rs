use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::camera::MainCamera;

use super::resources::*;

pub fn update_mouse_state(
    mut mouse_state: ResMut<MouseState>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor_pos| camera.viewport_to_world_2d(camera_transform, cursor_pos))
    {
        // Mouse is in the window.
        mouse_state.position = world_position;
        mouse_state.left_button_pressed = mouse_buttons.pressed(MouseButton::Left);
        mouse_state.right_button_pressed = mouse_buttons.pressed(MouseButton::Right);
    }
}
