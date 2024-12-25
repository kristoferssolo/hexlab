#[cfg(feature = "bevy_reflect")]
use bevy::prelude::*;
use hexx::EdgeDirection;

/// A bit-flag representation of walls in a hexagonal tile.
///
/// `Walls` uses an efficient bit-flag system to track the presence or absence of walls
/// along each edge of a hexagonal tile. Each of the six possible walls is represented
/// by a single bit in an 8-bit integer, allowing for fast operations and minimal memory usage.
///
/// # Examples
///
/// Creating and manipulating walls:
/// ```
/// use hexlab::prelude::*;
///
/// // Create a hexagon with all walls
/// let walls = Walls::new();
/// assert!(walls.is_closed());
///
/// // Create a hexagon with no walls
/// let mut walls = Walls::empty();
/// assert!(walls.is_empty());
///
/// // Add specific walls
/// walls.add(EdgeDirection::FLAT_NORTH);
/// walls.add(EdgeDirection::FLAT_SOUTH);
/// assert_eq!(walls.count(), 2);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "bevy", reflect(Component))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Walls(u8);

impl Walls {
    /// Creates a new set of walls with all edges closed.
    ///
    /// This is the default state where all six edges of the hexagon have walls.
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new set of walls with no edges (completely open).
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Checks if the walls are currently empty (no walls present).
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Adds a wall in the specified direction.
    ///
    /// # Arguments
    ///
    /// 0 `direction` - The direction in which to add the wall.
    #[cfg_attr(not(debug_assertions), inline)]
    pub fn add<T>(&mut self, direction: T)
    where
        T: Into<Self> + Copy,
    {
        self.0 |= direction.into().0;
    }

    /// Removes a wall in the specified direction.
    ///
    /// # Arguments
    ///
    /// - `direction` - The direction from which to remove the wall.
    #[cfg_attr(not(debug_assertions), inline)]
    pub fn remove<T>(&mut self, direction: T) -> bool
    where
        T: Into<Self> + Copy,
    {
        let was_removed = self.contains(direction);
        if was_removed {
            self.0 &= !direction.into().0;
        }
        was_removed
    }

    /// Checks if there is a wall in the specified direction.
    ///
    /// # Arguments
    ///
    /// - `other` - The direction to check for a wall.
    #[cfg_attr(not(debug_assertions), inline)]
    pub fn contains<T>(&self, other: T) -> bool
    where
        T: Into<Self> + Copy,
    {
        self.0 & other.into().0 != 0
    }

    /// Returns the raw bit representation of the walls
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    pub const fn as_bits(&self) -> u8 {
        self.0
    }

    /// Returns the total number of walls present
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    pub fn count(&self) -> u8 {
        u8::try_from(self.0.count_ones()).unwrap_or_default()
    }

    /// Returns a `Walls` value representing all possible directions.
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    pub const fn all_directions() -> Self {
        Self(0b11_1111)
    }

    /// Toggles a wall in the specified direction.
    ///
    /// If a wall exists in the given direction, it will be removed.
    /// If no wall exists, one will be added.
    ///
    /// # Arguments
    ///
    /// - `direction` - The direction in which to toggle the wall.
    ///
    /// # Returns
    ///
    /// The previous state (`true` if a wall was present before toggling, `false` otherwise).
    pub fn toggle<T>(&mut self, direction: T) -> bool
    where
        T: Into<Self> + Copy,
    {
        let is_present = self.contains(direction);
        if is_present {
            self.remove(direction);
        } else {
            self.add(direction);
        }
        is_present
    }

    /// Checks if walls are present in all six directions.
    ///
    /// # Returns
    ///
    /// `true` if the hexagon has all possible walls, making it completely enclosed.
    ///
    /// # Deprecated
    ///
    /// This method is deprecated since version 0.3.1. Use `is_enclosed()` instead.
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    #[deprecated(since = "0.3.1", note = "use `walls::Walls::is_enclosed()`")]
    pub fn is_closed(&self) -> bool {
        self.is_enclosed()
    }

