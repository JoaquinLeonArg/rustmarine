use crate::engine::board::Coordinate;
use cgmath::Vector2;

#[derive(Eq, PartialEq, Debug)]
pub enum TileKind {
    Water,
    Rock,
    Stingray,
    Cuttlefish,
    Jellyfish,
    CoralDark,
    CoralLight,
    Shipwreck,
    ButterflyShoal,
    Flag(u8),
    Beacon,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Tile {
    position: Coordinate,
    kind: TileKind,
}

impl Tile {
    pub fn new(position: Coordinate, kind: TileKind) -> Self {
        Tile { position, kind }
    }
    pub fn get_position(&self) -> &Coordinate {
        &self.position
    }
    pub fn get_kind(&self) -> &TileKind {
        &self.kind
    }
}
