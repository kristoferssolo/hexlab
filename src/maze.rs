use super::{Tile, Walls};
use crate::{
    errors::MazeError,
    traits::{TilePosition, WallStorage},
};
#[cfg(feature = "bevy")]
use bevy::prelude::*;
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "bevy", reflect(Component))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Maze(HashMap<Hex, Tile>);

impl Maze {
    /// Creates a new empty maze
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let maze = Maze::new();
    ///
    /// assert!(maze.is_empty());
    /// assert_eq!(maze.len(), 0);
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a new tile at the specified coordinates.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned.
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
    /// let mut maze = Maze::new();
    /// let coord = Hex::ZERO;
    ///
    /// assert_eq!(maze.insert(coord), None);
    /// assert_eq!(maze.insert(coord), Some(Tile::new(coord)));
    /// ```
    pub fn insert(&mut self, coords: Hex) -> Option<Tile> {
        let tile = Tile::new(coords);
        self.0.insert(coords, tile)
    }

    /// Adds a new tile at the specified coordinates.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned.
    ///
    /// It is recommended to use [`insert`].
    ///
    /// [`insert`]: Maze::insert
    ///
    /// # Arguments
    ///
    /// - `coords` - The hexagonal coordinates where the tile should be added.
    /// - `tile` - The tile to insert to.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let mut maze = Maze::new();
    /// let coord = Hex::ZERO;
    /// let tile1 = Tile::new(coord);
    /// let tile2 = Tile::new(Hex::new(1, 1));
    ///
    /// assert_eq!(maze.insert_with_tile(coord, tile1.clone()), None);
    /// assert_eq!(maze.insert_with_tile(coord, tile2), Some(tile1));
    /// ```
    pub fn insert_with_tile(&mut self, coords: Hex, tile: Tile) -> Option<Tile> {
        self.0.insert(coords, tile)
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
    /// let mut maze = Maze::new();
    /// let coord = Hex::ZERO;
    /// maze.insert(coord);
    ///
    /// assert!(maze.get(&coord).is_some());
    /// assert!(maze.get(&Hex::new(1, 1)).is_none());
    /// ```
    #[inline]
    #[must_use]
    pub fn get(&self, coord: &Hex) -> Option<&Tile> {
        self.0.get(coord)
    }

    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, coord: &Hex) -> Option<&mut Tile> {
        self.0.get_mut(coord)
    }

