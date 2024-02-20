use crate::chunk::{Block, Chunk, CHUNK_SIZE};
use crate::world::GridPosition;
use bevy::prelude::*;
use pathfinding::prelude::astar;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(Update, apply_pathfinding);
    }
}

#[derive(Component, Default, Debug)]
pub struct Path {
    pub locations: Vec<GridPosition>,
}

pub struct NeighborChunks<'a> {
    north: Option<(&'a Chunk, &'a GridPosition)>,
    south: Option<(&'a Chunk, &'a GridPosition)>,
    east: Option<(&'a Chunk, &'a GridPosition)>,
    west: Option<(&'a Chunk, &'a GridPosition)>,
}

pub fn find_neighbors(
    chunk: (&Chunk, &GridPosition),
    neighbor_chunks: NeighborChunks,
    location: &GridPosition,
) -> Vec<GridPosition> {
    let (x, y, z) = (location.x, location.y, location.z);
    let (chunk_x, chunk_y, chunk_z) = (x - chunk.1.x, y - chunk.1.y, z - chunk.1.z);
    let mut neighbors = Vec::new();

    //check x neigbors
    if let Some((w_chunk, _w_position)) = neighbor_chunks.west {
        if w_chunk.blocks[CHUNK_SIZE - 1][chunk_y][chunk_z] == Block::Air {
            if chunk_z > 0 && w_chunk.blocks[CHUNK_SIZE - 1][chunk_y][chunk_z - 1] != Block::Air {
                neighbors.push(GridPosition { x: x - 1, y, z });
            } else if chunk_z > 1
                && w_chunk.blocks[CHUNK_SIZE - 1][chunk_y][chunk_z - 2] != Block::Air
            {
                neighbors.push(GridPosition {
                    x: x - 1,
                    y,
                    z: z - 1,
                });
            }
        } else if (z < CHUNK_SIZE - 2
            && w_chunk.blocks[CHUNK_SIZE - 1][chunk_y][chunk_z + 1] == Block::Air)
            || z == CHUNK_SIZE - 2
        {
            neighbors.push(GridPosition {
                x: x - 1,
                y,
                z: z + 1,
            });
        }
        if chunk.0.blocks[chunk_x + 1][chunk_y][chunk_z] == Block::Air {
            if chunk_z > 0 && chunk.0.blocks[chunk_x + 1][chunk_y][chunk_z - 1] != Block::Air {
                neighbors.push(GridPosition { x: x + 1, y, z });
            } else if chunk_z > 1 && chunk.0.blocks[chunk_x + 1][chunk_y][chunk_z - 2] != Block::Air
            {
                neighbors.push(GridPosition {
                    x: x + 1,
                    y,
                    z: z - 1,
                });
            }
        } else if (z < CHUNK_SIZE - 2
            && chunk.0.blocks[chunk_x + 1][chunk_y][chunk_z + 1] == Block::Air)
            || z == CHUNK_SIZE - 2
        {
            neighbors.push(GridPosition {
                x: x + 1,
                y,
                z: z + 1,
            });
        }
    } else if let Some((e_chunk, _e_position)) = neighbor_chunks.east {
        if e_chunk.blocks[0][chunk_y][chunk_z] == Block::Air {
            if chunk_z > 0 && e_chunk.blocks[0][chunk_y][chunk_z - 1] != Block::Air {
                neighbors.push(GridPosition { x: x + 1, y, z });
            } else if chunk_z > 1 && e_chunk.blocks[0][chunk_y][chunk_z - 2] != Block::Air {
                neighbors.push(GridPosition {
                    x: x + 1,
                    y,
                    z: z - 1,
                });
            }
        } else if (z < CHUNK_SIZE - 2 && e_chunk.blocks[0][chunk_y][chunk_z + 1] == Block::Air)
            || z == CHUNK_SIZE - 2
        {
            neighbors.push(GridPosition {
                x: x + 1,
                y,
                z: z + 1,
            });
        }
        if chunk.0.blocks[chunk_x - 1][chunk_y][chunk_z] == Block::Air {
            if chunk_z > 0 && chunk.0.blocks[chunk_x - 1][chunk_y][chunk_z - 1] != Block::Air {
                neighbors.push(GridPosition { x: x - 1, y, z });
            } else if chunk_z > 1 && chunk.0.blocks[chunk_x - 1][chunk_y][chunk_z - 2] != Block::Air
            {
                neighbors.push(GridPosition {
                    x: x - 1,
                    y,
                    z: z - 1,
                });
            }
        } else if (z < CHUNK_SIZE - 2
            && chunk.0.blocks[chunk_x - 1][chunk_y][chunk_z + 1] == Block::Air)
            || z == CHUNK_SIZE - 2
        {
            neighbors.push(GridPosition {
                x: x - 1,
                y,
                z: z + 1,
            });
        }
    } else {
        if let Some(left) = chunk_x.checked_sub(1) {
            if chunk.0.blocks[left][chunk_y][chunk_z] == Block::Air {
                if chunk_z > 0 && chunk.0.blocks[left][chunk_y][chunk_z - 1] != Block::Air {
                    neighbors.push(GridPosition { x: left, y, z });
                } else if chunk_z > 1 && chunk.0.blocks[left][chunk_y][chunk_z - 2] != Block::Air {
                    neighbors.push(GridPosition {
                        x: x - 1,
                        y,
                        z: z - 1,
                    });
                }
            } else if (z < CHUNK_SIZE - 2
                && chunk.0.blocks[left][chunk_y][chunk_z + 1] == Block::Air)
                || z == CHUNK_SIZE - 2
            {
                neighbors.push(GridPosition {
                    x: x - 1,
                    y,
                    z: z + 1,
                });
            }
        }
        if chunk.0.blocks[chunk_x + 1][chunk_y][chunk_z] == Block::Air {
            if chunk_z > 0 && chunk.0.blocks[chunk_x + 1][chunk_y][chunk_z - 1] != Block::Air {
                neighbors.push(GridPosition { x: x + 1, y, z });
            } else if chunk_z > 1 && chunk.0.blocks[chunk_x + 1][chunk_y][chunk_z - 2] != Block::Air
            {
                neighbors.push(GridPosition {
                    x: x + 1,
                    y,
                    z: z - 1,
                });
            }
        } else if (z < CHUNK_SIZE - 2
            && chunk.0.blocks[chunk_x + 1][chunk_y][chunk_z + 1] == Block::Air)
            || z == CHUNK_SIZE - 2
        {
            neighbors.push(GridPosition {
                x: x + 1,
                y,
                z: z + 1,
            });
        }
    }

    //check y neighbors
    if let Some((s_chunk, _s_position)) = neighbor_chunks.south {
        if s_chunk.blocks[chunk_x][CHUNK_SIZE - 1][chunk_z] == Block::Air {
            if chunk_z > 0 && s_chunk.blocks[chunk_x][CHUNK_SIZE - 1][chunk_z - 1] != Block::Air {
                neighbors.push(GridPosition { x, y: y - 1, z });
            } else if chunk_z > 1
                && s_chunk.blocks[chunk_x][CHUNK_SIZE - 1][chunk_z - 2] != Block::Air
            {
                neighbors.push(GridPosition {
                    x,
                    y: y - 1,
                    z: z - 1,
                });
            }
        } else if (z < CHUNK_SIZE - 2
            && s_chunk.blocks[chunk_x][CHUNK_SIZE - 1][chunk_z + 1] == Block::Air)
            || z == CHUNK_SIZE - 2
        {
            neighbors.push(GridPosition {
                x,
                y: y - 1,
                z: z + 1,
            });
        }
        if chunk.0.blocks[chunk_x][chunk_y + 1][chunk_z] == Block::Air {
            if chunk_z > 0 && chunk.0.blocks[chunk_x][chunk_y + 1][chunk_z - 1] != Block::Air {
                neighbors.push(GridPosition { x, y: y + 1, z });
            } else if chunk_z > 1 && chunk.0.blocks[chunk_x][chunk_y + 1][chunk_z - 2] != Block::Air
            {
                neighbors.push(GridPosition {
                    x,
                    y: y + 1,
                    z: z - 1,
                });
            }
        } else if (z < CHUNK_SIZE - 2
            && chunk.0.blocks[chunk_x][chunk_y + 1][chunk_z + 1] == Block::Air)
            || z == CHUNK_SIZE - 2
        {
            neighbors.push(GridPosition {
                x,
                y: y + 1,
                z: z + 1,
            });
        }
    } else if let Some((n_chunk, _n_position)) = neighbor_chunks.north {
        if n_chunk.blocks[chunk_x][0][chunk_z] == Block::Air {
            if chunk_z > 0 && n_chunk.blocks[chunk_x][0][chunk_z - 1] != Block::Air {
                neighbors.push(GridPosition { x, y: y + 1, z });
            } else if chunk_z > 1 && n_chunk.blocks[chunk_x][0][chunk_z - 2] != Block::Air {
                neighbors.push(GridPosition {
                    x,
                    y: y + 1,
                    z: z - 1,
                });
            }
        } else if (z < CHUNK_SIZE - 2 && n_chunk.blocks[chunk_x][0][chunk_z + 1] == Block::Air)
            || z == CHUNK_SIZE - 2
        {
            neighbors.push(GridPosition {
                x,
                y: y + 1,
                z: z + 1,
            });
        }
        if chunk.0.blocks[chunk_x][chunk_y - 1][chunk_z] == Block::Air {
            if chunk_z > 0 && chunk.0.blocks[chunk_x][chunk_y - 1][chunk_z - 1] != Block::Air {
                neighbors.push(GridPosition { x, y: y - 1, z });
            } else if chunk_z > 1 && chunk.0.blocks[chunk_x][chunk_y - 1][chunk_z - 2] != Block::Air
            {
                neighbors.push(GridPosition {
                    x,
                    y: y - 1,
                    z: z - 1,
                });
            }
        } else if (z < CHUNK_SIZE - 2
            && chunk.0.blocks[chunk_x][chunk_y - 1][chunk_z + 1] == Block::Air)
            || z == CHUNK_SIZE - 2
        {
            neighbors.push(GridPosition {
                x,
                y: y - 1,
                z: z + 1,
            });
        }
    } else {
        if let Some(up) = chunk_y.checked_sub(1) {
            if chunk.0.blocks[chunk_x][up][chunk_z] == Block::Air {
                if chunk_z > 0 && chunk.0.blocks[chunk_x][up][chunk_z - 1] != Block::Air {
                    neighbors.push(GridPosition { x, y: y - 1, z });
                } else if chunk_z > 1 && chunk.0.blocks[chunk_x][up][chunk_z - 2] != Block::Air {
                    neighbors.push(GridPosition {
                        x,
                        y: y - 1,
                        z: z - 1,
                    });
                }
            } else if (z < CHUNK_SIZE - 2
                && chunk.0.blocks[chunk_x][chunk_y - 1][chunk_z + 1] == Block::Air)
                || z == CHUNK_SIZE - 2
            {
                neighbors.push(GridPosition {
                    x,
                    y: y - 1,
                    z: z + 1,
                });
            }
        }
        if chunk.0.blocks[chunk_x][chunk_y + 1][chunk_z] == Block::Air {
            if chunk_z > 0 && chunk.0.blocks[chunk_x][chunk_y + 1][chunk_z - 1] != Block::Air {
                neighbors.push(GridPosition { x, y: y + 1, z });
            } else if chunk_z > 1 && chunk.0.blocks[chunk_x][chunk_y + 1][chunk_z - 2] != Block::Air
            {
                neighbors.push(GridPosition {
                    x,
                    y: y + 1,
                    z: z - 1,
                });
            }
        } else if (z < CHUNK_SIZE - 2
            && chunk.0.blocks[chunk_x][chunk_y + 1][chunk_z + 1] == Block::Air)
            || z == CHUNK_SIZE - 2
        {
            neighbors.push(GridPosition {
                x,
                y: y + 1,
                z: z + 1,
            });
        }
    }
    neighbors
}

