use bevy::prelude::*;
use pathfinding::prelude::astar;
use std::collections::VecDeque;
use crate::chunk::block::components::*;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Update, apply_pathfinding);
    }
}

#[derive(Component, Default)]
pub struct Path {
    pub locations: Vec<Position>,
}

pub struct NeighborChunks {
    north: Option<(Chunk,Position)>,
    south: Option<(Chunk,Position)>,
    east: Option<(Chunk,Position)>,
    west: Option<(Chunk,Position)>,
}

pub fn find_neighbors(
    chunk: &(Chunk, Position),
    neighbor_chunks: &NeighborChunks,
    location: &Position,
) -> Vec<Position> {
    let (x,y,z) = (location.x, location.y, location.z);
    let (chunk_x, chunk_y, chunk_z) = (chunk.1.x, chunk.1.y, chunk.1.z);
    let mut neighbors = Vec::new();

    //check x neigbors
    if let Some((chunk, position)) = neighbor_chunks.west {
        if neighbor_chunks.west.0[CHUNK_SIZE-1][chunk_y][chunk_z] == 0 {
            if chunk_z > 0 && neighbor_chunks.west.0[CHUNK_SIZE-1][chunk_y][chunk_z -1] != 0{
                neighbors.push(Position { x: x - 1.0, y, z });
            }
            else if chunk_z > 1 && neighbor_chunks.west.0[CHUNK_SIZE-1][chunk_y][chunk_z -2] != 0{
                neighbors.push(Position { x: x - 1.0, y, z - 1.0 }); 
            }
        }
        else if (z < CHUNK_SIZE -2 && neighbor_chunks.west.0[CHUNK_SIZE-1][chunk_y][chunk_z +1] == 0) || z == CHUNK_SIZE -2{
            neighbors.push(Position { x: x - 1.0, y, z + 1.0 });
        }
        if chunk.0[chunk_x+1][chunk_y][chunk_z] == 0 {
            if chunk_z > 0 && chunk.0[chunk_x+1][chunk_y][chunk_z -1] != 0{
                neighbors.push(Position { x: x + 1.0, y, z });
            }
            else if chunk_z > 1 && chunk.0[chunk_x+1][chunk_y][chunk_z -2] != 0{
                neighbors.push(Position { x: x + 1.0, y, z - 1.0 }); 
            }

        }
        else if (z < CHUNK_SIZE -2 && chunk.0[chunk_x+1][chunk_y][chunk_z +1] == 0) || z == CHUNK_SIZE -2{
            neighbors.push(Position { x: x + 1.0, y, z + 1.0 });
        }
    }

    else if let Some((chunk,position)) = neighbor_chunks.east{
        if neighbor_chunks.east.0[0][chunk_y][chunk_z] == 0 {
            if chunk_z > 0 && neighbor_chunks.east.0[0][chunk_y][chunk_z -1] != 0{
                neighbors.push(Position { x: x + 1.0, y, z });
            }
            else if chunk_z > 1 && neighbor_chunks.east.0[0][chunk_y][chunk_z -2] != 0{
                neighbors.push(Position { x: x + 1.0, y, z - 1.0 }); 
            }
        }
        else if (z < CHUNK_SIZE -2 && neighbor_chunks.east.0[0][chunk_y][chunk_z +1] == 0) || z == CHUNK_SIZE -2{
            neighbors.push(Position { x: x + 1.0, y, z + 1.0 });
        }
        if chunk.0[chunk_x-1][chunk_y][chunk_z] == 0 {
            if chunk_z > 0 && chunk.0[chunk_x-1][chunk_y][chunk_z -1] != 0{
                neighbors.push(Position { x: x - 1.0, y, z });
            }
            else if chunk_z > 1 && chunk.0[chunk_x-1][chunk_y][chunk_z -2] != 0{
                neighbors.push(Position { x: x - 1.0, y, z - 1.0 }); 
            }

        }
        else if (z < CHUNK_SIZE -2 && chunk.0[chunk_x-1][chunk_y][chunk_z +1] == 0) || z == CHUNK_SIZE -2{
            neighbors.push(Position { x: x - 1.0, y, z + 1.0 });
        }
    }

    else {
        if chunk.0[chunk_x-1][chunk_y][chunk_z] == 0 {
            if chunk_z > 0 && chunk.0[chunk_x-1][chunk_y][chunk_z -1] != 0{
                neighbors.push(Position { x: x - 1.0, y, z });
            }
            else if chunk_z > 1 && chunk.0[chunk_x-1][chunk_y][chunk_z -2] != 0{
                neighbors.push(Position { x: x - 1.0, y, z - 1.0 }); 
            }

        }
        else if (z < CHUNK_SIZE -2 && chunk.0[chunk_x-1][chunk_y][chunk_z +1] == 0) || z == CHUNK_SIZE -2{
            neighbors.push(Position { x: x - 1.0, y, z + 1.0 });
        }
        if chunk.0[chunk_x+1][chunk_y][chunk_z] == 0 {
            if chunk_z > 0 && chunk.0[chunk_x+1][chunk_y][chunk_z -1] != 0{
                neighbors.push(Position { x: x + 1.0, y, z });
            }
            else if chunk_z > 1 && chunk.0[chunk_x+1][chunk_y][chunk_z -2] != 0{
                neighbors.push(Position { x: x + 1.0, y, z - 1.0 }); 
            }

        }
        else if (z < CHUNK_SIZE -2 && chunk.0[chunk_x+1][chunk_y][chunk_z +1] == 0) || z == CHUNK_SIZE -2{
            neighbors.push(Position { x: x + 1.0, y, z + 1.0 });
        }
    }

    //check y neighbors
    if let Some((chunk, position)) = neighbor_chunks.south {
        if neighbor_chunks.south.0[chunk_x][CHUNK_SIZE-1][chunk_z] == 0 {
            if chunk_z > 0 && neighbor_chunks.south.0[chunk_x][CHUNK_SIZE-1][chunk_z -1] != 0{
                neighbors.push(Position { x, y:y -1.0, z });
            }
            else if chunk_z > 1 && neighbor_chunks.south.0[chunk_x][CHUNK_SIZE-1][chunk_z -2] != 0{
                neighbors.push(Position { x, y:y -1.0, z - 1.0 }); 
            }
        }
        else if (z < CHUNK_SIZE -2 && neighbor_chunks.south.0[chunk_x][CHUNK_SIZE-1][chunk_z +1] == 0) || z == CHUNK_SIZE -2{
            neighbors.push(Position { x, y:y-1.0, z + 1.0 });
        }
        if chunk.0[chunk_x][chunk_y+1][chunk_z] == 0 {
            if chunk_z > 0 && chunk.0[chunk_x][chunk_y+1][chunk_z -1] != 0{
                neighbors.push(Position { x, y: y + 1.0, z });
            }
            else if chunk_z > 1 && chunk.0[chunk_x][chunk_y+1][chunk_z -2] != 0{
                neighbors.push(Position { x, y: y + 1.0, z - 1.0 }); 
            }

        }
        else if (z < CHUNK_SIZE -2 && chunk.0[chunk_x][chunk_y+1][chunk_z +1] == 0) || z == CHUNK_SIZE -2{
            neighbors.push(Position { x, y: y + 1.0, z + 1.0 });
        }
    }

    else if let Some((chunk,position)) = neighbor_chunks.north{
        if neighbor_chunks.north.0[chunk_x][0][chunk_z] == 0 {
            if chunk_z > 0 && neighbor_chunks.north.0[chunk_x][0][chunk_z -1] != 0{
                neighbors.push(Position { x, y: y + 1.0, z });
            }
            else if chunk_z > 1 && neighbor_chunks.north.0[chunk_x][0][chunk_z -2] != 0{
                neighbors.push(Position { x, y: y + 1.0, z - 1.0 }); 
            }
        }
        else if (z < CHUNK_SIZE -2 && neighbor_chunks.north.0[chunk_x][0][chunk_z +1] == 0) || z == CHUNK_SIZE -2{
            neighbors.push(Position { x, y: y + 1.0, z + 1.0 });
        }
        if chunk.0[chunk_x][chunk_y-1][chunk_z] == 0 {
            if chunk_z > 0 && chunk.0[chunk_x][chunk_y-1][chunk_z -1] != 0{
                neighbors.push(Position { x, y: y - 1.0, z });
            }
            else if chunk_z > 1 && chunk.0[chunk_x][chunk_y-1][chunk_z -2] != 0{
                neighbors.push(Position { x, y: y - 1.0, z - 1.0 }); 
            }

        }
        else if (z < CHUNK_SIZE -2 && chunk.0[chunk_x][chunk_y-1][chunk_z +1] == 0) || z == CHUNK_SIZE -2{
            neighbors.push(Position { x, y: y - 1.0, z + 1.0 });
        }
    }

    else {
        if chunk.0[chunk_x][chunk_y-1][chunk_z] == 0 {
            if chunk_z > 0 && chunk.0[chunk_x][chunk_y-1][chunk_z -1] != 0{
                neighbors.push(Position { x, y: y - 1.0, z });
            }
            else if chunk_z > 1 && chunk.0[chunk_x][chunk_y-1][chunk_z -2] != 0{
                neighbors.push(Position { x, y: y - 1.0, z - 1.0 }); 
            }

        }
        else if (z < CHUNK_SIZE -2 && chunk.0[chunk_x][chunk_y-1][chunk_z +1] == 0) || z == CHUNK_SIZE -2{
            neighbors.push(Position { x, y: y - 1.0, z + 1.0 });
        }
        if chunk.0[chunk_x][chunk_y+1][chunk_z] == 0 {
            if chunk_z > 0 && chunk.0[chunk_x][chunk_y+1][chunk_z -1] != 0{
                neighbors.push(Position { x, y: y + 1.0, z });
            }
            else if chunk_z > 1 && chunk.0[chunk_x][chunk_y+1][chunk_z -2] != 0{
                neighbors.push(Position { x, y: y + 1.0, z - 1.0 }); 
            }

        }
        else if (z < CHUNK_SIZE -2 && chunk.0[chunk_x][chunk_y+1][chunk_z +1] == 0) || z == CHUNK_SIZE -2{
            neighbors.push(Position { x, y: y + 1.0, z + 1.0 });
        }
    }
    neigbors
}

