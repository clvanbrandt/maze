use std::collections::{BinaryHeap, HashMap};

use crate::maze::{Maze, Point};

struct Path {}

struct State {
    cost: usize,
    position: Point,
}

struct AStarSolver {
    open_set: BinaryHeap<State>,
    came_from: HashMap<Point, Point>,
    g_score: HashMap<Point, usize>,
    f_score: HashMap<Point, usize>,
}

impl AStarSolver {
    pub fn new() -> Self {
        let open_set = BinaryHeap::new();
        let came_from = HashMap::new();
        let g_score = HashMap::new();
        let f_score = HashMap::new();
        Self {
            open_set,
            came_from,
            g_score,
            f_score,
        }
    }
}