pub fn find_chunks_for_neighbors<'a>(
    location: &'a GridPosition,
    chunks: &'a Vec<(Chunk, GridPosition)>,
) -> ((&'a Chunk, &'a GridPosition), NeighborChunks<'a>) {
    let mut chunk = None;
    let mut neighbor_chunks = NeighborChunks {
        north: None,
        south: None,
        east: None,
        west: None,
    };
    for (c, pos) in chunks {
        if pos.x <= location.x
            && pos.x + CHUNK_SIZE > location.x
            && pos.y <= location.y
            && pos.y + CHUNK_SIZE > location.y
        {
            chunk = Some((c, pos));
        }
        if pos.x == location.x + CHUNK_SIZE
            && pos.y <= location.y
            && pos.y + CHUNK_SIZE > location.y
        {
            neighbor_chunks.east = Some((c, pos));
        } else if let Some(left) = location.x.checked_sub(CHUNK_SIZE) {
            if pos.x == left && pos.y <= location.y && pos.y + CHUNK_SIZE > location.y {
                neighbor_chunks.west = Some((c, pos));
            }
        }
        if pos.x <= location.x
            && pos.x + CHUNK_SIZE > location.x
            && pos.y == location.y + CHUNK_SIZE
        {
            neighbor_chunks.north = Some((c, pos));
        } else if let Some(up) = location.y.checked_sub(CHUNK_SIZE) {
            if pos.x <= location.x && pos.x + CHUNK_SIZE > location.x && pos.y == up {
                neighbor_chunks.south = Some((c, pos));
            }
        }
    }
    (chunk.unwrap(), neighbor_chunks)
}

