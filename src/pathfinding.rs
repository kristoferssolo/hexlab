//! Maze pathfinding implementation for hexagonal grids.
//!
//! This module provides functionality for finding paths through a hexagonal maze
//! using the A* pathfinding algorithm. The maze is represented as a collection of
//! hexagonal cells, where each cell may have walls on any of its six edges.
//!
//! # Examples
//!
//! ```
//! use hexlab::prelude::*;
//!
//! let maze = MazeBuilder::new()
//!     .with_radius(3)
//!     .with_seed(12345)
//!     .build()
//!     .expect("Failed to create maze");
//! assert!(maze.find_path(Hex::ZERO, Hex::new(-1, 3)).is_some());
//! ```
//!
//! # Implementation Details
//!
//! The pathfinding algorithm uses Manhattan distance as a heuristic and considers
//! walls between cells when determining valid paths. Each step between adjacent
//! cells has a cost of 1.
use hexx::{EdgeDirection, Hex};
use pathfinding::prelude::*;

use crate::Maze;

impl Maze {
    #[must_use]
    /// Finds the shortest path between two hexagonal positions in the maze using A* pathfinding.
    ///
    /// This function calculates the optimal path while taking into account walls between cells.
    /// The path cost between adjacent cells is always 1, and Manhattan distance is used as the
    /// heuristic for pathfinding.
    ///
    /// # Arguments
    ///
    /// * `from` - The starting hexagonal position
    /// * `to` - The target hexagonal position
    ///
    /// # Returns
    ///
    /// * `Some(Vec<Hex>)` - A vector of hexagonal positions representing the path from start to target
    /// * `None` - If no valid path exists between the positions
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let maze = MazeBuilder::new()
    ///     .with_radius(3)
    ///     .with_seed(12345)
    ///     .build()
    ///     .expect("Failed to create maze");
    /// assert!(maze.find_path(Hex::ZERO, Hex::new(-1, 3)).is_some());
    /// ```
    pub fn find_path(&self, from: Hex, to: Hex) -> Option<Vec<Hex>> {
        let successors = |pos: &Hex| {
            {
                EdgeDirection::ALL_DIRECTIONS.iter().filter_map(|&dir| {
                    let neighbor = pos.neighbor(dir);
                    if let Some(current_tile) = self.get(pos) {
                        if self.get(&neighbor).is_some() && !current_tile.walls.contains(dir) {
                            return Some((neighbor, 1)); // Cost of 1 for each step
                        }
                    }
                    None
                })
            }
            .collect::<Vec<_>>()
        };

        let heuristic = |pos: &Hex| {
            // Manhatan distance
            let diff = *pos - to;
            (diff.x.abs() + diff.y.abs() + diff.z().abs()) / 2
        };

        astar(&from, successors, heuristic, |pos| *pos == to).map(|(path, _)| path)
    }
}
