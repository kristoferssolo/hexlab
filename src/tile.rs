use hexx::Hex;

use super::Walls;

/// Represents a single hexagonal tile in the maze
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexTile {
    pub pos: Hex,
    pub walls: Walls,
}

impl HexTile {
    /// Creates a new tile with pos and default walls
    #[inline]
    pub fn new(pos: Hex) -> Self {
        Self {
            pos,
            walls: Walls::default(),
        }
    }

    /// Returns a reference to the tile's walls
    #[inline]
    pub fn walls(&self) -> &Walls {
        &self.walls
    }

    /// Returns position of the tile
    #[inline]
    pub fn pos(&self) -> Hex {
        self.pos
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
            !tile.walls.has(EdgeDirection::FLAT_TOP),
            "Wall should be removed"
        );

        tile.walls.add(EdgeDirection::FLAT_TOP);
        assert!(
            tile.walls.has(EdgeDirection::FLAT_TOP),
            "Wall should be added back"
        );
    }

    #[test]
    fn tile_copy() {
        let pos = Hex::new(-1, 2);
        let tile = HexTile::new(pos);

        // Test Copy trait
        let copied_tile = tile;
        assert_eq!(tile, copied_tile, "Copied tile should equal original");

        // Verify both tiles are still usable
        assert_eq!(
            tile.pos, copied_tile.pos,
            "Positions should match after copy"
        );
        assert_eq!(
            tile.walls, copied_tile.walls,
            "Walls should match after copy"
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
