mod generator;
mod maze;
mod tile;
mod walls;

pub use maze::HexMaze;
pub use tile::HexTile;
pub use walls::Walls;

pub mod prelude {
    pub use super::{HexMaze, HexTile, Walls};
    pub use hexx::{EdgeDirection, Hex, HexLayout};
}
