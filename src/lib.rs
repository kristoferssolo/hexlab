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
//! let maze = MazeBuilder::new()
//!     .with_radius(3)
//!     .build()
//!     .expect("Failed to create maze");
//!
//! assert_eq!(maze.len(), 37); // A radius of 3 should create 37 tiles
//!```
//!
//! Customizing maze generation:
//!
//!```
//! use hexlab::prelude::*;
//!
//! let maze = MazeBuilder::new()
//!     .with_radius(2)
//!     .with_seed(12345)
//!     .with_start_position(Hex::new(1, -1))
//!     .build()
//!     .expect("Failed to create maze");
//!
//! assert!(maze.get(&Hex::new(1, -1)).is_some());
//!```
//!
//! Manipulating walls:
//!
//!```
//! use hexlab::prelude::*;
//!
//! let mut walls = Walls::empty();
//! assert!(!walls.insert(EdgeDirection::FLAT_NORTH));
//! assert!(walls.contains(EdgeDirection::FLAT_NORTH));
//! assert!(!walls.contains(EdgeDirection::FLAT_SOUTH));
//!```
mod builder;
pub mod errors;
mod generator;
mod maze;
mod tile;
pub mod traits;
mod walls;

pub use builder::MazeBuilder;
pub use errors::*;
pub use generator::GeneratorType;
pub use maze::Maze;
pub use tile::Tile;
pub use traits::*;
pub use walls::Walls;

/// Prelude module containing commonly used types
pub mod prelude {
    pub use super::{errors::*, traits::*, GeneratorType, Maze, MazeBuilder, Tile, Walls};
    pub use hexx::{EdgeDirection, Hex, HexLayout};
}
