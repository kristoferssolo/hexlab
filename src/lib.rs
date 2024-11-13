mod builder;
mod generator;
mod maze;
mod tile;
mod walls;

pub use builder::{MazeBuilder, MazeBuilderError};
pub use generator::GeneratorType;
pub use maze::HexMaze;
pub use tile::HexTile;
pub use walls::Walls;

pub mod prelude {
    pub use super::{GeneratorType, HexMaze, HexTile, MazeBuilder, MazeBuilderError, Walls};
    pub use hexx::{EdgeDirection, Hex, HexLayout};
}