    /// Returns an optional mutable reference to the walls at the specified coordinates.
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
    /// let mut maze = Maze::new();
    /// let coord = Hex::new(0, 0);
    /// maze.insert(coord);
    ///
    /// maze.add_tile_wall(&coord, EdgeDirection::FLAT_NORTH);
    /// let walls = maze.get_walls(&coord).unwrap();
    /// assert!(walls.contains(&EdgeDirection::FLAT_NORTH));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_walls(&self, coord: &Hex) -> Option<&Walls> {
        self.0.get(coord).map(Tile::walls)
    }

    /// Returns an optional mutable reference to the walls at the specified coordinates.
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
    /// let mut maze = Maze::new();
    /// let coord = Hex::new(0, 0);
    /// maze.insert(coord);
    ///
    /// maze.add_tile_wall(&coord, EdgeDirection::FLAT_NORTH);
    /// let mut walls = maze.get_walls_mut(&coord).unwrap();
    /// assert!(walls.remove(EdgeDirection::FLAT_NORTH));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_walls_mut(&mut self, coord: &Hex) -> Option<&mut Walls> {
        self.0.get_mut(coord).map(Tile::walls_mut)
    }

    /// Returns the number of tiles in the maze.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let mut maze = Maze::new();
    /// assert_eq!(maze.len(), 0);
    ///
    /// maze.insert(Hex::new(0, 0));
    /// assert_eq!(maze.len(), 1);
    ///
    /// maze.insert(Hex::new(1, -1));
    /// assert_eq!(maze.len(), 2);
    /// ```
    #[inline]
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
    /// let mut maze = Maze::new();
    /// assert!(maze.is_empty());
    ///
    /// maze.insert(Hex::ZERO);
    /// assert!(!maze.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Adds a wall from a tile in the specified direction.
    ///
    /// # Arguments
    ///
    /// - `coord` - The hexagonal coordinates of the tile.
    /// - `direction` - The direction of the wall to remove.
    ///
    /// # Errors
    ///
    /// Returns `MazeError::InvalidCoordinate` if the specified coordinate does not exist in the maze.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// // Create a maze with a single tile at the origin
    /// let mut tile = Tile::new(Hex::ZERO);
    /// tile.walls_mut().toggle(Walls::all_directions());
    /// let mut maze = Maze::from([tile]);
    ///
    /// // Initially, the tile should have no walls
    /// assert!(maze.get_walls(&Hex::ZERO).unwrap().is_empty());
    ///
    /// // Add a wall to the north
    /// assert!(maze.add_tile_wall(&Hex::ZERO, EdgeDirection::FLAT_NORTH).is_ok());
    ///
    /// // Check that the wall was added
    /// let walls = maze.get_walls(&Hex::ZERO).unwrap();
    /// assert!(walls.contains(&EdgeDirection::FLAT_NORTH));
    /// assert_eq!(walls.count(), 1);
    ///
    /// // Adding the same wall again should return true (no change)
    /// assert_eq!(maze.add_tile_wall(&Hex::ZERO, EdgeDirection::FLAT_NORTH), Ok(true));
    ///
    /// // Adding a wall to a non-existent tile should return an error
    /// let invalid_coord = Hex::new(1, 1);
    /// assert_eq!(
    ///     maze.add_tile_wall(&invalid_coord, EdgeDirection::FLAT_NORTH),
    ///     Err(MazeError::InvalidCoordinate(invalid_coord))
    /// );
    /// ```
    pub fn add_tile_wall(
        &mut self,
        coord: &Hex,
        direction: EdgeDirection,
    ) -> Result<bool, MazeError> {
        self.0
            .get_mut(coord)
            .map(|tile| tile.walls.insert(direction))
            .ok_or(MazeError::InvalidCoordinate(*coord))
    }

    /// Removes a wall from a tile in the specified direction.
    ///
    /// # Arguments
    ///
    /// - `coord` - The hexagonal coordinates of the tile.
    /// - `direction` - The direction of the wall to remove.
    ///
    /// # Errors
    ///
    /// Returns `MazeError::InvalidCoordinate` if the specified coordinate does not exist in the maze.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let mut maze = Maze::new();
    /// let coord = Hex::ZERO;
    /// maze.insert(coord);
    ///
    /// maze.add_tile_wall(&coord, EdgeDirection::FLAT_NORTH);
    /// maze.remove_tile_wall(&coord, EdgeDirection::FLAT_NORTH);
    ///
    /// let walls = maze.get_walls(&coord).unwrap();
    /// assert!(!walls.contains(&EdgeDirection::FLAT_NORTH));
    /// ```
    pub fn remove_tile_wall(
        &mut self,
        coord: &Hex,
        direction: EdgeDirection,
    ) -> Result<bool, MazeError> {
        self.0
            .get_mut(coord)
            .map(|tile| tile.walls.remove(direction))
            .ok_or(MazeError::InvalidCoordinate(*coord))
    }
}

impl FromIterator<Hex> for Maze {
    fn from_iter<T: IntoIterator<Item = Hex>>(iter: T) -> Self {
        Self(iter.into_iter().map(|hex| (hex, Tile::new(hex))).collect())
    }
}

impl FromIterator<Tile> for Maze {
    fn from_iter<T: IntoIterator<Item = Tile>>(iter: T) -> Self {
        Self(iter.into_iter().map(|tile| (tile.pos(), tile)).collect())
    }
}

impl FromIterator<(Hex, Tile)> for Maze {
    fn from_iter<T: IntoIterator<Item = (Hex, Tile)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<const N: usize> From<[Hex; N]> for Maze {
    fn from(value: [Hex; N]) -> Self {
        value.into_iter().collect()
    }
}

impl<const N: usize> From<[Tile; N]> for Maze {
    fn from(value: [Tile; N]) -> Self {
        value.into_iter().collect()
    }
}

impl Deref for Maze {
    type Target = HashMap<Hex, Tile>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Maze {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