pub fn find_chunks(    location: &Position,chunks: &[(Chunk, Position)]) -> &((Chunk, Position),NeighborChunks) {
    let mut chunk = None;
    let mut neighbor_chunks = NeighborChunks {
        north: None,
        south: None,
        east: None,
        west: None,
    };
    for (c, pos) in chunks {
        if pos.x <= location.x && pos.x+ CHUNK_SIZE > location.x  && pos.y <= location.y && pos.y+ CHUNK_SIZE > location.y {
            chunk = Some((c, pos));
        }
        else if pos.x == location.x + CHUNK_SIZE && pos.y <= location.y && pos.y+ CHUNK_SIZE > location.y {
            neighbor_chunks.east = Some((c, pos));
        }
        else if pos.x == location.x - CHUNK_SIZE && pos.y <= location.y && pos.y+ CHUNK_SIZE > location.y {
            neighbor_chunks.west = Some((c, pos));
        }
        else if pos.x <= location.x && pos.x+ CHUNK_SIZE > location.x && pos.y == location.y + CHUNK_SIZE {
            neighbor_chunks.north = Some((c, pos));
        }
        else if pos.x <= location.x && pos.x+ CHUNK_SIZE > location.x  && pos.y == location.y - CHUNK_SIZE {
            neighbor_chunks.south = Some((c, pos));
        }
    }
    (chunk.unwrap(), neighbor_chunks)
}



