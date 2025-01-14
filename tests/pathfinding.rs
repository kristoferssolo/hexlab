use claims::*;
use hexlab::MazeBuilder;
use hexx::{hex, Hex};

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
