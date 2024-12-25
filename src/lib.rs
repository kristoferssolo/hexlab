//! Hexlab is a library for generating and manipulating hexagonal mazes.
//!
//! # Features
//!
//! - Create hexagonal mazes of configurable size
//! - Customizable maze properties (radius, start position, seed)
//! - Efficient bit-flag representation of walls
//! - Multiple maze generation algorithms
//! - Maze builder pattern for easy maze creation

//!
//! # Examples
//!
//! Here's a quick example to create a simple hexagonal maze:
//!
//!```
//! use hexlab::prelude::*;
//!
//! // Create a new maze
//! let maze = MazeBuilder::new()
//!     .with_radius(5)
//!     .build()
//!     .expect("Failed to create maze");
//!
//! // Get a specific tile
//! let tile = maze.get_tile(&Hex::new(1, -1)).unwrap();
//!
//! // Check if a wall exists
//! let has_wall = tile.walls().contains(EdgeDirection::FLAT_NORTH);
//!```
//!
//! # Acknowledgements
//!
//! Hexlab relies on the excellent [hexx](https://github.com/ManevilleF/hexx) library for handling
//! hexagonal grid mathematics, coordinates, and related operations.
mod builder;
mod generator;
mod hex_maze;
mod hex_tile;
mod walls;

pub use builder::{MazeBuilder, MazeBuilderError};
pub use generator::GeneratorType;
pub use hex_maze::HexMaze;
pub use hex_tile::HexTile;
pub use walls::Walls;

/// Prelude module containing commonly used types
pub mod prelude {
    pub use super::{GeneratorType, HexMaze, HexTile, MazeBuilder, MazeBuilderError, Walls};
    pub use hexx::{EdgeDirection, Hex, HexLayout};
}
