use super::Walls;
#[cfg(feature = "bevy_reflect")]
use crate::traits::WorldPositionable;
use crate::traits::{TilePosition, WallStorage};
#[cfg(feature = "bevy")]
use bevy::prelude::*;
use hexx::Hex;
#[cfg(feature = "bevy_reflect")]
use hexx::HexLayout;
use std::fmt::Display;

/// Represents a single hexagonal tile in the maze
///
/// Each tile has a position and a set of walls defining its boundaries.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "bevy", reflect(Component))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Tile {
    pub(crate) pos: Hex,
    pub(crate) walls: Walls,
}

impl TilePosition for Tile {
    /// Returns position of the tile
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let tile = Tile::new(Hex::new(2, -2));
    /// assert_eq!(tile.pos(), Hex::new(2, -2));
    /// ```
    #[inline]
    fn pos(&self) -> Hex {
        self.pos
    }
}

impl WallStorage for Tile {
    /// Returns an immutable reference to the tile's walls
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let tile = Tile::new(Hex::ZERO);
    /// assert_eq!(*tile.walls(), Walls::default());
    /// ```
    #[inline]
    fn walls(&self) -> &Walls {
        &self.walls
    }

    /// Returns a mutable reference to the tile's walls
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let tile = Tile::new(Hex::ZERO);
    /// assert_eq!(*tile.walls(), Walls::default());
    /// ```
    #[inline]
    fn walls_mut(&mut self) -> &mut Walls {
        &mut self.walls
    }
}

#[cfg(feature = "bevy_reflect")]
impl WorldPositionable for Tile {
    /// Converts the tile's position to a 2D vector based on the given layout.
    ///
    /// # Arguments
    ///
    /// - `layout` - The hexagonal layout used for conversion.
    #[inline]
    fn to_vec2(&self, layout: &HexLayout) -> glam::Vec2 {
        layout.hex_to_world_pos(self.pos)
    }

    /// Converts the tile's position to a 3D vector based on the given layout.
    ///
    /// # Arguments
    ///
    /// - `layout` - The hexagonal layout used for conversion.
    #[inline]
    fn to_vec3(&self, layout: &HexLayout) -> glam::Vec3 {
        let pos = self.to_vec2(layout);
        glam::Vec3::new(pos.x, 0., pos.y)
    }
}

impl Tile {
    /// Creates a new tile with the given position and default walls.
    ///
    /// # Arguments
    ///
    /// - `pos` - The hexagonal coordinates of the tile.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let tile = Tile::new(Hex::new(1, -1));
    /// assert_eq!(tile.pos(), Hex::new(1, -1));
    /// assert_eq!(*tile.walls(), Walls::default());
    /// ```
    #[must_use]
    pub fn new(pos: Hex) -> Self {
        Self {
            pos,
            walls: Walls::default(),
        }
    }
}

impl From<Hex> for Tile {
    fn from(value: Hex) -> Self {
        Self {
            pos: value,
            walls: Walls::default(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.pos.x, self.pos.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hexx::EdgeDirection;
    use rand::{thread_rng, Rng};

    fn random_hex() -> Hex {
        let mut rng = thread_rng();
        Hex::new(rng.gen(), rng.gen())
    }

    #[test]
    fn different_positions() {
        let positions = [Hex::ZERO, Hex::new(1, 0), Hex::new(-1, 1), Hex::new(2, -2)];

        // Create tiles at different positions
        let tiles = positions
            .iter()
            .map(|&pos| Tile::new(pos))
            .collect::<Vec<_>>();

        // Verify each tile has correct position
        for (tile, &pos) in tiles.iter().zip(positions.iter()) {
            assert_eq!(tile.pos, pos);
        }
    }

    #[test]
    fn hex_boundaries() {
        // Test with extreme coordinate values
        let extreme_positions = [
            Hex::new(i32::MAX, i32::MIN),
            Hex::new(i32::MIN, i32::MAX),
            Hex::new(0, i32::MAX),
            Hex::new(i32::MIN, 0),
        ];

        for pos in extreme_positions {
            let tile = Tile::new(pos);
            assert_eq!(tile.pos, pos);
        }
    }

    #[test]
    fn hex_tile_creation_and_properties() {
        let hex = random_hex();
        let tile = Tile::new(hex);

        assert_eq!(tile.pos(), hex);
        assert!(tile.walls().is_enclosed());
    }

    #[test]
    fn hex_tile_from_hex() {
        let hex = random_hex();
        let tile = Tile::from(hex);

        assert_eq!(tile.pos, hex);
        assert_eq!(tile.walls, Walls::default());
    }

    #[test]
    fn hex_hex_into_tile() {
        let hex = random_hex();
        let tile: Tile = hex.into();

        assert_eq!(tile.pos, hex);
        assert_eq!(tile.walls, Walls::default());
    }

    #[test]
    fn hex_tile_display() {
        let tile = Tile::new(Hex::new(3, -3));
        assert_eq!(format!("{tile}"), "(3,-3)");
    }

    #[test]
    fn hex_tile_wall_modifications() {
        let mut tile = Tile::new(Hex::ZERO);

        for direction in EdgeDirection::ALL_DIRECTIONS {
            tile.walls.insert(direction);
        }
        assert_eq!(tile.walls.count(), 6);

        for direction in EdgeDirection::ALL_DIRECTIONS {
            tile.walls.remove(direction);
        }
        assert_eq!(tile.walls.count(), 0);
    }

    #[cfg(feature = "bevy_reflect")]
    mod bevy_tests {
        use super::*;
        use glam::{Vec2, Vec3};

        #[test]
        fn hex_tile_to_vec2() {
            let layout = HexLayout::default();
            let tile = Tile::new(Hex::new(1, 0));
            let vec2 = tile.to_vec2(&layout);
            assert_eq!(vec2, Vec2::new(1.5, -0.8660254));
        }

        #[test]
        fn hex_tile_to_vec3() {
            let layout = HexLayout::default();
            let tile = Tile::new(Hex::new(0, 1));
            let vec3 = tile.to_vec3(&layout);
            assert_eq!(vec3, Vec3::new(0.0, 0.0, -1.7320508));
        }
    }
}
