use claims::*;
use hexlab::MazeBuilder;
use hexx::{hex, EdgeDirection, Hex};

#[test]
fn basic_path() {
    let maze = assert_ok!(MazeBuilder::new().with_seed(12345).with_radius(5).build());

    let start = Hex::new(0, 0);
    let goal = Hex::new(2, 0);

    assert_some_eq!(
        maze.find_path(start, goal),
        vec![start, hex(1, 0), hex(1, 1), hex(2, 1), goal]
    );
}

#[test]
fn path_with_walls() {
    let mut maze = assert_ok!(MazeBuilder::new().with_seed(12345).with_radius(5).build());
    let start = Hex::new(0, 0);
    let goal = Hex::new(2, 0);

    // Block direct path with wall
    assert_ok!(maze.add_tile_wall(&start, EdgeDirection::FLAT_SOUTH));

    // Should find alternative path or no path
    let path = maze.find_path(start, goal);
    if let Some(path) = path {
        // If path exists, verify it's valid
        assert!(path.len() > 3); // Should be longer than direct path
        assert_eq!(path.first(), Some(&start));
        assert_eq!(path.last(), Some(&goal));
    }
}

#[test]
fn path_to_self() {
    let maze = assert_ok!(MazeBuilder::new().with_seed(12345).with_radius(5).build());
    let pos = Hex::new(0, 0);

    assert_some_eq!(maze.find_path(pos, pos), vec![pos]);
}

#[test]
fn no_path_exists() {
    let mut maze = assert_ok!(MazeBuilder::new().with_seed(12345).with_radius(5).build());
    let start = Hex::new(0, 0);
    let goal = Hex::new(2, 0);

    // Surround start with walls
    for dir in EdgeDirection::ALL_DIRECTIONS {
        assert_ok!(maze.add_tile_wall(&start, dir));
    }

    assert_none!(maze.find_path(start, goal));
}

#[test]
fn path_in_larger_maze() {
    let maze = assert_ok!(MazeBuilder::new().with_seed(12345).with_radius(10).build());
    let start = Hex::new(-5, -5);
    let goal = Hex::new(5, 5);

    let path = assert_some!(maze.find_path(start, goal));

    // Basic path properties
    assert_eq!(path.first(), Some(&start));
    assert_eq!(path.last(), Some(&goal));

    // Path should be continuous
    for window in path.windows(2) {
        let current = window[0];
        let next = window[1];
        assert!(EdgeDirection::ALL_DIRECTIONS
            .iter()
            .any(|&dir| current.neighbor(dir) == next));
    }
}
