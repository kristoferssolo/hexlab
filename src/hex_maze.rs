use super::{HexTile, Walls};
#[cfg(feature = "bevy_reflect")]
use bevy::prelude::*;
#[cfg(feature = "bevy_reflect")]
use bevy::utils::HashMap;
use hexx::{EdgeDirection, Hex};
#[cfg(not(feature = "bevy_reflect"))]
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

/// Represents a hexagonal maze with tiles and walls
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "bevy", reflect(Component))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HexMaze(HashMap<Hex, HexTile>);

impl HexMaze {
    /// Creates a new empty maze
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new tile at the specified coordinates
    pub fn add_tile(&mut self, coords: Hex) {
        let tile = HexTile::new(coords);
        self.0.insert(coords, tile);
    }

    /// Adds a wall in the specified direction at the given coordinates
    pub fn add_wall(&mut self, coord: Hex, direction: EdgeDirection) {
        if let Some(tile) = self.0.get_mut(&coord) {
            tile.walls.add(direction);
        }
    }

    /// Returns a reference to the tile at the specified coordinates
    #[inline]
    #[must_use]
    pub fn get_tile(&self, coord: &Hex) -> Option<&HexTile> {
        self.0.get(coord)
    }

    /// Returns a reference to the walls at the specified coordinates
    pub fn get_walls(&self, coord: &Hex) -> Option<&Walls> {
        self.0.get(coord).map(HexTile::walls)
    }

    /// Returns the number of tiles in the maze
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the maze is empty
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn remove_tile_wall(&mut self, coord: &Hex, direction: EdgeDirection) {
        if let Some(tile) = self.0.get_mut(coord) {
            tile.walls.remove(direction);
        }
    }
}

impl Deref for HexMaze {
    type Target = HashMap<Hex, HexTile>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HexMaze {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_maze() {
        let maze = HexMaze::default();
        assert!(maze.is_empty(), "New maze should be empty");
        assert_eq!(maze.len(), 0, "New maze should have zero tiles");
    }

    #[test]
    fn add_tile() {
        let mut maze = HexMaze::default();
        let coords = [Hex::ZERO, Hex::new(1, -1), Hex::new(-1, 1)];

        // Add tiles
        for &coord in &coords {
            maze.add_tile(coord);
            assert!(
                maze.get_tile(&coord).is_some(),
                "Tile should exist after adding"
            );
        }

        assert_eq!(
            maze.len(),
            coords.len(),
            "Maze should contain all added tiles"
        );
    }

    #[test]
    fn wall_operations() {
        let mut maze = HexMaze::default();
        let coord = Hex::ZERO;
        maze.add_tile(coord);

        // Test adding walls
        let directions = [
            EdgeDirection::FLAT_TOP,
            EdgeDirection::FLAT_BOTTOM,
            EdgeDirection::POINTY_TOP_RIGHT,
        ];

        for &direction in &directions {
            maze.add_wall(coord, direction);
            assert!(
                maze.get_walls(&coord).unwrap().contains(direction),
                "Wall should exist after adding"
            );
        }
    }

    #[test]
    fn tile_iteration() {
        let mut maze = HexMaze::default();
        let coords = [Hex::ZERO, Hex::new(1, 0), Hex::new(0, 1)];

        // Add tiles
        for &coord in &coords {
            maze.add_tile(coord);
        }

        // Test iterator
        let collected = maze.iter().map(|(_, tile)| tile).collect::<Vec<_>>();
        assert_eq!(
            collected.len(),
            coords.len(),
            "Iterator should yield all tiles"
        );
    }

    #[test]
    fn maze_clone() {
        let mut maze = HexMaze::default();
        let coord = Hex::ZERO;
        maze.add_tile(coord);
        maze.add_wall(coord, EdgeDirection::FLAT_TOP);

        // Test cloning
        let cloned_maze = maze.clone();
        assert_eq!(
            maze.len(),
            cloned_maze.len(),
            "Cloned maze should have same size"
        );
        assert!(
            cloned_maze
                .get_walls(&coord)
                .unwrap()
                .contains(EdgeDirection::FLAT_TOP),
            "Cloned maze should preserve wall state"
        );
    }

    #[test]
    fn empty_tile_operations() {
        let mut maze = HexMaze::default();
        let coord = Hex::ZERO;

        // Operations on non-existent tile
        assert!(
            maze.get_tile(&coord).is_none(),
            "Should return None for non-existent tile"
        );
        assert!(
            maze.get_walls(&coord).is_none(),
            "Should return None for non-existent walls"
        );

        // Adding wall to non-existent tile should not panic
        maze.add_wall(coord, EdgeDirection::FLAT_TOP);
    }

    #[test]
    fn maze_boundaries() {
        let mut maze = HexMaze::default();
        let extreme_coords = [
            Hex::new(i32::MAX, i32::MIN),
            Hex::new(i32::MIN, i32::MAX),
            Hex::new(0, i32::MAX),
            Hex::new(0, i32::MIN),
            Hex::new(i32::MAX, 0),
            Hex::new(i32::MIN, 0),
        ];

        // Test with extreme coordinates
        for &coord in &extreme_coords {
            maze.add_tile(coord);
            assert!(
                maze.get_tile(&coord).is_some(),
                "Should handle extreme coordinates"
            );
        }
    }

    #[test]
    fn iterator_consistency() {
        let mut maze = HexMaze::default();
        let coords = [Hex::ZERO, Hex::new(1, -1), Hex::new(-1, 1)];

        // Add tiles
        for &coord in &coords {
            maze.add_tile(coord);
        }

        // Verify iterator
        let iter_coords = maze.iter().map(|(coord, _)| *coord).collect::<Vec<_>>();
        assert_eq!(
            iter_coords.len(),
            coords.len(),
            "Iterator should yield all coordinates"
        );

        for coord in coords {
            assert!(
                iter_coords.contains(&coord),
                "Iterator should contain all added coordinates"
            );
        }
    }

    #[test]
    fn empty_maze() {
        let maze = HexMaze::default();
        assert!(maze.is_empty(), "New maze should be empty");
    }
}
