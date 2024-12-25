use hexlab::prelude::*;

#[test]
fn hex_maze_creation_and_basic_operations() {
    let mut maze = HexMaze::new();
    assert!(maze.is_empty());

    let center = Hex::ZERO;
    maze.add_tile(center);
    assert_eq!(maze.len(), 1);
    assert!(!maze.is_empty());

    let tile = maze.get_tile(&center);
    assert!(tile.is_some());
    assert_eq!(tile.unwrap().pos(), center);
}

#[test]
fn hex_maze_wall_operations() {
    let mut maze = HexMaze::new();
    let center = Hex::ZERO;
    maze.add_tile(center);

    // Add walls
    for direction in EdgeDirection::ALL_DIRECTIONS {
        maze.add_wall(center, direction);
    }

    let walls = maze.get_walls(&center).unwrap();
    assert_eq!(walls.count(), 6);

    // Remove walls
    for direction in EdgeDirection::ALL_DIRECTIONS {
        maze.remove_tile_wall(&center, direction);
    }

    let walls = maze.get_walls(&center).unwrap();
    assert_eq!(walls.count(), 0);
}

#[test]
fn hex_maze_multiple_tiles() {
    let mut maze = HexMaze::new();
    let tiles = [Hex::ZERO, Hex::new(1, -1), Hex::new(0, 1), Hex::new(-1, 1)];

    for &tile in &tiles {
        maze.add_tile(tile);
    }

    assert_eq!(maze.len(), tiles.len());

    for &tile in &tiles {
        assert!(maze.get_tile(&tile).is_some());
    }
}

#[test]
fn hex_maze_edge_cases() {
    let mut maze = HexMaze::new();
    let non_existent = Hex::new(10, 10);

    // Operations on non-existent tiles should not panic
    maze.add_wall(non_existent, EdgeDirection::FLAT_NORTH);
    maze.remove_tile_wall(&non_existent, EdgeDirection::FLAT_NORTH);

    assert!(maze.get_tile(&non_existent).is_none());
    assert!(maze.get_walls(&non_existent).is_none());
}
