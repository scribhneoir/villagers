use super::components::*;
use super::helpers::*;
use crate::world::resources::World;
use bevy::prelude::*;

const BLOCK_TEXTURE_SIZE: f32 = 24.0;

pub fn spawn_chunks(
    mut commands: Commands,
    world: Res<World>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    //add block texture to texture_atlases resource
    let texture_handle = asset_server.load("blocks.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(BLOCK_TEXTURE_SIZE, BLOCK_TEXTURE_SIZE + 1.0),
        24,
        20,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let perlin = world.perlin;
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