    /// Checks if walls are present in all six directions.
    ///
    /// # Returns
    ///
    /// `true` if the hexagon has all possible walls, making it completely enclosed.
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    pub fn is_enclosed(&self) -> bool {
        self.count() == 6
    }

    /// Sets walls for multiple directions at once.
    ///
    /// This method efficiently adds multiple walls in a single operation while
    /// preserving any existing walls not specified in the input.
    ///
    /// # Arguments
    ///
    /// - `other` - The walls to add, specified as a `Walls` instance or any type
    ///   that can be converted into `Walls`.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// let mut walls = Walls::empty();
    /// walls.fill([EdgeDirection::FLAT_NORTH ,EdgeDirection::FLAT_SOUTH, EdgeDirection::FLAT_SOUTH_EAST]);
    ///
    /// assert!(walls.contains(EdgeDirection::FLAT_SOUTH));
    /// assert_eq!(walls.count(), 3);
    /// ```
    #[cfg_attr(not(debug_assertions), inline)]
    pub fn fill<T>(&mut self, other: T)
    where
        T: Into<Self>,
    {
        self.0 |= other.into().0;
    }
}

impl From<EdgeDirection> for Walls {
    fn from(value: EdgeDirection) -> Self {
        Self(1 << value.index())
    }
}

impl From<u8> for Walls {
    fn from(value: u8) -> Self {
        Self(1 << value)
    }
}

impl FromIterator<EdgeDirection> for Walls {
    fn from_iter<T: IntoIterator<Item = EdgeDirection>>(iter: T) -> Self {
        let mut walls = 0u8;
        for direction in iter {
            walls |= 1 << direction.index();
        }
        Self(walls)
    }
}

impl<const N: usize> From<[EdgeDirection; N]> for Walls {
    fn from(value: [EdgeDirection; N]) -> Self {
        value.into_iter().collect()
    }
}

