pub type Coordinate = Vector2<i8>;
pub type Offset = Vector2<i8>;

use crate::engine::tile::Tile;
use cgmath::Vector2;
use std::{collections::HashMap, iter::Map};

pub(crate) struct Board {
    tiles: HashMap<Coordinate, Tile>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            tiles: HashMap::new(),
        }
    }
    fn get_tile_offsets(&self, directions: &'static [Offset], coord: &Coordinate) -> Vec<&Tile> {
        directions
            .iter()
            .map(|offset| coord + offset)
            .filter_map(|coord| self.tiles.get(&coord))
            .collect()
    }
    pub fn get_neighbour_tiles(&self, coord: &Coordinate) -> Vec<&Tile> {
        static NEIGHBOUR_DIRECTIONS: [Offset; 8] = [
            Offset::new(-1, -1),
            Offset::new(0, -1),
            Offset::new(1, -1),
            Offset::new(-1, 0),
            Offset::new(1, 0),
            Offset::new(-1, 1),
            Offset::new(0, 1),
            Offset::new(1, 1),
        ];
        self.get_tile_offsets(&NEIGHBOUR_DIRECTIONS, coord)
    }
    pub fn get_adjacent_tiles(&self, coord: &Coordinate) -> Vec<&Tile> {
        static ADJACENT_DIRECTIONS: [Offset; 4] = [
            Offset::new(0, -1),
            Offset::new(-1, 0),
            Offset::new(1, 0),
            Offset::new(0, 1),
        ];
        self.get_tile_offsets(&ADJACENT_DIRECTIONS, coord)
    }
    pub fn get_tile(&self, position: &Coordinate) -> Option<&Tile> {
        self.tiles.get(position)
    }
    pub fn set_tile(&mut self, position: Coordinate, tile: Tile) {
        debug_assert!(self.get_tile(&position) == None);
        self.tiles.insert(position, tile);
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::tile::TileKind;

    use super::*;

    #[test]
    fn test_board_tile() {
        let mut board = Board::new();
        let position = Coordinate::new(1, 1);
        board.set_tile(position, Tile::new(position, TileKind::Water));
        // Tile should have the same position
        assert_eq!(
            board
                .get_tile(&Coordinate::new(1, 1))
                .unwrap()
                .get_position(),
            &Coordinate::new(1, 1)
        );
        assert_eq!(
            board.get_tile(&Coordinate::new(1, 1)).unwrap().get_kind(),
            &TileKind::Water
        )
    }
    #[test]
    fn test_adjacent() {
        let mut board = Board::new();
        let position = Coordinate::new(1, 1);
        board.set_tile(position, Tile::new(position, TileKind::CoralDark));
        // Adjacent to 1, 1
        let position = Coordinate::new(2, 1);
        board.set_tile(position, Tile::new(position, TileKind::Water));
        // Not adjacent to 1, 1 (diagonal)
        let position = Coordinate::new(2, 2);
        board.set_tile(position, Tile::new(position, TileKind::Rock));
        assert_eq!(board.get_adjacent_tiles(&Coordinate::new(1, 1)).len(), 1);
        // Should be the 2, 1 water tile
        assert_eq!(
            board.get_adjacent_tiles(&Coordinate::new(1, 1))[0].get_position(),
            &Coordinate::new(2, 1)
        );
        assert_eq!(
            board.get_adjacent_tiles(&Coordinate::new(1, 1))[0].get_kind(),
            &TileKind::Water
        );
    }
}
