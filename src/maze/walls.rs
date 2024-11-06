use std::ops::{Deref, DerefMut};

use hexx::EdgeDirection;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Walls(u8);

impl Walls {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, direction: EdgeDirection) {
        self.0 |= Self::from(direction).0
    }

    pub fn remove(&mut self, direction: EdgeDirection) {
        self.0 &= !Self::from(direction).0
    }

    pub fn has(&self, direction: EdgeDirection) -> bool {
        self.0 & Self::from(direction).0 != 0
    }
}

impl From<EdgeDirection> for Walls {
    fn from(value: EdgeDirection) -> Self {
        let bits = match value.index() {
            0 => 0b000001,
            1 => 0b000010,
            2 => 0b000011,
            3 => 0b000100,
            4 => 0b000101,
            5 => 0b000110,
            _ => unreachable!(),
        };
        Self(bits)
    }
}

impl Deref for Walls {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Walls {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Walls {
    fn default() -> Self {
        Self(0b111111)
    }
}
