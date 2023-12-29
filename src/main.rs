use bevy::prelude::*;

use noise::{NoiseFn, Perlin};

const WORLD_SEED: u32 = 10;

const RENDER_SCALE: f32 = 0.5;
const BLOCK_TEXTURE_SIZE: f32 = 24.0;
const CHUNK_SIZE: usize = 44; //blocks^3
const SAMPLE_SCALE: f64 = 0.02;
const WATER_LEVEL: usize = 4;

// const WORLD_SIZE: usize = 9; //chunks^2

const DIRT: usize = 0;
const GRASS: usize = 1;
const STONE: usize = 2;
const SAND: usize = 3;
const LOG: usize = 4;
const LEAF: usize = 5;

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

fn check_surroundings(
    chunk: &[[[i32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    x: usize,
    y: usize,
    z: usize,
    radius: usize,
) -> bool {
    let x_min = if x > radius { x - radius } else { 0 };
    let y_min = if y > radius { y - radius } else { 0 };
    let z_min = if z > radius { z - radius } else { 0 };
    let x_max = if x + radius < CHUNK_SIZE {
        x + radius
    } else {
        CHUNK_SIZE
    };
    let y_max = if y + radius < CHUNK_SIZE {
        y + radius
    } else {
        CHUNK_SIZE
    };
    let z_max = if z + radius < CHUNK_SIZE {
        z + radius
    } else {
        CHUNK_SIZE
    };
    for i in x_min..x_max {
        for j in y_min..y_max {
            for k in z_min..z_max {
                if i > 0 && j > 0 && k > 0 && i < CHUNK_SIZE && j < CHUNK_SIZE && k < CHUNK_SIZE {
                    if chunk[i][j][k] == LOG as i32 || chunk[i][j][k] == LEAF as i32 {
                        return false;
                    }
                }
            }
        }
    }
    return true;
}

fn spawn_block(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    sprite_index: usize,
    x: usize,
    y: usize,
    z: usize,
) -> Entity {
    return commands
        .spawn((
            Block,
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(sprite_index),
                transform: Transform::from_translation(Vec3::new(
                    (x as f32 * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 2.0)
                        - (y as f32 * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 2.0) as f32,
                    -(y as f32 * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 4.0)
                        - (x as f32 * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 4.0)
                        + (z as f32 * BLOCK_TEXTURE_SIZE * RENDER_SCALE / (7.0 / 3.0)) as f32,
                    1.0,
                ))
                .with_scale(Vec3::splat(RENDER_SCALE)),
                ..Default::default()
            },
        ))
        .id();
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

    let mut chunk: [[[i32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] =
        [[[-1; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
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
                chunk[x][y][z] = STONE as i32;
                spawn_block(&mut commands, texture_atlas_handle.clone(), STONE, x, y, z);
            }

            for z in stone_h..dirt_h {
                chunk[x][y][z] = DIRT as i32;
                spawn_block(&mut commands, texture_atlas_handle.clone(), DIRT, x, y, z);
            }
            if dirt_h > WATER_LEVEL {
                chunk[x][y][dirt_h] = GRASS as i32;
                spawn_block(
                    &mut commands,
                    texture_atlas_handle.clone(),
                    GRASS,
                    x,
                    y,
                    dirt_h,
                );
                //spawn trees
                if perlin.get([x as f64 * SAMPLE_SCALE, y as f64 * SAMPLE_SCALE, 4.0]) > 0.2
                    && check_surroundings(
                        &chunk,
                        x,
                        y,
                        dirt_h + 5,
                        (perlin
                            .get([x as f64 * SAMPLE_SCALE, y as f64 * SAMPLE_SCALE, 4.0])
                            .abs()
                            * 5.0
                            + 3.0) as usize,
                    )
                {
                    //spawn trunk
                    for z in dirt_h..dirt_h + 5 {
                        chunk[x][y][z] = LOG as i32;
                        spawn_block(&mut commands, texture_atlas_handle.clone(), LOG, x, y, z);
                    }

                    //spawn leaves
                    let radius = 1;
                    let x_min = if x > radius { x - radius } else { 0 };
                    let y_min = if y > radius { y - radius } else { 0 };
                    let z_min = if dirt_h + 5 > radius {
                        dirt_h + 5 - radius
                    } else {
                        0
                    };
                    let x_max = if x + radius < CHUNK_SIZE {
                        x + radius
                    } else {
                        CHUNK_SIZE
                    };
                    let y_max = if y + radius < CHUNK_SIZE {
                        y + radius
                    } else {
                        CHUNK_SIZE
                    };
                    let z_max = if dirt_h + 5 + radius < CHUNK_SIZE {
                        dirt_h + 5 + radius
                    } else {
                        CHUNK_SIZE
                    };
                    for i in x_min..x_max {
                        for j in y_min..y_max {
                            for k in z_min..z_max {
                                {
                                    chunk[i][j][k] = LEAF as i32;
                                    spawn_block(
                                        &mut commands,
                                        texture_atlas_handle.clone(),
                                        LEAF,
                                        i,
                                        j,
                                        k,
                                    );
                                }
                            }
                        }
                    }
                }
            } else if dirt_h == WATER_LEVEL {
                chunk[x][y][dirt_h] = SAND as i32;
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
                    chunk[x][y][z] = WATER as i32;
                    spawn_block(&mut commands, texture_atlas_handle.clone(), WATER, x, y, z);
                }
                chunk[x][y][WATER_LEVEL] = WATER_TOP as i32;
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
