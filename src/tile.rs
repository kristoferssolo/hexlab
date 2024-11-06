use hexx::Hex;

use super::Walls;

/// Represents a single hexagonal tile in the maze
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexTile {
    pos: Hex,
    pub walls: Walls,
}

impl HexTile {
    /// Creates a new tile with pos and default walls
    #[inline]
    pub fn new(pos: Hex) -> Self {
        Self {
            pos,
            walls: Walls::default(),
        }
    }

    /// Returns a reference to the tile's walls
    #[inline]
    pub fn walls(&self) -> &Walls {
        &self.walls
    }
}
