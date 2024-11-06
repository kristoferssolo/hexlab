use hexx::Hex;

use super::Walls;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexTile {
    pub pos: Hex,
    pub walls: Walls,
}

impl HexTile {
    pub fn new(pos: Hex) -> Self {
        Self {
            pos,
            walls: Walls::default(),
        }
    }
}
