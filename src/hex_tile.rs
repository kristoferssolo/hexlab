use std::fmt::Display;

use hexx::Hex;

#[cfg(feature = "bevy")]
use hexx::HexLayout;

use super::Walls;
#[cfg(feature = "bevy")]
use bevy::prelude::*;

/// Represents a single hexagonal tile in the maze
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy", derive(Reflect, Component))]
#[cfg_attr(feature = "bevy", reflect(Component))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HexTile {
    pub(crate) pos: Hex,
    pub(crate) walls: Walls,
}

impl HexTile {
    /// Creates a new tile with pos and default walls
    #[must_use]
    pub fn new(pos: Hex) -> Self {
        Self {
            pos,
            walls: Walls::default(),
        }
    }

    /// Returns a reference to the tile's walls
    #[inline]
    #[must_use]
    pub const fn walls(&self) -> &Walls {
        &self.walls
    }

    /// Returns position of the tile
    #[inline]
    #[must_use]
    pub const fn pos(&self) -> Hex {
        self.pos
    }

    #[cfg(feature = "bevy")]
    #[inline]
    #[must_use]
    pub fn to_vec2(&self, layout: &HexLayout) -> Vec2 {
        layout.hex_to_world_pos(self.pos)
    }

    #[cfg(feature = "bevy")]
    #[inline]
    #[must_use]
    pub fn to_vec3(&self, layout: &HexLayout) -> Vec3 {
        let pos = self.to_vec2(layout);
        Vec3::new(pos.x, 0., pos.y)
    }
}

impl From<Hex> for HexTile {
    fn from(value: Hex) -> Self {
        Self {
            pos: value,
            walls: Walls::default(),
        }
    }
}

impl Display for HexTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.pos.x, self.pos.y)
    }
}

#[cfg(test)]
mod tests {
    use hexx::EdgeDirection;

    use super::*;

    #[test]
    fn new_tile() {
        let pos = Hex::ZERO;
        let tile = HexTile::new(pos);

        assert_eq!(tile.pos, pos, "Position should match constructor argument");
        assert_eq!(
            tile.walls,
            Walls::default(),
            "Walls should be initialized to default"
        );
    }

    #[test]
    fn tile_walls_accessor() {
        let pos = Hex::new(1, -1);
        let tile = HexTile::new(pos);

        // Test walls accessor method
        let walls_ref = tile.walls();
        assert_eq!(
            walls_ref, &tile.walls,
            "Walls accessor should return reference to walls"
        );
    }

    #[test]
    fn tile_modification() {
        let pos = Hex::new(2, 3);
        let mut tile = HexTile::new(pos);

        // Modify walls
        tile.walls.remove(EdgeDirection::FLAT_TOP);
        assert!(
            !tile.walls.contains(EdgeDirection::FLAT_TOP),
            "Wall should be removed"
        );

        tile.walls.add(EdgeDirection::FLAT_TOP);
        assert!(
            tile.walls.contains(EdgeDirection::FLAT_TOP),
            "Wall should be added back"
        );
    }

    #[test]
    fn tile_clone() {
        let pos = Hex::new(0, -2);
        let tile = HexTile::new(pos);

        // Test Clone trait
        let cloned_tile = tile.clone();
        assert_eq!(tile, cloned_tile, "Cloned tile should equal original");
    }

    #[test]
    fn tile_debug() {
        let pos = Hex::ZERO;
        let tile = HexTile::new(pos);

        // Test Debug trait
        let debug_string = format!("{:?}", tile);
        assert!(
            debug_string.contains("HexTile"),
            "Debug output should contain struct name"
        );
    }

    #[test]
    fn different_positions() {
        let positions = [Hex::ZERO, Hex::new(1, 0), Hex::new(-1, 1), Hex::new(2, -2)];

        // Create tiles at different positions
        let tiles = positions
            .iter()
            .map(|&pos| HexTile::new(pos))
            .collect::<Vec<_>>();

        // Verify each tile has correct position
        for (tile, &pos) in tiles.iter().zip(positions.iter()) {
            assert_eq!(
                tile.pos, pos,
                "Tile position should match constructor argument"
            );
        }
    }

    #[test]
    fn tile_equality() {
        let pos1 = Hex::new(1, 1);
        let pos2 = Hex::new(1, 1);
        let pos3 = Hex::new(2, 1);

        let tile1 = HexTile::new(pos1);
        let tile2 = HexTile::new(pos2);
        let tile3 = HexTile::new(pos3);

        assert_eq!(tile1, tile2, "Tiles with same position should be equal");
        assert_ne!(
            tile1, tile3,
            "Tiles with different positions should not be equal"
        );

        // Test with modified walls
        let mut tile4 = HexTile::new(pos1);
        tile4.walls.remove(EdgeDirection::FLAT_TOP);
        assert_ne!(
            tile1, tile4,
            "Tiles with different walls should not be equal"
        );
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
            let tile = HexTile::new(pos);
            assert_eq!(
                tile.pos, pos,
                "Tile should handle extreme coordinate values"
            );
        }
    }
}
