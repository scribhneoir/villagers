use bevy::prelude::*;

const RENDER_SCALE: f32 = 2.0;
const BLOCK_TEXTURE_SIZE: f32 = 16.0;
const CHUNK_SIZE: usize = 16; //blocks^3
const WORLD_SIZE: usize = 9; //chunks^2

// const GRASS =

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, startup)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Block;

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
) {
    //spawn 2d camera
    commands.spawn((Camera2dBundle::default(), MainCamera));

    //generate chunk
    commands.spawn((Chunk {
        wx: 0,
        wy: 0,
        blocks: [[[3; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        inactive_render: [[3; CHUNK_SIZE]; CHUNK_SIZE],
        active: true,
        dirty: false,
    },));

    //add block texture to texture_atlases resource
    let texture_handle = asset_server.load("blocks.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 2, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    //spawn blocks for chunk
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                commands.spawn((
                    Block,
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        sprite: TextureAtlasSprite::new(3),
                        transform: Transform::from_translation(Vec3::new(
                            (x as f32 * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 2.0)
                                - (y as f32 * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 2.0) as f32,
                            -(y as f32 * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 3.0)
                                - (x as f32 * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 3.0)
                                + (z as f32 * BLOCK_TEXTURE_SIZE * RENDER_SCALE / (7.0 / 3.0))
                                    as f32,
                            1.0,
                        ))
                        .with_scale(Vec3::splat(RENDER_SCALE)),
                        ..Default::default()
                    },
                ));
            }
        }
    }
}