pub fn successors(
    position: &GridPosition,
    chunks: &Vec<(Chunk, GridPosition)>,
) -> Vec<(GridPosition, usize)> {
    let (chunk, neighbor_chunks) = find_chunks_for_neighbors(position, chunks);
    find_neighbors(chunk, neighbor_chunks, position)
        .iter()
        .map(|neighbor| (neighbor.clone(), 1))
        .collect::<Vec<_>>()
}

const fn check_success(position: &GridPosition, end: &GridPosition) -> bool {
    let lower_bound = GridPosition {
        x: {
            if end.x > 0 {
                end.x - 1
            } else {
                0
            }
        },
        y: {
            if end.y > 0 {
                end.y - 1
            } else {
                0
            }
        },
        z: {
            if end.z > 0 {
                end.z - 1
            } else {
                0
            }
        },
    };
    position.x > lower_bound.x
        && position.x < end.x + 1
        && position.y > lower_bound.y
        && position.y < end.y + 1
        && position.z > lower_bound.z
        && position.z < end.z + 1
}

pub fn path_to(
    chunks: &Vec<(Chunk, GridPosition)>,
    start: &GridPosition,
    end: &GridPosition,
) -> Result<Path, String> {
    let result = astar(
        start,
        |p| successors(p, chunks),
        |p| {
            ((p.x as isize - end.x as isize).abs()
                + (p.y as isize - end.y as isize).abs()
                + (p.z as isize - end.z as isize).abs()) as usize
                / 9
        },
        |p| check_success(p, end),
    );
    if let Some((path, _cost)) = result {
        Ok(Path { locations: path })
    } else {
        Err("No path found".to_string())
    }
}

