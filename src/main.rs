use bevy::prelude::*;

use noise::{NoiseFn, Perlin};

const WORLD_SEED: u32 = 10;

const RENDER_SCALE: f32 = 1.0;
const BLOCK_TEXTURE_SIZE: f32 = 24.0;
const CHUNK_SIZE: usize = 20; //blocks^3
const SAMPLE_SCALE: f64 = 0.01;
const WATER_LEVEL: usize = 4;
const TREE_HEIGHT: usize = 5;

const WORLD_SIZE: usize = 8; //chunks^2

const DIRT: i32 = 0;
const GRASS: i32 = 1;
const STONE: i32 = 2;
const SAND: i32 = 3;
const LOG: i32 = 4;
const LEAF: i32 = 5;

const WATER: i32 = 24;
const WATER_TOP: i32 = 25;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, startup)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct World {
    perlin: Perlin,
}

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Chunk {
    blocks: [[[i32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

#[derive(Component)]
struct ChunkPosition {
    x: usize,
    y: usize,
    z: usize,
    active: bool,
}

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

fn generate_chunk(
    perlin: &Perlin,
    x_off: f64,
    y_off: f64,
) -> [[[i32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] {
    let mut chunk: [[[i32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] =
        [[[-1; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let stone_h = ((perlin.get([
                (x as f64 + x_off) * SAMPLE_SCALE,
                (y as f64 + y_off) * SAMPLE_SCALE,
                0.0,
            ]) * (CHUNK_SIZE as f64))
                * 1.2
                + 1.0) as usize;
            let dirt_h = ((stone_h as f64
                + (perlin.get([
                    (x as f64 + x_off) * SAMPLE_SCALE,
                    (y as f64 + y_off) * SAMPLE_SCALE,
                    2.0,
                ]) * 5.0))
                * 1.2
                + 1.0) as usize;

            for z in 0..stone_h {
                chunk[x][y][z] = STONE;
            }

            for z in stone_h..dirt_h {
                chunk[x][y][z] = DIRT;
            }
            if dirt_h > WATER_LEVEL + 1 {
                chunk[x][y][dirt_h] = GRASS;
                //spawn trees
                if x > 2
                    && y > 2
                    && x < CHUNK_SIZE - 2
                    && y < CHUNK_SIZE - 2
                    && perlin.get([
                        (x as f64 + x_off) * SAMPLE_SCALE,
                        (y as f64 + y_off) * SAMPLE_SCALE,
                        4.0,
                    ]) > 0.2
                    && check_surroundings(
                        &chunk,
                        x,
                        y,
                        dirt_h + 5,
                        (perlin
                            .get([
                                (x as f64 + x_off) * SAMPLE_SCALE,
                                (y as f64 + y_off) * SAMPLE_SCALE,
                                4.0,
                            ])
                            .abs()
                            * 5.0
                            + 3.0) as usize,
                    )
                {
                    //spawn trunk
                    for z in dirt_h..dirt_h + TREE_HEIGHT {
                        chunk[x][y][z] = LOG;
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
                                    chunk[i][j][k] = LEAF;
                                }
                            }
                        }
                    }
                }
            } else if dirt_h == WATER_LEVEL || dirt_h == WATER_LEVEL + 1 {
                chunk[x][y][dirt_h] = SAND;
            } else {
                for z in dirt_h..WATER_LEVEL {
                    chunk[x][y][z] = WATER;
                }
                chunk[x][y][WATER_LEVEL] = WATER_TOP;
            }
        }
    }

    //spawn chunk entity
    return chunk;
}

fn find_top_block(
    chunk: &[[[i32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    x: usize,
    y: usize,
    z: usize,
) -> (i32, usize, usize, usize) {
    let min_coor = x.min(y).min(z);
    for i in 0..min_coor {
        if chunk[x - i][y - i][z - i] != -1 {
            return (chunk[x - i][y - i][z - i], x - i, y - i, z - i);
        }
    }
    return (-1, 0, 0, 0);
}

fn spawn_visable_blocks(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    chunk: &[[[i32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    chunk_ent: Entity,
    x_off: f32,
    y_off: f32,
    z_off: f32,
) {
    for i in 0..CHUNK_SIZE {
        for j in 0..CHUNK_SIZE {
            let mut top_block_id = find_top_block(&chunk, i, j, CHUNK_SIZE - 1);
            if top_block_id.0 > -1 {
                let block_ent = spawn_block(
                    commands,
                    texture_atlas_handle.clone(),
                    top_block_id.0,
                    top_block_id.1 as f32 + x_off,
                    top_block_id.2 as f32 + y_off,
                    top_block_id.3 as f32 + z_off,
                );
                commands.entity(chunk_ent).push_children(&[block_ent]);
            }
            top_block_id = find_top_block(&chunk, i, CHUNK_SIZE - 1, j);
            if top_block_id.0 > -1 {
                let block_ent = spawn_block(
                    commands,
                    texture_atlas_handle.clone(),
                    top_block_id.0,
                    top_block_id.1 as f32 + x_off,
                    top_block_id.2 as f32 + y_off,
                    top_block_id.3 as f32 + z_off,
                );
                commands.entity(chunk_ent).push_children(&[block_ent]);
            }
            top_block_id = find_top_block(&chunk, CHUNK_SIZE - 1, i, j);
            if top_block_id.0 > -1 {
                let block_ent = spawn_block(
                    commands,
                    texture_atlas_handle.clone(),
                    top_block_id.0,
                    top_block_id.1 as f32 + x_off,
                    top_block_id.2 as f32 + y_off,
                    top_block_id.3 as f32 + z_off,
                );
                commands.entity(chunk_ent).push_children(&[block_ent]);
            }
        }
    }
}

fn spawn_block(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    sprite_index: i32,
    x: f32,
    y: f32,
    z: f32,
) -> Entity {
    return commands
        .spawn((
            Block,
            // ChunkPosition { x, y, z },
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(sprite_index as usize),
                transform: Transform::from_translation(Vec3::new(
                    (x * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 2.0)
                        - (y * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 2.0)
                        - 1.0,
                    -(y * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 4.0)
                        - (x * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 4.0)
                        + (z * BLOCK_TEXTURE_SIZE * RENDER_SCALE / (7.0 / 3.0))
                        - 1.0,
                    x + y + z,
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

    //spawn 2d camera
    commands.spawn((Camera2dBundle::default(), MainCamera));

    //generate perlin noise
    let perlin = Perlin::new(WORLD_SEED);

    //add world to resources
    commands.insert_resource(World { perlin });

    //generate chunks
    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            let x_off = x as f32 * (CHUNK_SIZE as f32 - 1.0);
            let y_off = y as f32 * (CHUNK_SIZE as f32 - 1.0);

            let chunk = generate_chunk(&perlin, x_off as f64, y_off as f64);

            let chunk_ent = commands
                .spawn((Chunk { blocks: chunk }, SpatialBundle::default()))
                .id();

            spawn_visable_blocks(
                &mut commands,
                texture_atlas_handle.clone(),
                &chunk,
                chunk_ent,
                x_off,
                y_off,
                0.0,
            );
        }
    }
}
