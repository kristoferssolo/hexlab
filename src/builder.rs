use hexx::Hex;
use thiserror::Error;

use crate::{
    generator::{generate_backtracking, GeneratorType},
    HexMaze,
};

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
/// It offers flexibility in specifying the maze size, random seed, and generation algorithm.
///
/// # Examples
///
/// Basic usage:
/// ```rust
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
/// ```rust
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
/// ```rust
/// use hexlab::prelude::*;
///
/// let maze = MazeBuilder::new()
///     .with_radius(7)
///     .with_generator(GeneratorType::RecursiveBacktracking)
///     .build()
///     .expect("Failed to create maze");
/// ```
#[derive(Default)]
pub struct MazeBuilder {
    radius: Option<u32>,
    seed: Option<u64>,
    generator_type: GeneratorType,
    start_position: Option<Hex>,
}

impl MazeBuilder {
    /// Creates a new [`MazeBuilder`] instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the radius for the hexagonal maze.
    ///
    /// # Arguments
    ///
    /// * `radius` - The size of the maze (number of tiles along one edge).
    #[inline]
    pub fn with_radius(mut self, radius: u32) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Sets the random seed for maze generation.
    ///
    /// # Arguments
    ///
    /// * `seed` - The random seed value.
    #[inline]
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Sets the generator algorithm for maze creation.
    ///
    /// Different generators may produce different maze patterns and characteristics.
    ///
    /// # Arguments
    ///
    /// * `generator_type` - The maze generation algorithm to use.
    #[inline]
    pub fn with_generator(mut self, generator_type: GeneratorType) -> Self {
        self.generator_type = generator_type;
        self
    }

    #[inline]
    pub fn with_start_position(mut self, pos: Hex) -> Self {
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
    /// ```rust
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
        let mut maze = self.create_hex_maze(radius);

        if let Some(start_pos) = self.start_position {
            if maze.get_tile(&start_pos).is_none() {
                return Err(MazeBuilderError::InvalidStartPosition(start_pos));
            }
        }

        if !maze.is_empty() {
            self.generate_maze(&mut maze);
        }

        Ok(maze)
    }

    fn create_hex_maze(&self, radius: u32) -> HexMaze {
        let mut maze = HexMaze::new();
        let radius = radius as i32;
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

    fn generate_maze(&self, maze: &mut HexMaze) {
        match self.generator_type {
            GeneratorType::RecursiveBacktracking => {
                generate_backtracking(maze, self.start_position, self.seed)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use hexx::EdgeDirection;

    use super::*;

    /// Helper function to count the number of tiles for a given radius
    fn calculate_hex_tiles(radius: u32) -> usize {
        let r = radius as i32;
        (3 * r * r + 3 * r + 1) as usize
    }

    #[test]
    fn new_builder() {
        let builder = MazeBuilder::new();
        assert!(builder.radius.is_none());
        assert!(builder.seed.is_none());
        assert!(builder.start_position.is_none());
    }

    #[test]
    fn builder_with_radius() {
        let radius = 5;
        let maze = MazeBuilder::new().with_radius(radius).build().unwrap();

        assert_eq!(maze.len(), calculate_hex_tiles(radius));
        assert!(maze.get_tile(&Hex::ZERO).is_some());
    }

    #[test]
    fn builder_without_radius() {
        let maze = MazeBuilder::new().build();
        assert!(matches!(maze, Err(MazeBuilderError::NoRadius)));
    }

    #[test]
    fn builder_with_seed() {
        let radius = 3;
        let seed = 12345;

        let maze1 = MazeBuilder::new()
            .with_radius(radius)
            .with_seed(seed)
            .build()
            .unwrap();

        let maze2 = MazeBuilder::new()
            .with_radius(radius)
            .with_seed(seed)
            .build()
            .unwrap();

        // Same seed should produce identical mazes
        assert_eq!(maze1, maze2);
    }

    #[test]
    fn different_seeds_produce_different_mazes() {
        let radius = 3;

        let maze1 = MazeBuilder::new()
            .with_radius(radius)
            .with_seed(12345)
            .build()
            .unwrap();

        let maze2 = MazeBuilder::new()
            .with_radius(radius)
            .with_seed(54321)
            .build()
            .unwrap();

        // Different seeds should produce different mazes
        assert_ne!(maze1, maze2);
    }

    #[test]
    fn maze_connectivity() {
        let radius = 3;
        let maze = MazeBuilder::new().with_radius(radius).build().unwrap();

        // Helper function to count accessible neighbors
        fn count_accessible_neighbors(maze: &HexMaze, pos: Hex) -> usize {
            EdgeDirection::ALL_DIRECTIONS
                .iter()
                .filter(|&&dir| {
                    let neighbor = pos + dir;
                    if let Some(walls) = maze.get_walls(&pos) {
                        !walls.contains(dir) && maze.get_tile(&neighbor).is_some()
                    } else {
                        false
                    }
                })
                .count()
        }

        // Check that each tile has at least one connection
        for &pos in maze.keys() {
            let accessible_neighbors = count_accessible_neighbors(&maze, pos);
            assert!(
                accessible_neighbors > 0,
                "Tile at {:?} has no accessible neighbors",
                pos
            );
        }
    }

    #[test]
    fn start_position() {
        let radius = 3;
        let start_pos = Hex::new(1, 1);

        let maze = MazeBuilder::new()
            .with_radius(radius)
            .with_start_position(start_pos)
            .build()
            .unwrap();

        assert!(maze.get_tile(&start_pos).is_some());
    }

    #[test]
    fn invalid_start_position() {
        let maze = MazeBuilder::new()
            .with_radius(3)
            .with_start_position(Hex::new(10, 10))
            .build();

        assert!(matches!(
            maze,
            Err(MazeBuilderError::InvalidStartPosition(_))
        ));
    }

    #[test]
    fn maze_boundaries() {
        let radius = 3;
        let maze = MazeBuilder::new().with_radius(radius).build().unwrap();

        // Test that tiles exist within the radius
        for q in -(radius as i32)..=(radius as i32) {
            for r in -(radius as i32)..=(radius as i32) {
                let pos = Hex::new(q, r);
                if q.abs() + r.abs() <= radius as i32 {
                    assert!(
                        maze.get_tile(&pos).is_some(),
                        "Expected tile at {:?} to exist",
                        pos
                    );
                }
            }
        }
    }

    #[test]
    fn different_radii() {
        for radius in 1..=5 {
            let maze = MazeBuilder::new().with_radius(radius).build().unwrap();

            assert_eq!(
                maze.len(),
                calculate_hex_tiles(radius),
                "Incorrect number of tiles for radius {}",
                radius
            );
        }
    }

    #[test]
    fn wall_consistency() {
        let radius = 3;
        let maze = MazeBuilder::new().with_radius(radius).build().unwrap();

        // Check that if tile A has no wall to tile B,
        // then tile B has no wall to tile A
        for &pos in maze.keys() {
            for &dir in &EdgeDirection::ALL_DIRECTIONS {
                let neighbor = pos + dir;
                if let (Some(walls), Some(neighbor_walls)) =
                    (maze.get_walls(&pos), maze.get_walls(&neighbor))
                {
                    assert_eq!(
                        walls.contains(dir),
                        neighbor_walls.contains(dir.const_neg()),
                        "Wall inconsistency between {:?} and {:?}",
                        pos,
                        neighbor
                    );
                }
            }
        }
    }
}
