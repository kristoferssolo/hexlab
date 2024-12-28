use hexlab::prelude::*;
use rstest::rstest;

#[rstest]
#[case(GeneratorType::RecursiveBacktracking, None, None)]
#[case(GeneratorType::RecursiveBacktracking, Some(Hex::new(1, -1)), None)]
#[case(GeneratorType::RecursiveBacktracking, None, Some(12345))]
fn generator_type(
    #[case] generator: GeneratorType,
    #[case] start_pos: Option<Hex>,
    #[case] seed: Option<u64>,
) {
    let mut maze = Maze::new();
    for q in -3..=3 {
        for r in -3..=3 {
            let hex = Hex::new(q, r);
            if hex.length() <= 3 {
                maze.insert(hex);
            }
        }
    }
    let initial_size = maze.len();

    generator.generate(&mut maze, start_pos, seed);

    assert_eq!(maze.len(), initial_size, "Maze size should not change");

    // Check maze connectivity
    let start = start_pos.unwrap_or(Hex::ZERO);
    let mut to_visit = vec![start];
    let mut visited = std::collections::HashSet::new();
    while let Some(current) = to_visit.pop() {
        if !visited.insert(current) {
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
    assert_eq!(visited.len(), maze.len(), "All tiles should be connected");

    // Check that each tile has at least one open wall
    for &pos in maze.keys() {
        let walls = maze.get_walls(&pos).unwrap();
        assert!(
            walls.count() < 6,
            "Tile at {:?} should have at least one open wall",
            pos
        );
    }
}

#[test]
fn test_empty_maze() {
    let mut maze = Maze::new();
    GeneratorType::RecursiveBacktracking.generate(&mut maze, None, None);
    assert!(
        maze.is_empty(),
        "Empty maze should remain empty after generation"
    );
}
