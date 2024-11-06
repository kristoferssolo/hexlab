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
    fn new_walls() {
        let walls = Walls::new();
        // All walls should be present by default
        for direction in EdgeDirection::iter() {
            assert!(
                walls.has(direction),
                "Wall should exist in direction {:?}",
                direction
            );
        }
    }

    #[test]
    fn add_remove_single_wall() {
        let mut walls = Walls::new();

        // Remove and verify each wall
        walls.remove(EdgeDirection::FLAT_TOP);
        assert!(!walls.has(EdgeDirection::FLAT_TOP));

        // Add back and verify
        walls.add(EdgeDirection::FLAT_TOP);
        assert!(walls.has(EdgeDirection::FLAT_TOP));
    }

    #[test]
    fn multiple_operations() {
        let mut walls = Walls::new();

        // Remove multiple walls
        walls.remove(EdgeDirection::FLAT_TOP);
        walls.remove(EdgeDirection::FLAT_BOTTOM);

        // Verify removed walls
        assert!(!walls.has(EdgeDirection::FLAT_TOP));
        assert!(!walls.has(EdgeDirection::FLAT_BOTTOM));

        // Verify other walls still exist
        assert!(walls.has(EdgeDirection::FLAT_TOP_RIGHT));
        assert!(walls.has(EdgeDirection::FLAT_TOP_LEFT));

        // Add back one wall
        walls.add(EdgeDirection::FLAT_TOP);
        assert!(walls.has(EdgeDirection::FLAT_TOP));
        assert!(!walls.has(EdgeDirection::FLAT_BOTTOM));
    }

    #[test]
    fn bit_patterns() {
        let mut walls = Walls::new();
        assert_eq!(
            walls.as_bits(),
            0b111111,
            "Initial state should have all walls"
        );

        walls.remove(EdgeDirection::FLAT_BOTTOM_RIGHT);
        assert_eq!(walls.as_bits() & 0b000001, 0, "First bit should be cleared");

        walls.add(EdgeDirection::FLAT_BOTTOM_RIGHT);
        assert_eq!(walls.as_bits() & 0b000001, 1, "First bit should be set");
    }

    #[test]
    fn remove_all_walls() {
        let mut walls = Walls::new();

        // Remove all walls
        for direction in EdgeDirection::iter() {
            walls.remove(direction);
        }

        // Verify all walls are removed
        assert_eq!(walls.as_bits(), 0, "All walls should be removed");

        // Verify each direction
        for direction in EdgeDirection::iter() {
            assert!(
                !walls.has(direction),
                "No wall should exist in direction {:?}",
                direction
            );
        }
    }

    #[test]
    fn deref_operations() {
        let mut walls = Walls::new();

        // Test Deref
        let bits: &u8 = walls.deref();
        assert_eq!(*bits, 0b111111);

        // Test DerefMut
        *walls.deref_mut() = 0;
        assert_eq!(walls.as_bits(), 0);
    }

    #[test]
    fn idempotent_operations() {
        let mut walls = Walls::new();

        // Adding twice shouldn't change the result
        walls.add(EdgeDirection::FLAT_TOP);
        let first_add = walls.as_bits();
        walls.add(EdgeDirection::FLAT_TOP);
        assert_eq!(walls.as_bits(), first_add);

        // Removing twice shouldn't change the result
        walls.remove(EdgeDirection::FLAT_TOP);
        let first_remove = walls.as_bits();
        walls.remove(EdgeDirection::FLAT_TOP);
        assert_eq!(walls.as_bits(), first_remove);
    }
}
