use std::collections::HashMap;

use hexx::{EdgeDirection, Hex, HexLayout};

use super::{HexTile, Walls};

pub struct HexMaze {
    pub tiles: HashMap<Hex, HexTile>,
    pub layout: HexLayout,
}

impl HexMaze {
    pub fn new(layout: HexLayout) -> Self {
        Self {
            tiles: HashMap::new(),
            layout,
        }
    }

    pub fn add_tile(&mut self, coords: Hex) {
        let tile = HexTile::new(coords);
        self.tiles.insert(coords, tile);
    }

    pub fn add_wall(&mut self, coord: Hex, direction: EdgeDirection) {
        if let Some(tile) = self.tiles.get_mut(&coord) {
            tile.walls.add(direction)
        }
    }

    pub fn get_tile(&self, coord: &Hex) -> Option<&HexTile> {
        self.tiles.get(coord)
    }

    pub fn get_walls(&self, coord: &Hex) -> Option<&Walls> {
        self.tiles.get(coord).map(|tile| &tile.walls)
    }
}
