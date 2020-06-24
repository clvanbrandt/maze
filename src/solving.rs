use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::maze::{Maze, Point};
use piston::input::keyboard::Key::P;
use std::borrow::Borrow;
use std::cmp::Ordering;

pub type Path = Vec<Point>;

#[derive(Eq, Debug)]
struct State {
    cost: usize,
    position: Point,
}

impl State {
    fn new(cost: usize, position: Point) -> Self {
        Self { cost, position }
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        self.cost > other.cost
    }

    fn le(&self, other: &Self) -> bool {
        self.cost >= other.cost
    }

    fn gt(&self, other: &Self) -> bool {
        self.cost < other.cost
    }

    fn ge(&self, other: &Self) -> bool {
        self.cost <= other.cost
    }
}

impl std::cmp::PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.position == other.position
    }
}

pub struct AStarSolver {
    maze: Maze,
    open_set: HashSet<Point>,
    came_from: HashMap<Point, Point>,
    g_score: HashMap<Point, usize>,
    f_score: HashMap<Point, usize>,
}

impl AStarSolver {
    fn heuristic(&self, node: Point) -> usize {
        node.get_distance(&self.maze.get_end())
    }

    pub fn new(maze: Maze) -> Self {
        let open_set = HashSet::new();
        let came_from = HashMap::new();
        let g_score = HashMap::with_capacity(maze.width * maze.height);
        let f_score = HashMap::with_capacity(maze.width * maze.height);
        Self {
            maze,
            open_set,
            came_from,
            g_score,
            f_score,
        }
    }

    pub fn set_maze(&mut self, maze: Maze) {
        self.maze = maze;
    }

    fn initialisation(&mut self) {
        let start = self.maze.get_start();
        self.open_set.insert(start);

        for x in 0..self.maze.width {
            for y in 0..self.maze.height {
                self.g_score.insert(Point { x, y }, std::usize::MAX);
                self.f_score.insert(Point { x, y }, std::usize::MAX);
            }
        }
        self.g_score.insert(start, 0);
        self.f_score.insert(start, self.heuristic(start));
    }

    fn next_step(&mut self) {}

    pub fn solve(&mut self) -> Option<Path> {
        self.initialisation();

        let goal = self.maze.get_end();

        while !self.open_set.is_empty() {
            let mut score = std::usize::MAX;
            let mut current = Point { x: 0, y: 0 };
            for node in self.open_set.iter() {
                let pot_score = *self.f_score.get(&node).unwrap();
                if pot_score <= score {
                    current = *node;
                    score = pot_score;
                }
            }
            if current == goal {
                return Some(self.reconstruct_path(&current));
            }
            self.open_set.remove(&current);
            let maze = &self.maze;
            for &neighbor in current
                .get_neighbors(self.maze.width, self.maze.height)
                .iter()
                .filter(|&p| !maze.is_wall_present(&current, p))
            {
                let tentative_gscore = self.g_score.get(&current).unwrap() + 1;
                // 1 because distance between node and neighbor is 1
                if tentative_gscore < *self.g_score.get(&neighbor).unwrap() {
                    self.came_from.insert(neighbor, current);
                    self.g_score.insert(neighbor, tentative_gscore);
                    self.f_score
                        .insert(neighbor, tentative_gscore + self.heuristic(neighbor));
                    match self.open_set.get(&neighbor) {
                        Some(_) => {}
                        None => {
                            self.open_set.insert(neighbor);
                        }
                    }
                }
            }
        }
        None
    }

    fn reconstruct_path(&self, node: &Point) -> Path {
        let mut path = vec![*node];
        let mut current = *node;
        while let Some(&prev) = self.came_from.get(&current) {
            current = prev;
            path.push(current);
        }
        path
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::Point;
    use crate::solving::State;
    use std::collections::BinaryHeap;

    #[test]
    fn min_heap() {
        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        let point = Point { x: 0, y: 0 };

        heap.push(State::new(5, point));
        heap.push(State::new(1, point));
        heap.push(State::new(2, point));
        heap.push(State::new(25, point));
        heap.push(State::new(10, point));

        assert_eq!(heap.pop(), Some(State::new(1, point)));
        assert_eq!(heap.pop(), Some(State::new(2, point)));
        assert_eq!(heap.pop(), Some(State::new(5, point)));
        assert_eq!(heap.pop(), Some(State::new(10, point)));
        assert_eq!(heap.pop(), Some(State::new(25, point)));
    }
}
