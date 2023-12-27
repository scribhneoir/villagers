use bevy::prelude::*;

use noise::{NoiseFn, Perlin};

const WORLD_SEED: u32 = 3;

const RENDER_SCALE: f32 = 2.0;
const BLOCK_TEXTURE_SIZE: f32 = 16.0;
const CHUNK_SIZE: usize = 16; //blocks^3

// const WORLD_SIZE: usize = 9; //chunks^2

// const AIR = 0;
// const DIRT = 1;
// const GRASS = 2;
// const STONE = 3;

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
struct Chunk;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    //spawn 2d camera
    commands.spawn((Camera2dBundle::default(), MainCamera));

    //generate chunk
    commands.spawn((Chunk,));

    //add block texture to texture_atlases resource
    let texture_handle = asset_server.load("blocks.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::splat(BLOCK_TEXTURE_SIZE),
        2,
        2,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    //generate perlin noise
    let perlin = Perlin::new(WORLD_SEED);

    //spawn blocks for chunk
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            const SAMPLE_SCALE: f64 = 0.01;
            let height_noise = perlin
                .get([x as f64 * SAMPLE_SCALE, y as f64 * SAMPLE_SCALE])
                .abs();

            let stone_h = (height_noise * CHUNK_SIZE as f64) as usize;
            let dirt_h = CHUNK_SIZE - stone_h - 1;

            for z in 0..stone_h {
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

            for z in stone_h..dirt_h {
                commands.spawn((
                    Block,
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        sprite: TextureAtlasSprite::new(2),
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
