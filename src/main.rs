use bevy::prelude::*;

use noise::{NoiseFn, Perlin};

const WORLD_SEED: u32 = 187756;

const RENDER_SCALE: f32 = 2.0;
const BLOCK_TEXTURE_SIZE: f32 = 24.0;
const CHUNK_SIZE: usize = 16; //blocks^3
const SAMPLE_SCALE: f64 = 0.06;
const WATER_LEVEL: usize = 4;

// const WORLD_SIZE: usize = 9; //chunks^2

const DIRT: usize = 0;
const GRASS: usize = 1;
const STONE: usize = 2;
const SAND: usize = 3;
// const LOG: usize = 4;
// const LEAF: usize = 5;

const WATER: usize = 24;
const WATER_TOP: usize = 25;

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

fn spawn_block(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    sprite_index: usize,
    x: usize,
    y: usize,
    z: usize,
) {
    commands.spawn((
        Block,
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(sprite_index),
            transform: Transform::from_translation(Vec3::new(
                (x as f32 * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 2.0)
                    - (y as f32 * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 2.0) as f32,
                -(y as f32 * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 4.0)
                    - (x as f32 * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 4.0)
                    + (z as f32 * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 3.0) as f32,
                1.0,
            ))
            .with_scale(Vec3::splat(RENDER_SCALE)),
            ..Default::default()
        },
    ));
}

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
        24,
        24,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    //generate perlin noise
    let perlin = Perlin::new(WORLD_SEED);

    //spawn blocks for chunk
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let stone_h = ((perlin.get([x as f64 * SAMPLE_SCALE, y as f64 * SAMPLE_SCALE, 0.0])
                * (CHUNK_SIZE as f64))
                * 1.2
                + 1.0) as usize;
            let dirt_h = ((stone_h as f64
                + (perlin.get([x as f64 * SAMPLE_SCALE, y as f64 * SAMPLE_SCALE, 2.0]) * 5.0))
                * 1.2
                + 1.0) as usize;

            for z in 0..stone_h {
                spawn_block(&mut commands, texture_atlas_handle.clone(), STONE, x, y, z);
            }

            for z in stone_h..dirt_h {
                spawn_block(&mut commands, texture_atlas_handle.clone(), DIRT, x, y, z);
            }
            if dirt_h > WATER_LEVEL {
                spawn_block(
                    &mut commands,
                    texture_atlas_handle.clone(),
                    GRASS,
                    x,
                    y,
                    dirt_h,
                );
            } else if dirt_h == WATER_LEVEL {
                spawn_block(
                    &mut commands,
                    texture_atlas_handle.clone(),
                    SAND,
                    x,
                    y,
                    dirt_h,
                );
            } else {
                for z in dirt_h..WATER_LEVEL {
                    spawn_block(&mut commands, texture_atlas_handle.clone(), WATER, x, y, z);
                }
                spawn_block(
                    &mut commands,
                    texture_atlas_handle.clone(),
                    WATER_TOP,
                    x,
                    y,
                    WATER_LEVEL,
                );
            }
        }
    }
}