#[derive(Debug)]
pub struct PathfindingError;

#[cfg(test)]
mod tests {

    use crate::{
        chunk::{Block, Chunk},
        pathfinding::path_to,
        world::GridPosition,
    };

    #[test]
    fn single_chunk_pathfinding() {
        let goal = GridPosition::new(3, 3, 1);
        let start = GridPosition::new(0, 0, 1);
        let mut chunk = Chunk::default();
        for x in 0..4 {
            for y in 0..4 {
                chunk.blocks[x][y][0] = Block::Dirt;
            }
        }
        let chunks = vec![(chunk, GridPosition::new(0, 0, 0))];

        let result = path_to(&chunks, &start, &goal);
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn multi_chunk_pathfinding() {
        let goal = GridPosition::new(9, 9, 1);
        let start = GridPosition::new(0, 0, 1);
        let mut chunk = Chunk::default();
        for x in 0..4 {
            for y in 0..4 {
                chunk.blocks[x][y][0] = Block::Dirt;
            }
        }
        let chunks = vec![
            (chunk.clone(), GridPosition::new(0, 0, 0)),
            (chunk.clone(), GridPosition::new(3, 0, 0)),
            (chunk.clone(), GridPosition::new(6, 0, 0)),
            (chunk.clone(), GridPosition::new(0, 3, 0)),
            (chunk.clone(), GridPosition::new(3, 3, 0)),
            (chunk.clone(), GridPosition::new(6, 3, 0)),
            (chunk.clone(), GridPosition::new(0, 6, 0)),
            (chunk.clone(), GridPosition::new(3, 6, 0)),
            (chunk.clone(), GridPosition::new(6, 6, 0)),
        ];

        let result = path_to(&chunks, &start, &goal);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
