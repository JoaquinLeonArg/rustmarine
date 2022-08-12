type CubeCoordinate = Vector2<i8>;
type CubeOffset = Vector2<i8>;

use crate::engine::tile::Tile;
use cgmath::Vector2;
use std::{collections::HashMap, iter::Map};

pub(crate) struct Board {
    tiles: HashMap<CubeCoordinate, Tile>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            tiles: HashMap::new(),
        }
    }
    fn get_tile_offsets(
        &self,
        directions: &'static [CubeOffset],
        coord: &CubeCoordinate,
    ) -> Vec<&Tile> {
        directions
            .iter()
            .map(|offset| coord + offset)
            .map(|coord| self.tiles.get(&coord))
            .filter(|tile| tile.is_some())
            .map(|tile| tile.expect("tile is none"))
            .collect()
    }
    pub fn get_tile_neighbours(&self, coord: &CubeCoordinate) -> Vec<&Tile> {
        static NEIGHBOUR_DIRECTIONS: [CubeOffset; 8] = [
            CubeOffset::new(-1, -1),
            CubeOffset::new(0, -1),
            CubeOffset::new(1, -1),
            CubeOffset::new(-1, 0),
            CubeOffset::new(1, 0),
            CubeOffset::new(-1, 1),
            CubeOffset::new(0, 1),
            CubeOffset::new(1, 1),
        ];
        self.get_tile_offsets(&NEIGHBOUR_DIRECTIONS, coord)
    }
    pub fn get_tile_adjacent(&self, coord: &CubeCoordinate) -> Vec<&Tile> {
        static ADJACENT_DIRECTIONS: [CubeOffset; 4] = [
            CubeOffset::new(0, -1),
            CubeOffset::new(-1, 0),
            CubeOffset::new(1, 0),
            CubeOffset::new(0, 1),
        ];
        self.get_tile_offsets(&ADJACENT_DIRECTIONS, coord)
    }
}
