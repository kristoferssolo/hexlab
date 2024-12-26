use crate::Walls;
use hexx::Hex;

pub trait TilePosition {
    /// Returns position of the tile
    #[must_use]
    fn pos(&self) -> Hex;
}

#[cfg(feature = "bevy_reflect")]
pub trait WorldPositionable {
    #[must_use]
    fn to_vec2(&self, layout: &hexx::HexLayout) -> glam::Vec2;
    #[must_use]
    fn to_vec3(&self, layout: &hexx::HexLayout) -> glam::Vec3;
}

pub trait WallStorage {
    #[must_use]
    fn walls(&self) -> &Walls;
    fn walls_mut(&mut self) -> &mut Walls;
}
