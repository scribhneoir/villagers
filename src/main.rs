use bevy::prelude::*;

const CHUNK_SIZE: usize = 16;

// const GRASS = 

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest(),
        ))
        .add_systems(Startup, startup)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Chunk {
    wx: i32,
    wy: i32,
    blocks: [[[u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    inactive_render: [[u32; CHUNK_SIZE]; CHUNK_SIZE],
    active: bool,
    dirty: bool,
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,

){
    commands.spawn((Camera2dBundle::default(),MainCamera));

    //add block texture to texture_atlases resource
    let texture_handle = asset_server.load("blocks.png");
    let texture_atlas =
    TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 2, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    //spawn a block
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(3),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
    ));
}