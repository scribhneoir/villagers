use bevy::prelude::*;

// pub struct PhysicsPlugin;
// impl Plugin for PhysicsPlugin {
//     fn build(&self, app: &mut App) {
//         // app.add_systems(Update, apply_gravity);
//     }
// }

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// fn apply_gravity(
//     mut commands: Commands,
//     mut query: Query<(&mut Position, &mut Velocity), With<Gravity>>,
// ) {
// }
