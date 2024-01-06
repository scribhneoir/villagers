use bevy::prelude::*;

use crate::chunk::block::components::*;
use crate::physics::*;

const VILLAGER_TEXTURE_W: f32 = 15.0;
const VILLAGER_TEXTURE_H: f32 = 20.0;
const VILLAGER_X_OFFSET: f32 = 0.0;
const VILLAGER_Y_OFFSET: f32 = 13.0;
const INITIAL_VILLAGER_COUNT: usize = 1;
pub struct VillagerPlugin;
impl Plugin for VillagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_villagers)
            .add_systems(Update, (move_villager));
    }
}

#[derive(Component)]
pub struct Villager;

fn spawn_villagers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("villager.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(VILLAGER_TEXTURE_W, VILLAGER_TEXTURE_H),
        24,
        20,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for _ in 0..INITIAL_VILLAGER_COUNT {
        commands.spawn((
            Villager,
            Position { x: 4, y: 4, z: 4 },
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(1),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..Default::default()
            },
        ));
    }
}

fn move_villager(
    input: Res<Input<KeyCode>>,
    mut villager_position_query: Query<&mut Position, With<Villager>>,
    mut villager_transform_query: Query<&mut Transform, With<Villager>>,
    mut villager_sprite_query: Query<&mut TextureAtlasSprite, With<Villager>>,
) {
    let mut transform = villager_transform_query.single_mut();
    let mut position = villager_position_query.single_mut();
    let mut sprite = villager_sprite_query.single_mut();

    let mut change = false;
    //move camera using wasd
    if input.pressed(KeyCode::Up) {
        position.y -= 1;
        sprite.index = 0;
        change = true;
    }
    if input.pressed(KeyCode::Down) {
        position.y += 1;
        sprite.index = 2;
        change = true;
    }
    if input.pressed(KeyCode::Left) {
        position.x -= 1;
        sprite.index = 3;
        change = true;
    }
    if input.pressed(KeyCode::Right) {
        position.x += 1;
        sprite.index = 1;
        change = true;
    }
    if input.pressed(KeyCode::Space) {
        position.z += 1;
        change = true;
    }
    if input.pressed(KeyCode::ShiftLeft) {
        position.z -= 1;
        change = true;
    }

    if change {
        transform.translation.x = (position.x as f32 * BLOCK_TEXTURE_SIZE / 2.0)
            - (position.y as f32 * BLOCK_TEXTURE_SIZE / 2.0)
            + VILLAGER_X_OFFSET;
        transform.translation.y = -(position.y as f32 * BLOCK_TEXTURE_SIZE / 4.0)
            - (position.x as f32 * BLOCK_TEXTURE_SIZE / 4.0)
            + (position.z as f32 * BLOCK_TEXTURE_SIZE / 2.0)
            + VILLAGER_Y_OFFSET;
        transform.translation.z = (position.x + position.y + position.z + 1) as f32;
    }
}
