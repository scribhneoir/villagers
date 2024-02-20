use ::bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, move_camera);
    }
}

#[derive(Component)]
pub struct MainCamera;

pub fn startup(mut commands: Commands) {
    //spawn 2d camera
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn move_camera(
    input: Res<ButtonInput<KeyCode>>,
    mut camera_projection_query: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut camera_transform_query: Query<&mut Transform, With<Camera>>,
) {
    let mut projection = camera_projection_query.single_mut();
    let mut transform = camera_transform_query.single_mut();

    if input.pressed(KeyCode::Minus) {
        projection.scale += 0.05;
    }
    if input.pressed(KeyCode::Equal) {
        projection.scale -= 0.05;
    }
    //move camera using wasd
    if input.pressed(KeyCode::KeyW) {
        transform.translation.y += 1.0 * projection.scale;
    }
    if input.pressed(KeyCode::KeyS) {
        transform.translation.y -= 1.0 * projection.scale;
    }
    if input.pressed(KeyCode::KeyA) {
        transform.translation.x -= 1.0 * projection.scale;
    }
    if input.pressed(KeyCode::KeyD) {
        transform.translation.x += 1.0 * projection.scale;
    }

    projection.scale = projection.scale.clamp(0.2, 5.);
}
