use hexlab::prelude::*;

#[test]
fn hex_maze_creation_and_basic_operations() {
    let mut maze = Maze::new();
    assert!(maze.is_empty());

    let center = Hex::ZERO;
    maze.insert(center);
    assert_eq!(maze.count(), 1);
    assert!(!maze.is_empty());

    let tile = maze.get(&center);
    assert!(tile.is_some());
    assert_eq!(tile.unwrap().pos(), center);
}

#[test]
fn hex_maze_wall_operations() {
    let mut maze = Maze::new();
    let center = Hex::ZERO;
    maze.insert(center);

    // Add walls
    for direction in EdgeDirection::ALL_DIRECTIONS {
        let _ = maze.add_tile_wall(&center, direction);
    }

    let walls = maze.get_walls(&center).unwrap();
    assert_eq!(walls.count(), 6);

    // Remove walls
    for direction in EdgeDirection::ALL_DIRECTIONS {
        let _ = maze.remove_tile_wall(&center, direction);
    }

    let walls = maze.get_walls(&center).unwrap();
    assert_eq!(walls.count(), 0);
}

#[test]
fn hex_maze_multiple_tiles() {
    let mut maze = Maze::new();
    let tiles = [Hex::ZERO, Hex::new(1, -1), Hex::new(0, 1), Hex::new(-1, 1)];

    for &tile in &tiles {
        maze.insert(tile);
    }

    assert_eq!(maze.count(), tiles.len());

    for &tile in &tiles {
        assert!(maze.get(&tile).is_some());
    }
}

#[test]
fn hex_maze_edge_cases() {
    let mut maze = Maze::new();
    let non_existent = Hex::new(10, 10);

    // Operations on non-existent tiles should not panic
    let _ = maze.add_tile_wall(&non_existent, EdgeDirection::FLAT_NORTH);
    let _ = maze.remove_tile_wall(&non_existent, EdgeDirection::FLAT_NORTH);

    assert!(maze.get(&non_existent).is_none());
    assert!(maze.get_walls(&non_existent).is_none());
}
