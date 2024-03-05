use crate::mouse::MouseState;
use crate::world::{Position, World, WORLD_SIZE};
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

mod cursor;
use cursor::components::*;

//Const
pub const CHUNK_SIZE: usize = 20; //blocks^3
pub const BLOCK_TEXTURE_SIZE: f32 = 24.0;
const RENDER_SCALE: f32 = 1.0;
pub const SAMPLE_SCALE: f64 = 0.01;
pub const WATER_LEVEL: usize = 4;
pub const TREE_HEIGHT: usize = 5;

//Types
#[repr(usize)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Block {
    Air = 0,
    Dirt = 1,
    Grass = 2,
    Stone = 3,
    Sand = 4,
    Log = 5,
    Leaf = 6,
    Water = 24,
    WaterTop = 25,
}

//Plugin
pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_chunks, spawn_cursor))
            .add_systems(Update, move_cursor);
    }
}

//Components
#[derive(Component, Clone)]
pub struct Chunk {
    pub blocks: [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            blocks: [[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct RenderedBlock;

//Systems
pub fn spawn_chunks(
    mut commands: Commands,
    world: Res<World>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    //add block texture to texture_atlases resource
    let block_texture_handle = asset_server.load("blocks.png");
    let texture_atlas = TextureAtlasLayout::from_grid(
        Vec2::new(BLOCK_TEXTURE_SIZE, BLOCK_TEXTURE_SIZE + 1.0),
        24,
        20,
        None,
        None,
    );
    let block_layout_handle = texture_atlases.add(texture_atlas);

    let perlin = world.perlin;
    //generate chunks
    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            let x_off = x as f32 * (CHUNK_SIZE as f32 - 1.0);
            let y_off = y as f32 * (CHUNK_SIZE as f32 - 1.0);

            let chunk = generate_chunk(&perlin, f64::from(x_off), f64::from(y_off));

            let chunk_ent = commands
                .spawn((
                    Chunk { blocks: chunk },
                    Position {
                        x: x_off,
                        y: y_off,
                        z: 0.0,
                    },
                    SpatialBundle::default(),
                ))
                .id();

            spawn_visable_blocks(
                &mut commands,
                block_layout_handle.clone(),
                block_texture_handle.clone(),
                &chunk,
                chunk_ent,
                x_off,
                y_off,
                0.0,
            );
        }
    }
}

pub fn check_surroundings(
    chunk: &[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
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
    for (i, _) in chunk.iter().enumerate().take(x_max).skip(x_min) {
        for (j, _) in chunk.iter().enumerate().take(y_max).skip(y_min) {
            for (k, _) in chunk.iter().enumerate().take(z_max).skip(z_min) {
                if i > 0
                    && j > 0
                    && k > 0
                    && i < CHUNK_SIZE
                    && j < CHUNK_SIZE
                    && k < CHUNK_SIZE
                    && (chunk[i][j][k] == Block::Log || chunk[i][j][k] == Block::Leaf)
                {
                    return false;
                }
            }
        }
    }
    true
}

pub fn find_top_block(
    chunk: &[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    x: usize,
    y: usize,
    z: usize,
) -> (Block, usize, usize, usize) {
    let min_coor = x.min(y).min(z);
    for i in 0..min_coor {
        if chunk[x - i][y - i][z - i] != Block::Air {
            return (chunk[x - i][y - i][z - i], x - i, y - i, z - i);
        }
    }
    (Block::Air, 0, 0, 0)
}

pub fn generate_chunk(
    perlin: &Perlin,
    x_off: f64,
    y_off: f64,
) -> [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] {
    let mut chunk = Chunk::default().blocks;

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let stone_h = (perlin.get([
                (x as f64 + x_off) * SAMPLE_SCALE,
                (y as f64 + y_off) * SAMPLE_SCALE,
                0.0,
            ]) * (CHUNK_SIZE as f64))
                .mul_add(1.2, 1.0) as usize;
            let dirt_h = perlin
                .get([
                    (x as f64 + x_off) * SAMPLE_SCALE,
                    (y as f64 + y_off) * SAMPLE_SCALE,
                    2.0,
                ])
                .mul_add(5.0, stone_h as f64)
                .mul_add(1.2, 1.0) as usize;

            for z in 0..stone_h {
                chunk[x][y][z] = Block::Stone;
            }

            for z in stone_h..dirt_h {
                chunk[x][y][z] = Block::Dirt;
            }
            if dirt_h > WATER_LEVEL + 1 {
                chunk[x][y][dirt_h] = Block::Grass;
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
                        perlin
                            .get([
                                (x as f64 + x_off) * SAMPLE_SCALE,
                                (y as f64 + y_off) * SAMPLE_SCALE,
                                4.0,
                            ])
                            .abs()
                            .mul_add(5.0, 3.0) as usize,
                    )
                {
                    //spawn trunk
                    for z in dirt_h..dirt_h + TREE_HEIGHT {
                        chunk[x][y][z] = Block::Log;
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
                    for chunk_x in &mut chunk[x_min..x_max] {
                        for chunk_y in &mut chunk_x[y_min..y_max] {
                            for element in &mut chunk_y[z_min..z_max] {
                                *element = Block::Leaf;
                            }
                        }
                    }
                }
            } else if dirt_h == WATER_LEVEL || dirt_h == WATER_LEVEL + 1 {
                chunk[x][y][dirt_h] = Block::Sand;
            } else {
                for z in dirt_h..WATER_LEVEL {
                    chunk[x][y][z] = Block::Water;
                }
                chunk[x][y][WATER_LEVEL] = Block::WaterTop;
            }
        }
    }

    //spawn chunk entity
    chunk
}

fn spawn_block(
    commands: &mut Commands,
    block_layout_handle: Handle<TextureAtlasLayout>,
    block_texture_handle: Handle<Image>,
    sprite_index: Block,
    x: f32,
    y: f32,
    z: f32,
) -> Entity {
    return commands
        .spawn((
            RenderedBlock,
            // ChunkPosition { x, y, z },
            SpriteSheetBundle {
                atlas: TextureAtlas {
                    layout: block_layout_handle,
                    index: sprite_index as usize,
                },
                sprite: Sprite::default(),
                texture: block_texture_handle,
                transform: Transform::from_translation(Vec3::new(
                    (x * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 2.0)
                        - (y * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 2.0),
                    -(y * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 4.0)
                        - (x * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 4.0)
                        + (z * BLOCK_TEXTURE_SIZE * RENDER_SCALE / 2.0),
                    x + y + z,
                ))
                .with_scale(Vec3::splat(RENDER_SCALE)),
                ..Default::default()
            },
        ))
        .id();
}

pub fn spawn_visable_blocks(
    commands: &mut Commands,
    block_layout_handle: Handle<TextureAtlasLayout>,
    block_texture_handle: Handle<Image>,
    chunk: &[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    chunk_ent: Entity,
    x_off: f32,
    y_off: f32,
    z_off: f32,
) {
    for i in 0..CHUNK_SIZE {
        for j in 0..CHUNK_SIZE {
            let mut top_block_id = find_top_block(chunk, i, j, CHUNK_SIZE - 1);
            if top_block_id.0 != Block::Air {
                let block_ent = spawn_block(
                    commands,
                    block_layout_handle.clone(),
                    block_texture_handle.clone(),
                    top_block_id.0,
                    top_block_id.1 as f32 + x_off,
                    top_block_id.2 as f32 + y_off,
                    top_block_id.3 as f32 + z_off,
                );
                commands.entity(chunk_ent).push_children(&[block_ent]);
            }
            top_block_id = find_top_block(chunk, i, CHUNK_SIZE - 1, j);
            if top_block_id.0 != Block::Air {
                let block_ent = spawn_block(
                    commands,
                    block_layout_handle.clone(),
                    block_texture_handle.clone(),
                    top_block_id.0,
                    top_block_id.1 as f32 + x_off,
                    top_block_id.2 as f32 + y_off,
                    top_block_id.3 as f32 + z_off,
                );
                commands.entity(chunk_ent).push_children(&[block_ent]);
            }
            top_block_id = find_top_block(chunk, CHUNK_SIZE - 1, i, j);
            if top_block_id.0 != Block::Air {
                let block_ent = spawn_block(
                    commands,
                    block_layout_handle.clone(),
                    block_texture_handle.clone(),
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

pub fn spawn_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("cursor.png");
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(CURSOR_TEXTURE_SIZE, CURSOR_TEXTURE_SIZE + 1.0),
        4,
        1,
        None,
        None,
    );
    let layout_handle = texture_atlases.add(layout);

    commands.spawn((
        Cursor,
        SpriteSheetBundle {
            atlas: TextureAtlas {
                layout: layout_handle,
                index: CURSOR_FULL,
            },
            sprite: Sprite::default(),
            texture: texture_handle,
            transform: Transform::default().with_scale(Vec3::splat(RENDER_SCALE)),
            ..default()
        },
    ));
}

pub fn move_cursor(mut q: Query<&mut Transform, With<Cursor>>, mouse_state: Res<MouseState>) {
    const HALF_CURSOR_SIZE: f32 = CURSOR_TEXTURE_SIZE / 2.0;
    let mut transform = q.single_mut();

    let Vec2 { x, y } = mouse_state.position;
    let grid_x = (x / HALF_CURSOR_SIZE).trunc();
    let grid_y = (y / HALF_CURSOR_SIZE).trunc() - (if grid_x as i32 % 2 != 0 { 0.5 } else { 0.0 });

    let new_position = Vec3::new(grid_x * HALF_CURSOR_SIZE, grid_y * HALF_CURSOR_SIZE, 100.);

    transform.translation = new_position;
}
