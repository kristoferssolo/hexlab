use std::collections::HashSet;

use hexx::{EdgeDirection, Hex};
use rand::{seq::SliceRandom, thread_rng, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::HexMaze;

#[derive(Debug, Clone, Copy)]
pub enum GeneratorType {
    BackTracking,
}

impl HexMaze {
    pub fn generate(&mut self, generator_type: GeneratorType) {
        match generator_type {
            GeneratorType::BackTracking => self.generate_backtracking(),
        }
    }

    pub fn generate_from_seed(&mut self, generator_type: GeneratorType, seed: u64) {
        match generator_type {
            GeneratorType::BackTracking => self.generate_backtracking_from_seed(seed),
        }
    }

    pub fn generate_backtracking(&mut self) {
        if self.is_empty() {
            return;
        }
        let start = *self.keys().next().unwrap();

        let mut visited = HashSet::new();
        let mut rng = thread_rng();
        self.recursive_backtrack(start, &mut visited, &mut rng);
    }

    pub fn generate_backtracking_from_seed(&mut self, seed: u64) {
        if self.is_empty() {
            return;
        }
        // let start = *self.keys().next().unwrap();
        let start = Hex::ZERO;

        let mut visited = HashSet::new();
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        self.recursive_backtrack(start, &mut visited, &mut rng);
    }

    fn recursive_backtrack<R: Rng>(
        &mut self,
        current: Hex,
        visited: &mut HashSet<Hex>,
        rng: &mut R,
    ) {
        visited.insert(current);

        let mut directions = EdgeDirection::ALL_DIRECTIONS;
        directions.shuffle(rng);

        for direction in directions {
            let neighbor = current + direction;

            if self.get_tile(&neighbor).is_some() && !visited.contains(&neighbor) {
                self.remove_tile_wall(&current, direction);
                self.remove_tile_wall(&neighbor, direction.const_neg());

                self.recursive_backtrack(neighbor, visited, rng);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backtracking_generation() {
        let mut maze = HexMaze::with_radius(2);

        // Before generation
        for tile in maze.values() {
            assert_eq!(tile.walls.as_bits(), 0b111111);
        }

        // Generate using backtracking
        maze.generate(GeneratorType::BackTracking);

        // After generation
        let all_walls = maze.values().all(|tile| tile.walls.as_bits() == 0b111111);
        assert!(!all_walls, "Some walls should be removed");
    }

    #[test]
    fn empty_maze() {
        let mut maze = HexMaze::default();
        maze.generate(GeneratorType::BackTracking);
        assert!(maze.is_empty(), "Empty maze should remain empty");
    }
}
