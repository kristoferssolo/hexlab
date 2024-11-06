use std::collections::HashMap;

use hexx::{EdgeDirection, Hex, HexLayout};

use super::{HexTile, Walls};

/// Represents a hexagonal maze with tiles and walls
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct HexMaze {
    tiles: HashMap<Hex, HexTile>,
    layout: HexLayout,
}

impl HexMaze {
    /// Creates a new empty maze with the specified layout
    #[inline]
    pub fn new(layout: HexLayout) -> Self {
        Self {
            tiles: HashMap::new(),
            layout,
        }
    }

    /// Adds a new tile at the specified coordinates
    #[inline]
    pub fn add_tile(&mut self, coords: Hex) {
        let tile = HexTile::new(coords);
        self.tiles.insert(coords, tile);
    }

    /// Adds a wall in the specified direction at the given coordinates
    #[inline]
    pub fn add_wall(&mut self, coord: Hex, direction: EdgeDirection) {
        if let Some(tile) = self.tiles.get_mut(&coord) {
            tile.walls.add(direction)
        }
    }

    /// Returns a reference to the tile at the specified coordinates
    #[inline]
    pub fn get_tile(&self, coord: &Hex) -> Option<&HexTile> {
        self.tiles.get(coord)
    }

    /// Returns a reference to the walls at the specified coordinates
    #[inline]
    pub fn get_walls(&self, coord: &Hex) -> Option<&Walls> {
        self.tiles.get(coord).map(|tile| tile.walls())
    }

    /// Returns the layout of the maze
    #[inline]
    pub fn layout(&self) -> &HexLayout {
        &self.layout
    }

    /// Returns an iterator over all tiles
    #[inline]
    pub fn tiles(&self) -> impl Iterator<Item = (&Hex, &HexTile)> {
        self.tiles.iter()
    }

    /// Returns the number of tiles in the maze
    #[inline]
    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    /// Returns true if the maze is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maze() {
        let layout = HexLayout::default();
        let mut maze = HexMaze::new(layout);
        let coord = Hex::ZERO;

        maze.add_tile(coord);
        assert!(maze.get_tile(&coord).is_some());

        maze.add_wall(coord, EdgeDirection::FLAT_TOP_LEFT);
        assert!(maze
            .get_walls(&coord)
            .unwrap()
            .has(EdgeDirection::FLAT_TOP_LEFT));
    }
}
