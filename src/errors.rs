use hexx::{EdgeDirection, Hex};
use thiserror::Error;

use crate::Tile;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MazeBuilderError {
    /// Occurs when attempting to build a maze without specifying a radius.
    #[error("Radius must be specified to build a maze")]
    NoRadius,

    /// Occurs when the specified start position is outside the maze bounds.
    #[error("Start position {0:?} is outside maze bounds")]
    InvalidStartPosition(Hex),

    /// Occurs when maze generation fails.
    #[error("Failed to generate maze: {0}")]
    GenerationError(String),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MazeError {
    /// Error when attempting to access or modify a tile at a non-existent coordinate.
    #[error("Invalid coordinate: {0:?}")]
    InvalidCoordinate(Hex),

    /// Error when a tile's internal position doesn't match its insertion coordinate.
    #[error("Tile position ({tile_pos:?}) does not match insertion coordinates ({insert_pos:?})")]
    PositionMismatch { tile_pos: Hex, insert_pos: Hex },

    /// Error when attempting to insert a tile at an already occupied position.
    #[error("A tile {old_tile:?} already exists at position {pos:?}")]
    TileAlreadyExists { pos: Hex, old_tile: Tile },

    /// Error when a wall operation fails at the specified coordinate and direction.
    #[error("Cannot add wall at {coord:?} in direction {direction:?}")]
    WallOperationFailed {
        coord: Hex,
        direction: EdgeDirection,
    },
}
