use std::ops::{Deref, DerefMut};

use hexx::EdgeDirection;

/// Represents the walls of a hexagonal tile using bit flags
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Walls(u8);

impl Walls {
    /// Creates a new walls configuration with all walls
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a wall in the specified direction
    #[inline]
    pub fn add(&mut self, direction: EdgeDirection) {
        self.0 |= Self::from(direction).0
    }

    /// Removes a wall in the specified direction
    #[inline]
    pub fn remove(&mut self, direction: EdgeDirection) {
        self.0 &= !Self::from(direction).0
    }

    /// Returns true if there is a wall in the specified direction
    #[inline]
    pub fn has(&self, direction: EdgeDirection) -> bool {
        self.0 & Self::from(direction).0 != 0
    }

    /// Returns the raw bit representation of the walls
    #[inline]
    pub fn as_bits(&self) -> u8 {
        self.0
    }
}

impl From<EdgeDirection> for Walls {
    fn from(value: EdgeDirection) -> Self {
        let bits = 1 << value.index();
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn walls() {
        let mut walls = Walls::new();
        assert!(walls.has(EdgeDirection::FLAT_TOP));

        walls.remove(EdgeDirection::FLAT_TOP);
        assert!(!walls.has(EdgeDirection::FLAT_TOP));

        walls.add(EdgeDirection::FLAT_TOP);
        assert!(walls.has(EdgeDirection::FLAT_TOP));
    }
}
