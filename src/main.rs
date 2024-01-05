mod chunk;
mod mouse;
mod world;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(world::WorldPlugin)
        .add_plugins(chunk::ChunkPlugin)
        .add_plugins(mouse::MousePlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, move_camera)
        .run();
}

#[derive(Component)]
pub struct MainCamera;

pub fn startup(mut commands: Commands) {
    //spawn 2d camera
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn move_camera(
    input: Res<Input<KeyCode>>,
    mut camera_projection_query: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut camera_transform_query: Query<&mut Transform, With<Camera>>,
) {
    let mut projection = camera_projection_query.single_mut();
    let mut transform = camera_transform_query.single_mut();

    if input.pressed(KeyCode::Minus) {
        projection.scale += 0.05;
    }

    if input.pressed(KeyCode::Equals) {
        projection.scale -= 0.05;
    }

    //move camera using wasd
    if input.pressed(KeyCode::W) {
        transform.translation.y += 1.0 * projection.scale;
    }
    if input.pressed(KeyCode::S) {
        transform.translation.y -= 1.0 * projection.scale;
    }
    if input.pressed(KeyCode::A) {
        transform.translation.x -= 1.0 * projection.scale;
    }
    if input.pressed(KeyCode::D) {
        transform.translation.x += 1.0 * projection.scale;
    }

    projection.scale = projection.scale.clamp(0.2, 5.);
}
