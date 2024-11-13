use std::collections::HashSet;

use hexx::{EdgeDirection, Hex};
use rand::{seq::SliceRandom, thread_rng, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::{generator::GeneratorType, HexMaze};

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
    /// Returns an error if no radius is specified.
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
    pub fn build(self) -> Result<HexMaze, String> {
        let radius = self.radius.ok_or("Radius must be specified")?;
        let mut maze = self.create_hex_maze(radius);

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
        match self.seed {
            Some(seed) => self.generate_from_seed(maze, seed),
            None => self.generate_backtracking(maze),
        }
    }

    fn generate_from_seed(&self, maze: &mut HexMaze, seed: u64) {
        if maze.is_empty() {
            return;
        }
        let start = Hex::ZERO;
        let mut visited = HashSet::new();
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        self.recursive_backtrack(maze, start, &mut visited, &mut rng);
    }

    fn generate_backtracking(&self, maze: &mut HexMaze) {
        if maze.is_empty() {
            return;
        }
        let start = *maze.keys().next().unwrap();
        let mut visited = HashSet::new();
        let mut rng = thread_rng();
        self.recursive_backtrack(maze, start, &mut visited, &mut rng);
    }

    fn recursive_backtrack<R: Rng>(
        &self,
        maze: &mut HexMaze,
        current: Hex,
        visited: &mut HashSet<Hex>,
        rng: &mut R,
    ) {
        visited.insert(current);
        let mut directions = EdgeDirection::ALL_DIRECTIONS;
        directions.shuffle(rng);

        for direction in directions {
            let neighbor = current + direction;
            if maze.get_tile(&neighbor).is_some() && !visited.contains(&neighbor) {
                maze.remove_tile_wall(&current, direction);
                maze.remove_tile_wall(&neighbor, direction.const_neg());
                self.recursive_backtrack(maze, neighbor, visited, rng);
            }
        }
    }
}
