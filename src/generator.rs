use std::collections::HashSet;

use hexx::{EdgeDirection, Hex};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

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

    pub fn generate_backtracking(&mut self) {
        if self.is_empty() {
            return;
        }
        let start = *self.tiles().keys().next().unwrap();

        let mut visited = HashSet::new();
        let mut rng = thread_rng();
        self.recursive_backtrack(start, &mut visited, &mut rng);
    }

    fn recursive_backtrack(
        &mut self,
        current: Hex,
        visited: &mut HashSet<Hex>,
        rng: &mut ThreadRng,
    ) {
        visited.insert(current);

        let mut directions = EdgeDirection::ALL_DIRECTIONS;
        directions.shuffle(rng);

        for direction in directions {
            let neighbor = current + direction;

            if let Some(_) = self.get_tile(&neighbor) {
                if !visited.contains(&neighbor) {
                    self.remove_tile_wall(&current, direction);
                    self.remove_tile_wall(&neighbor, direction.const_neg());

                    self.recursive_backtrack(neighbor, visited, rng);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use hexx::HexLayout;

    use super::*;

    #[test]
    fn backtracking_generation() {
        let mut maze = HexMaze::default().with_radius(2);

        // Before generation
        for tile in maze.tiles.values() {
            assert_eq!(tile.walls.as_bits(), 0b111111);
        }

        // Generate using backtracking
        maze.generate(GeneratorType::BackTracking);

        // After generation
        let all_walls = maze
            .tiles
            .values()
            .all(|tile| tile.walls.as_bits() == 0b111111);
        assert!(!all_walls, "Some walls should be removed");
    }

    #[test]
    fn empty_maze() {
        let mut maze = HexMaze::with_layout(HexLayout::default());
        maze.generate(GeneratorType::BackTracking);
        assert!(maze.is_empty(), "Empty maze should remain empty");
    }
}
