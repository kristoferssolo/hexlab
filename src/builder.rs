use crate::{GeneratorType, HexMaze};
use hexx::Hex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MazeBuilderError {
    /// Occurs when attempting to build a maze without specifying a radius.
    #[error("Radius must be specified to build a maze")]
    NoRadius,

    /// Occurs when the specified radius is too large.
    #[error("Radius {0} is too large. Maximum allowed radius is {1}")]
    RadiusTooLarge(u32, u32),

    /// Occurs when the specified start position is outside the maze bounds.
    #[error("Start position {0:?} is outside maze bounds")]
    InvalidStartPosition(Hex),

    /// Occurs when maze generation fails.
    #[error("Failed to generate maze: {0}")]
    GenerationError(String),
}

/// A builder pattern for creating hexagonal mazes.
///
/// This struct provides a fluent interface for configuring and building hexagonal mazes.
/// It offers flexibility in specifying the maze size, random seed, generation algorithm,
/// and starting position.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use hexlab::prelude::*;
///
/// let maze = MazeBuilder::new()
///     .with_radius(5)
///     .build()
///     .expect("Failed to create maze");
///
/// // A radius of 5 creates 61 hexagonal tiles
/// assert!(!maze.is_empty());
/// assert_eq!(maze.len(), 91);
/// ```
///
/// Using a seed for reproducible results:
/// ```
/// use hexlab::prelude::*;
///
/// let maze1 = MazeBuilder::new()
///     .with_radius(3)
///     .with_seed(12345)
///     .build()
///     .expect("Failed to create maze");
///
/// let maze2 = MazeBuilder::new()
///     .with_radius(3)
///     .with_seed(12345)
///     .build()
///     .expect("Failed to create maze");
///
/// // Same seed should produce identical mazes
/// assert_eq!(maze1.len(), maze2.len());
/// assert_eq!(maze1, maze2);
/// ```
///
/// Specifying a custom generator:
/// ```
/// use hexlab::prelude::*;
///
/// let maze = MazeBuilder::new()
///     .with_radius(7)
///     .with_generator(GeneratorType::RecursiveBacktracking)
///     .build()
///     .expect("Failed to create maze");
/// ```
#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct MazeBuilder {
    radius: Option<u16>,
    seed: Option<u64>,
    generator_type: GeneratorType,
    start_position: Option<Hex>,
}

impl MazeBuilder {
    /// Creates a new [`MazeBuilder`] instance with default settings.
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the radius for the hexagonal maze.
    ///
    /// The radius determines the size of the maze, specifically the number of tiles
    /// from the center (0,0) to the edge of the hexagon, not including the center tile.
    /// For example, a radius of 3 would create a maze with 3 tiles from center to edge,
    /// resulting in a total diameter of 7 tiles (3 + 1 + 3).
    ///
    /// # Arguments
    ///
    /// - `radius` - The number of tiles from the center to the edge of the hexagon.
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    pub const fn with_radius(mut self, radius: u16) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Sets the random seed for maze generation.
    ///
    /// Using the same seed will produce identical mazes, allowing for reproducible results.
    ///
    /// # Arguments
    ///
    /// - `seed` - The random seed value.
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    pub const fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Sets the generator algorithm for maze creation.
    ///
    /// Different generators may produce different maze patterns and characteristics.
    ///
    /// # Arguments
    ///
    /// - `generator_type` - The maze generation algorithm to use.
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    pub const fn with_generator(mut self, generator_type: GeneratorType) -> Self {
        self.generator_type = generator_type;
        self
    }
    /// Sets the starting position for maze generation.
    ///
    /// # Arguments
    ///
    /// - `pos` - The hexagonal coordinates for the starting position.
    #[cfg_attr(not(debug_assertions), inline)]
    #[must_use]
    pub const fn with_start_position(mut self, pos: Hex) -> Self {
        self.start_position = Some(pos);
        self
    }

    /// Builds the hexagonal maze based on the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns [`MazeBuilderError::NoRadius`] if no radius is specified.
    /// Returns [`MazeBuilderError::InvalidStartPosition`] if the start position is outside maze bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexlab::prelude::*;
    ///
    /// // Should fail without radius
    /// let result = MazeBuilder::new().build();
    /// assert!(result.is_err());
    ///
    /// // Should succeed with radius
    /// let result = MazeBuilder::new()
    ///     .with_radius(3)
    ///     .build();
    /// assert!(result.is_ok());
    ///
    /// let maze = result.unwrap();
    /// assert!(!maze.is_empty());
    /// ```
    pub fn build(self) -> Result<HexMaze, MazeBuilderError> {
        let radius = self.radius.ok_or(MazeBuilderError::NoRadius)?;
        let mut maze = create_hex_maze(radius);

        if let Some(start_pos) = self.start_position {
            if maze.get_tile(&start_pos).is_none() {
                return Err(MazeBuilderError::InvalidStartPosition(start_pos));
            }
        }

        if !maze.is_empty() {
            self.generator_type
                .generate(&mut maze, self.start_position, self.seed);
        }

        Ok(maze)
    }
}
pub fn create_hex_maze(radius: u16) -> HexMaze {
    let mut maze = HexMaze::new();
    let radius = i32::from(radius);

    for q in -radius..=radius {
        let r1 = (-radius).max(-q - radius);
        let r2 = radius.min(-q + radius);
        for r in r1..=r2 {
            let pos = Hex::new(q, r);
            maze.add_tile(pos);
        }
    }

    maze
}

#[cfg(test)]
mod test {
    use super::*;
    use claims::assert_gt;
    use rstest::rstest;

    #[test]
    fn maze_builder_new() {
        let builder = MazeBuilder::new();
        assert_eq!(builder.radius, None);
        assert_eq!(builder.seed, None);
        assert_eq!(builder.generator_type, GeneratorType::default());
        assert_eq!(builder.start_position, None);
    }

    #[rstest]
    #[case(0, 1)] // Minimum size is 1 tile
    #[case(1, 7)]
    #[case(2, 19)]
    #[case(3, 37)]
    #[case(10, 331)]
    #[case(100, 30301)]
    fn create_hex_maze_various_radii(#[case] radius: u16, #[case] expected_size: usize) {
        let maze = create_hex_maze(radius);
        assert_eq!(maze.len(), expected_size);
    }

    #[test]
    fn create_hex_maze_large_radius() {
        let large_radius = 1000;
        let maze = create_hex_maze(large_radius);
        assert_gt!(maze.len(), 0);

        // Calculate expected size for this radius
        let expected_size = 3 * (large_radius as usize).pow(2) + 3 * large_radius as usize + 1;
        assert_eq!(maze.len(), expected_size);
    }

    #[test]
    fn create_hex_maze_tile_positions() {
        let maze = create_hex_maze(2);
        let expected_positions = [
            Hex::new(0, 0),
            Hex::new(1, -1),
            Hex::new(1, 0),
            Hex::new(0, 1),
            Hex::new(-1, 1),
            Hex::new(-1, 0),
            Hex::new(0, -1),
            Hex::new(2, -2),
            Hex::new(2, -1),
            Hex::new(2, 0),
            Hex::new(1, 1),
            Hex::new(0, 2),
            Hex::new(-1, 2),
            Hex::new(-2, 2),
            Hex::new(-2, 1),
            Hex::new(-2, 0),
            Hex::new(-1, -1),
            Hex::new(0, -2),
            Hex::new(1, -2),
        ];
        for pos in expected_positions.iter() {
            assert!(maze.get_tile(pos).is_some(), "Expected tile at {:?}", pos);
        }
    }
}
