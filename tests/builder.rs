use claims::{assert_err, assert_gt, assert_matches, assert_ok, assert_some};
use hexlab::prelude::*;
use rstest::rstest;

#[rstest]
#[case(1, 7)]
#[case(2, 19)]
#[case(3, 37)]
#[case(4, 61)]
#[case(5, 91)]
fn maze_size(#[case] radius: u16, #[case] expected_size: usize) {
    let maze = assert_ok!(MazeBuilder::new().with_radius(radius).build());
    assert_eq!(maze.len(), expected_size);
}

#[test]
fn builder_without_radius() {
    let result = MazeBuilder::new().build();
    assert_err!(&result);
    assert_matches!(result, Err(MazeBuilderError::NoRadius));
}

#[rstest]
#[case(Hex::ZERO)]
#[case(Hex::new(1,-1))]
#[case(Hex::new(-2,1))]
fn valid_start_position(#[case] start_pos: Hex) {
    let maze = assert_ok!(MazeBuilder::new()
        .with_radius(3)
        .with_start_position(start_pos)
        .build());
    assert_some!(maze.get_tile(&start_pos));
}

#[test]
fn invalid_start_position() {
    let maze = MazeBuilder::new()
        .with_radius(3)
        .with_start_position(Hex::new(10, 10))
        .build();

    assert_err!(&maze);
    assert_matches!(maze, Err(MazeBuilderError::InvalidStartPosition(_)));
}

#[test]
fn maze_with_seed() {
    let maze1 = assert_ok!(MazeBuilder::new().with_radius(3).with_seed(12345).build());
    let maze2 = assert_ok!(MazeBuilder::new().with_radius(3).with_seed(12345).build());

    assert_eq!(maze1, maze2, "Mazes with the same seed should be identical");
}

#[test]
fn different_seeds_produce_different_mazes() {
    let maze1 = assert_ok!(MazeBuilder::new().with_radius(3).with_seed(12345).build());
    let maze2 = assert_ok!(MazeBuilder::new().with_radius(3).with_seed(54321).build());

    assert_ne!(
        maze1, maze2,
        "Mazes with different seeds should be different"
    );
}

#[test]
fn maze_connectivity() {
    let maze = assert_ok!(MazeBuilder::new().with_radius(3).build());

    // Helper function to count accessible neighbors
    fn count_accessible_neighbors(maze: &HexMaze, pos: Hex) -> usize {
        hexx::EdgeDirection::ALL_DIRECTIONS
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
        claims::assert_gt!(
            accessible_neighbors,
            0,
            "Tile at {:?} has no accessible neighbors",
            pos
        );
    }
}

#[test]
fn generator_type() {
    let maze = assert_ok!(MazeBuilder::new()
        .with_radius(3)
        .with_generator(GeneratorType::RecursiveBacktracking)
        .build());
    claims::assert_gt!(maze.len(), 0);
}

#[test]
fn maze_boundaries() {
    let radius = 3;
    let maze = MazeBuilder::new()
        .with_radius(radius as u16)
        .build()
        .unwrap();

    // Test that tiles exist within the radius
    for q in -radius..=radius {
        for r in -radius..=radius {
            let pos = Hex::new(q, r);
            if q.abs() + r.abs() <= radius {
                assert!(
                    maze.get_tile(&pos).is_some(),
                    "Expected tile at {:?} to exist",
                    pos
                );
            }
        }
    }
}

#[rstest]
#[case(GeneratorType::RecursiveBacktracking)]
fn generate_maze_with_different_types(#[case] generator: GeneratorType) {
    // TODO: Add more generator types when they become available

    let maze = assert_ok!(MazeBuilder::new()
        .with_radius(3)
        .with_generator(generator)
        .build());

    assert_gt!(maze.len(), 0);
}
