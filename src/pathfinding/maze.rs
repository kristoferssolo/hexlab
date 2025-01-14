use crate::Maze;
#[cfg(feature = "bevy_reflect")]
use bevy_utils::{HashMap, HashSet};
use hexx::{EdgeDirection, Hex};
#[cfg(not(feature = "bevy_reflect"))]
use std::collections::{HashMap, HashSet};
use std::{collections::BinaryHeap, i32};

use super::node::Node;

impl Maze {
    pub fn find_path(&self, from: Hex, to: Hex) -> Option<Vec<Hex>> {
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        let mut closed_set = HashSet::new();

        initialize_search(&mut open_set, &mut g_score, from, to);
        while let Some(current) = open_set.pop() {
            if current.position == to {
                return Some(self.reconstruct_path(came_from, current.position));
            }
            if !self.process_current_node(
                current,
                to,
                &mut open_set,
                &mut came_from,
                &mut g_score,
                &mut closed_set,
            ) {
                continue;
            }
        }
        None
    }

    fn is_valid_move(&self, from: &Hex, to: &Hex, direction: EdgeDirection) -> bool {
        if let Some(current_tile) = self.get(from) {
            if let Some(_) = self.get(to) {
                return !current_tile.walls.contains(direction);
            }
        }
        false
    }

    fn process_current_node(
        &self,
        current: Node,
        goal: Hex,
        open_set: &mut BinaryHeap<Node>,
        came_from: &mut HashMap<Hex, Hex>,
        g_score: &mut HashMap<Hex, i32>,
        closed_set: &mut HashSet<Hex>,
    ) -> bool {
        if closed_set.contains(&current.position) {
            return false;
        }
        closed_set.insert(current.position);

        self.process_neighbors(current, goal, open_set, came_from, g_score, closed_set);

        true
    }

    fn process_neighbors(
        &self,
        current: Node,
        goal: Hex,
        open_set: &mut BinaryHeap<Node>,
        came_from: &mut HashMap<Hex, Hex>,
        g_score: &mut HashMap<Hex, i32>,
        closed_set: &HashSet<Hex>,
    ) {
        for direction in EdgeDirection::ALL_DIRECTIONS {
            let neighbor_pos = current.position.neighbor(direction);
            if closed_set.contains(&neighbor_pos) {
                continue;
            }
            if self.is_valid_move(&current.position, &neighbor_pos, direction) {
                self.update_neighbor(current, neighbor_pos, goal, open_set, came_from, g_score);
            }
        }
    }

    fn update_neighbor(
        &self,
        current: Node,
        neighbor_pos: Hex,
        goal: Hex,
        open_set: &mut BinaryHeap<Node>,
        came_from: &mut HashMap<Hex, Hex>,
        g_score: &mut HashMap<Hex, i32>,
    ) {
        let tentative_g_score = g_score.get(&current.position).unwrap() + 1;

        if tentative_g_score < *g_score.get(&neighbor_pos).unwrap_or(&i32::MAX) {
            came_from.insert(neighbor_pos, current.position);
            g_score.insert(neighbor_pos, tentative_g_score);
            open_set.push(Node {
                position: neighbor_pos,
                f_score: tentative_g_score + heuristic(neighbor_pos, goal),
                g_score: tentative_g_score,
            });
        }
    }

    fn reconstruct_path(&self, came_from: HashMap<Hex, Hex>, current: Hex) -> Vec<Hex> {
        let mut path = vec![current];
        let mut current_pos = current;
        while let Some(&prev) = came_from.get(&current_pos) {
            path.push(prev);
            current_pos = prev;
        }
        path.reverse();
        path
    }
}

fn initialize_search(
    open_set: &mut BinaryHeap<Node>,
    g_score: &mut HashMap<Hex, i32>,
    start: Hex,
    goal: Hex,
) {
    g_score.insert(start, 0);
    open_set.push(Node {
        position: start,
        f_score: heuristic(start, goal),
        g_score: 0,
    })
}

fn heuristic(from: Hex, to: Hex) -> i32 {
    // Manhatan distance for hex grid
    let pos = from - to;
    pos.x.abs() + pos.y.abs()
}
