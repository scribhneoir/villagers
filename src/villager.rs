use bevy::prelude::*;

use crate::chunk::BLOCK_TEXTURE_SIZE;
use crate::physics::Position;

const VILLAGER_TEXTURE_W: f32 = 15.0;
const VILLAGER_TEXTURE_H: f32 = 20.0;
const VILLAGER_X_OFFSET: f32 = 0.0;
const VILLAGER_Y_OFFSET: f32 = 13.0;
const VILLAGER_SPEED: f32 = 2.0;
const INITIAL_VILLAGER_COUNT: usize = 1;

pub struct VillagerPlugin;
impl Plugin for VillagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_villagers)
            .add_systems(Update, move_villager);
    }
}

#[derive(Component)]
pub struct Villager {
    pub selected: bool,
}

fn spawn_villagers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("villager.png");
    let texture_atlas = TextureAtlasLayout::from_grid(
        Vec2::new(VILLAGER_TEXTURE_W, VILLAGER_TEXTURE_H),
        24,
        20,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for _ in 0..INITIAL_VILLAGER_COUNT {
        commands.spawn((
            Villager { selected: false },
            Position {
                x: 5.0,
                y: 5.0,
                z: 5.0,
            },
            SpriteSheetBundle {
                atlas: TextureAtlas {
                    layout: texture_atlas_handle.clone(),
                    index: 1,
                },
                sprite: Sprite::default(),
                texture: texture_handle.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..Default::default()
            },
        ));
    }
}

fn move_villager(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut villager_position_query: Query<&mut Position, With<Villager>>,
    mut villager_transform_query: Query<&mut Transform, With<Villager>>,
    mut villager_sprite_query: Query<&mut TextureAtlas, With<Villager>>,
) {
    let mut transform = villager_transform_query.single_mut();
    let mut position = villager_position_query.single_mut();
    let mut sprite = villager_sprite_query.single_mut();

    let mut change = false;
    //move camera using wasd
    if input.pressed(KeyCode::ArrowUp) {
        position.y -= VILLAGER_SPEED * time.delta_seconds();
        sprite.index = 0;
        change = true;
    }
    if input.pressed(KeyCode::ArrowDown) {
        position.y += VILLAGER_SPEED * time.delta_seconds();
        sprite.index = 2;
        change = true;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        position.x -= VILLAGER_SPEED * time.delta_seconds();
        sprite.index = 3;
        change = true;
    }
    if input.pressed(KeyCode::ArrowRight) {
        position.x += VILLAGER_SPEED * time.delta_seconds();
        sprite.index = 1;
        change = true;
    }
    if input.pressed(KeyCode::Space) {
        position.z += VILLAGER_SPEED * time.delta_seconds();
        change = true;
    }
    if input.pressed(KeyCode::ShiftLeft) {
        position.z -= VILLAGER_SPEED * time.delta_seconds();
        change = true;
    }

    if change {
        transform.translation.x = (position.x.round() * BLOCK_TEXTURE_SIZE / 2.0)
            - (position.y.round() * BLOCK_TEXTURE_SIZE / 2.0)
            + VILLAGER_X_OFFSET;
        transform.translation.y = -(position.y.round() * BLOCK_TEXTURE_SIZE / 4.0)
            - (position.x.round() * BLOCK_TEXTURE_SIZE / 4.0)
            + (position.z.round() * BLOCK_TEXTURE_SIZE / 2.0)
            + VILLAGER_Y_OFFSET;
        transform.translation.z =
            position.x.round() + position.y.round() + position.z.round() + 1.0;
    }
}
