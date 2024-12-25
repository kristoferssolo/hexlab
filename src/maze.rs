use super::{HexTile, Walls};
#[cfg(feature = "bevy_reflect")]
use bevy_utils::HashMap;
use hexx::{EdgeDirection, Hex};
#[cfg(not(feature = "bevy_reflect"))]
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

/// Represents a hexagonal maze with tiles and walls.
///
/// This struct stores the layout of a hexagonal maze, including the positions
/// of tiles and their associated walls.
#[allow(clippy::module_name_repetitions)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", derive(bevy::Component))]
#[cfg_attr(feature = "bevy", reflect(bevy::Component))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HexMaze(HashMap<Hex, HexTile>);

impl HexMaze {
    /// Creates a new empty maze
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let maze = HexMaze::new();
    ///
    /// assert!(maze.is_empty());
    /// assert_eq!(maze.len(), 0);
    /// ```
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new tile at the specified coordinates
    ///
    /// # Arguments
    ///
    /// - `coords` - The hexagonal coordinates where the tile should be added.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let mut maze = HexMaze::new();
    /// let coord = Hex::ZERO;
    /// maze.add_tile(coord);
    ///
    /// assert_eq!(maze.len(), 1);
    /// assert!(!maze.is_empty());
    /// ```
    pub fn add_tile(&mut self, coords: Hex) {
        let tile = HexTile::new(coords);
        self.0.insert(coords, tile);
    }

    /// Adds a wall in the specified direction at the given coordinates.
    ///
    /// # Arguments
    ///
    /// - `coord` - The hexagonal coordinates of the tile.
    /// - `direction` - The direction in which to add the wall.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let mut maze = HexMaze::new();
    /// let coord = Hex::ZERO;
    /// maze.add_tile(coord);
    ///
    /// maze.add_wall(coord, EdgeDirection::FLAT_NORTH);
    /// let walls = maze.get_walls(&coord);
    /// assert!(walls.is_some());
    /// assert!(walls.unwrap().contains(EdgeDirection::FLAT_NORTH));
    /// ```
    pub fn add_wall(&mut self, coord: Hex, direction: EdgeDirection) {
        if let Some(tile) = self.0.get_mut(&coord) {
            tile.walls.add(direction);
        }
    }

    /// Returns a reference to the tile at the specified coordinates.
    ///
    /// # Arguments
    ///
    /// - `coord` - The hexagonal coordinates of the tile to retrieve.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let mut maze = HexMaze::new();
    /// let coord = Hex::ZERO;
    /// maze.add_tile(coord);
    ///
    /// assert!(maze.get_tile(&coord).is_some());
    /// assert!(maze.get_tile(&Hex::new(1, 1)).is_none());
    /// ```
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    pub fn get_tile(&self, coord: &Hex) -> Option<&HexTile> {
        self.0.get(coord)
    }

    /// Returns an optional reference to the walls at the specified coordinates.
    ///
    /// # Arguments
    ///
    /// - `coord` - The hexagonal coordinates of the tile whose walls to retrieve.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let mut maze = HexMaze::new();
    /// let coord = Hex::new(0, 0);
    /// maze.add_tile(coord);
    ///
    /// maze.add_wall(coord, EdgeDirection::FLAT_NORTH);
    /// let walls = maze.get_walls(&coord).unwrap();
    /// assert!(walls.contains(EdgeDirection::FLAT_NORTH));
    /// ```
    pub fn get_walls(&self, coord: &Hex) -> Option<&Walls> {
        self.0.get(coord).map(HexTile::walls)
    }

    /// Returns the number of tiles in the maze.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let mut maze = HexMaze::new();
    /// assert_eq!(maze.len(), 0);
    ///
    /// maze.add_tile(Hex::new(0, 0));
    /// assert_eq!(maze.len(), 1);
    ///
    /// maze.add_tile(Hex::new(1, -1));
    /// assert_eq!(maze.len(), 2);
    /// ```
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the maze contains no tiles.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let mut maze = HexMaze::new();
    /// assert!(maze.is_empty());
    ///
    /// maze.add_tile(Hex::ZERO);
    /// assert!(!maze.is_empty());
    /// ```
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Removes a wall from a tile in the specified direction.
    ///
    /// # Arguments
    ///
    /// - `coord` - The hexagonal coordinates of the tile.
    /// - `direction` - The direction of the wall to remove.
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let mut maze = HexMaze::new();
    /// let coord = Hex::ZERO;
    /// maze.add_tile(coord);
    ///
    /// maze.add_wall(coord, EdgeDirection::FLAT_NORTH);
    /// maze.remove_tile_wall(&coord, EdgeDirection::FLAT_NORTH);
    ///
    /// let walls = maze.get_walls(&coord).unwrap();
    /// assert!(!walls.contains(EdgeDirection::FLAT_NORTH));
    /// ```
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