impl Default for Walls {
    fn default() -> Self {
        Self(0b11_1111)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // all_directions
    #[test]
    fn all_directions_creates_closed_walls() {
        let walls = Walls::all_directions();
        assert!(walls.is_closed());
        assert!(!walls.is_empty());
        assert_eq!(walls.as_bits(), 0b111111);
    }

    // as_bits
    #[test]
    fn as_bits_empty() {
        let walls = Walls::empty();
        assert_eq!(walls.as_bits(), 0);
    }

    #[test]
    fn as_bits_single_wall() {
        let mut walls = Walls::empty();
        walls.add(EdgeDirection::FLAT_NORTH);
        assert_eq!(walls.as_bits(), 0b010000);
    }

    #[test]
    fn as_bits_multiple_walls() {
        let mut walls = Walls::empty();
        walls.add(EdgeDirection::FLAT_NORTH);
        walls.add(EdgeDirection::FLAT_SOUTH);
        assert_eq!(walls.as_bits(), 0b010010);
    }

    #[test]
    fn as_bits_all_walls() {
        let walls = Walls::new();
        assert_eq!(walls.as_bits(), 0b111111);
    }

    // new
    #[test]
    fn new_created_closed_walls() {
        let walls = Walls::new();
        assert!(walls.is_closed());
        assert_eq!(walls.as_bits(), 0b111111);
    }

    // empty
    #[test]
    fn empty_creates_no_walls() {
        let walls = Walls::empty();
        assert!(walls.is_empty());
        assert_eq!(walls.as_bits(), 0);
    }

    // add
    #[test]
    fn add_single_wall() {
        let mut walls = Walls::empty();
        walls.add(EdgeDirection::FLAT_NORTH);
        assert!(walls.contains(EdgeDirection::FLAT_NORTH));
        assert_eq!(walls.count(), 1);
    }

    // remove
    #[test]
    fn remove_existing_wall() {
        let mut walls = Walls::new();
        assert!(walls.remove(EdgeDirection::FLAT_NORTH));
        assert!(!walls.contains(EdgeDirection::FLAT_NORTH));
    }

    #[test]
    fn remove_nonexistent_wall() {
        let mut walls = Walls::empty();
        assert!(!walls.remove(EdgeDirection::FLAT_NORTH));
        walls.add(EdgeDirection::FLAT_NORTH);
        assert!(walls.remove(EdgeDirection::FLAT_NORTH));
    }

    // toggle
    #[test]
    fn toggle_adds_wall() {
        let mut walls = Walls::empty();
        assert!(!walls.toggle(EdgeDirection::FLAT_NORTH));
        assert!(walls.contains(EdgeDirection::FLAT_NORTH));
    }

    #[test]
    fn toggle_removes_wall() {
        let mut walls = Walls::new();
        assert!(walls.toggle(EdgeDirection::FLAT_NORTH));
        assert!(!walls.contains(EdgeDirection::FLAT_NORTH));
    }

    // fill
    #[test]
    fn fill_adds_multiple_walls() {
        let mut walls = Walls::empty();
        walls.fill([EdgeDirection::FLAT_NORTH, EdgeDirection::FLAT_SOUTH]);
        assert!(walls.contains(EdgeDirection::FLAT_NORTH));
        assert!(walls.contains(EdgeDirection::FLAT_SOUTH));
        assert_eq!(walls.count(), 2);
    }

    #[test]
    fn fill_preserves_existing_walls() {
        let mut walls = Walls::empty();
        walls.add(EdgeDirection::FLAT_NORTH);
        walls.fill([EdgeDirection::FLAT_SOUTH, EdgeDirection::FLAT_SOUTH_EAST]);
        assert!(walls.contains(EdgeDirection::FLAT_NORTH));
        assert!(walls.contains(EdgeDirection::FLAT_SOUTH));
        assert!(walls.contains(EdgeDirection::FLAT_SOUTH_EAST));
        assert_eq!(walls.count(), 3);
    }

    #[test]
    fn from_edge_direction_conversion() {
        let walls: Walls = EdgeDirection::FLAT_NORTH.into();
        assert!(walls.contains(EdgeDirection::FLAT_NORTH));
        assert_eq!(walls.count(), 1);
    }

    #[test]
    fn from_u8_conversion() {
        let walls: Walls = 0u8.into();
        assert!(walls.contains(EdgeDirection::FLAT_SOUTH_EAST));
        assert_eq!(walls.count(), 1);
    }

    #[test]
    fn from_array_conversion() {
        let walls: Walls = [EdgeDirection::FLAT_NORTH, EdgeDirection::FLAT_SOUTH].into();
        assert!(walls.contains(EdgeDirection::FLAT_NORTH));
        assert!(walls.contains(EdgeDirection::FLAT_SOUTH));
        assert_eq!(walls.count(), 2);
    }

    #[test]
    fn from_iterator_handles_duplicates() {
        let directions = vec![
            EdgeDirection::FLAT_NORTH,
            EdgeDirection::FLAT_SOUTH,
            EdgeDirection::FLAT_NORTH, // Duplicate
        ];
        let walls: Walls = directions.into_iter().collect();
        assert_eq!(walls.count(), 2);
    }

    #[test]
    fn default_creates_closed_walls() {
        let walls = Walls::default();
        assert!(walls.is_closed());
        assert_eq!(walls.as_bits(), 0b111111);
    }

    #[test]
    fn from_iterator() {
        let directions = vec![
            EdgeDirection::FLAT_NORTH,
            EdgeDirection::FLAT_SOUTH,
            EdgeDirection::FLAT_NORTH, // Duplicate should not affect result
        ];
        let walls: Walls = directions.into_iter().collect();
        assert_eq!(walls.count(), 2);
        assert!(walls.contains(EdgeDirection::FLAT_NORTH));
        assert!(walls.contains(EdgeDirection::FLAT_SOUTH));
    }

    #[test]
    fn bit_manipulation() {
        let mut walls = Walls::empty();

        // Test single bit operations
        walls.add(EdgeDirection::FLAT_NORTH);
        assert_eq!(walls.as_bits(), 0b010000);

        walls.add(EdgeDirection::FLAT_SOUTH);
        assert_eq!(walls.as_bits(), 0b010010);

        // Test removing middle bit
        walls.add(EdgeDirection::FLAT_SOUTH_EAST);
        assert_eq!(walls.as_bits(), 0b010011);
        walls.remove(EdgeDirection::FLAT_SOUTH);
        assert_eq!(walls.as_bits(), 0b010001);
    }

    // From<EdgeDirection> tests
    #[test]
    fn from_edge_direction_flat_south_east() {
        let walls = Walls::from(EdgeDirection::FLAT_SOUTH_EAST);
        assert_eq!(walls.as_bits(), 0b000001);
    }

    #[test]
    fn from_edge_direction_flat_south() {
        let walls = Walls::from(EdgeDirection::FLAT_SOUTH);
        assert_eq!(walls.as_bits(), 0b000010);
    }

    #[test]
    fn from_edge_direction_flat_south_west() {
        let walls = Walls::from(EdgeDirection::FLAT_SOUTH_WEST);
        assert_eq!(walls.as_bits(), 0b000100);
    }

    #[test]
    fn from_edge_direction_flat_north_west() {
        let walls = Walls::from(EdgeDirection::FLAT_NORTH_WEST);
        assert_eq!(walls.as_bits(), 0b001000);
    }

    #[test]
    fn from_edge_direction_flat_north() {
        let walls = Walls::from(EdgeDirection::FLAT_NORTH);
        assert_eq!(walls.as_bits(), 0b010000);
    }

    #[test]
    fn from_edge_direction_flat_east() {
        let walls = Walls::from(EdgeDirection::FLAT_NORTH_EAST);
        assert_eq!(walls.as_bits(), 0b100000);
    }

    // FromIterator tests
    #[test]
    fn from_iterator_empty() {
        let walls = Vec::new().into_iter().collect::<Walls>();
        assert!(walls.is_empty());
    }

    #[test]
    fn from_iterator_single() {
        let walls = vec![EdgeDirection::FLAT_SOUTH]
            .into_iter()
            .collect::<Walls>();
        assert_eq!(walls.as_bits(), 0b000010);
    }

    #[test]
    fn from_iterator_multiple() {
        let walls = vec![EdgeDirection::FLAT_NORTH, EdgeDirection::FLAT_SOUTH]
            .into_iter()
            .collect::<Walls>();
        assert_eq!(walls.as_bits(), 0b010010);
    }

    #[test]
    fn from_iterator_duplicates() {
        let walls = vec![
            EdgeDirection::FLAT_NORTH,
            EdgeDirection::FLAT_NORTH,
            EdgeDirection::FLAT_SOUTH,
        ]
        .into_iter()
        .collect::<Walls>();
        assert_eq!(walls.as_bits(), 0b010010);
    }

    #[test]
    fn from_iterator_all_directions() {
        let walls = EdgeDirection::iter().collect::<Walls>();
        assert_eq!(walls.as_bits(), 0b111111);
    }

    // From<[EdgeDirection; N]> tests
    #[test]
    fn from_array_empty() {
        let walls = Walls::from([]);
        assert!(walls.is_empty());
    }

    #[test]
    fn from_array_single() {
        let walls = Walls::from([EdgeDirection::FLAT_NORTH]);
        assert_eq!(walls.as_bits(), 0b010000);
    }

    #[test]
    fn from_array_multiple() {
        let walls = Walls::from([EdgeDirection::FLAT_NORTH, EdgeDirection::FLAT_SOUTH]);
        assert_eq!(walls.as_bits(), 0b010010);
    }

    #[test]
    fn from_array_duplicates() {
        let walls = Walls::from([
            EdgeDirection::FLAT_NORTH,
            EdgeDirection::FLAT_NORTH,
            EdgeDirection::FLAT_SOUTH,
        ]);
        assert_eq!(walls.as_bits(), 0b010010);
    }

    #[test]
    fn from_array_all_directions() {
        let walls = Walls::from([
            EdgeDirection::FLAT_NORTH,
            EdgeDirection::FLAT_NORTH_EAST,
            EdgeDirection::FLAT_SOUTH_EAST,
            EdgeDirection::FLAT_SOUTH,
            EdgeDirection::FLAT_SOUTH_WEST,
            EdgeDirection::FLAT_NORTH_WEST,
        ]);
        assert_eq!(walls.as_bits(), 0b111111);
    }
}
