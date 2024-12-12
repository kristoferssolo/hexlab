#[cfg(feature = "bevy_reflect")]
use bevy::prelude::*;
use hexx::{EdgeDirection, Hex};
use rand::{seq::SliceRandom, thread_rng, Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

use crate::HexMaze;

#[allow(clippy::module_name_repetitions)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "bevy", reflect(Component))]
#[derive(Debug, Clone, Copy, Default)]
pub enum GeneratorType {
    #[default]
    RecursiveBacktracking,
}

pub fn generate_backtracking(maze: &mut HexMaze, start_pos: Option<Hex>, seed: Option<u64>) {
    if maze.is_empty() {
        return;
    }

    let start = start_pos.unwrap_or(Hex::ZERO);

    let mut visited = HashSet::new();

    let mut rng: Box<dyn RngCore> = seed.map_or_else(
        || Box::new(thread_rng()) as Box<dyn RngCore>,
        |seed| Box::new(ChaCha8Rng::seed_from_u64(seed)) as Box<dyn RngCore>,
    );
    recursive_backtrack(maze, start, &mut visited, &mut rng);
}

fn recursive_backtrack<R: Rng>(
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
            recursive_backtrack(maze, neighbor, visited, rng);
        }
    }
}
