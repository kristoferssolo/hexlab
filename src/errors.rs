use hexx::Hex;
use thiserror::Error;

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
    #[error("Invalid coordinate: {0:?}")]
    InvalidCoordinate(Hex),
}
