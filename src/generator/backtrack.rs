use crate::Maze;
use hexx::{EdgeDirection, Hex};
use rand::{rngs::StdRng, seq::SliceRandom, thread_rng, Rng, RngCore, SeedableRng};
use std::collections::HashSet;

pub(super) fn generate_backtracking(maze: &mut Maze, start_pos: Option<Hex>, seed: Option<u64>) {
    if maze.is_empty() {
        return;
    }

    let start = start_pos.unwrap_or(Hex::ZERO);

    let mut visited = HashSet::new();

    let mut rng: Box<dyn RngCore> = seed.map_or_else(
        || Box::new(thread_rng()) as Box<dyn RngCore>,
        |seed| Box::new(StdRng::seed_from_u64(seed)) as Box<dyn RngCore>,
    );

    recursive_backtrack(maze, start, &mut visited, &mut rng);
}

fn recursive_backtrack<R: Rng>(
    maze: &mut Maze,
    current: Hex,
    visited: &mut HashSet<Hex>,
    rng: &mut R,
) {
    visited.insert(current);
    let mut directions = EdgeDirection::ALL_DIRECTIONS;
    directions.shuffle(rng);

    for direction in directions {
        let neighbor = current + direction;
        if maze.get(&neighbor).is_some() && !visited.contains(&neighbor) {
            let _ = maze.remove_tile_wall(&current, direction);
            let _ = maze.remove_tile_wall(&neighbor, direction.const_neg());
            recursive_backtrack(maze, neighbor, visited, rng);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::builder::create_hex_maze;
    use rstest::rstest;

    #[rstest]
    #[case(Hex::ZERO)]
    #[case(Hex::new(1, -1))]
    #[case(Hex::new(-2, 2))]
    fn recursive_backtrack_start_visited(#[case] start: Hex) {
        let mut maze = create_hex_maze(3);
        let mut rng = StdRng::seed_from_u64(12345);
        let mut visited = HashSet::new();

        recursive_backtrack(&mut maze, start, &mut visited, &mut rng);

        assert!(visited.contains(&start), "Start position should be visited");
    }

    #[rstest]
    #[case(Hex::ZERO)]
    #[case(Hex::new(1, -1))]
    #[case(Hex::new(-2, 2))]
    fn recursive_backtrack_walls_removed(#[case] start: Hex) {
        let mut maze = create_hex_maze(3);
        let mut rng = StdRng::seed_from_u64(12345);
        let mut visited = HashSet::new();

        recursive_backtrack(&mut maze, start, &mut visited, &mut rng);

        for &pos in maze.keys() {
            let walls = maze.get_walls(&pos).unwrap();
            assert!(
                walls.count() < 6,
                "At least one wall should be removed for each tile"
            );
        }
    }

    #[rstest]
    #[case(Hex::ZERO)]
    #[case(Hex::new(1, -1))]
    #[case(Hex::new(-2, 2))]
    fn recursive_backtrack_connectivity(#[case] start: Hex) {
        let mut maze = create_hex_maze(3);
        let mut rng = StdRng::seed_from_u64(12345);
        let mut visited = HashSet::new();

        recursive_backtrack(&mut maze, start, &mut visited, &mut rng);

        let mut to_visit = vec![start];
        let mut connected = HashSet::new();
        while let Some(current) = to_visit.pop() {
            if !connected.insert(current) {
                continue;
            }
            for dir in EdgeDirection::ALL_DIRECTIONS {
                let neighbor = current + dir;
                if let Some(walls) = maze.get_walls(&current) {
                    if !walls.contains(dir) && maze.get(&neighbor).is_some() {
                        to_visit.push(neighbor);
                    }
                }
            }
        }
        assert_eq!(
            connected.len(),
            maze.count(),
            "All tiles should be connected"
        );
    }
}
