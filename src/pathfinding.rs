use hexx::{EdgeDirection, Hex};
use pathfinding::prelude::*;

use crate::Maze;

impl Maze {
    pub fn find_path(&self, from: Hex, to: Hex) -> Option<Vec<Hex>> {
        let successors = |pos: &Hex| {
            {
                EdgeDirection::ALL_DIRECTIONS.iter().filter_map(|&dir| {
                    let neighbor = pos.neighbor(dir);
                    if let Some(current_tile) = self.get(pos) {
                        if let Some(_) = self.get(&neighbor) {
                            if !current_tile.walls.contains(dir) {
                                return Some((neighbor, 1)); // Cost of 1 for each step
                            }
                        }
                    }
                    None
                })
            }
            .collect::<Vec<_>>()
        };

        let heuristic = |pos: &Hex| {
            let diff = *pos - to;
            (diff.x.abs() + diff.y.abs()) as u32
        };

        astar(&from, successors, heuristic, |pos| *pos == to).map(|(path, _)| path)
    }
}
