use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::maze::{Maze, Point};

use std::cmp::Ordering;

pub type Path = Vec<Point>;

#[derive(Eq, Debug)]
struct CostState {
    cost: usize,
    position: Point,
}

impl CostState {
    fn new(cost: usize, position: Point) -> Self {
        Self { cost, position }
    }
}

impl std::cmp::Ord for CostState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl std::cmp::PartialOrd for CostState {
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

impl std::cmp::PartialEq for CostState {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.position == other.position
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SolverState {
    Clear,
    Initialised,
    InProgress,
    Done,
}

pub struct AStarSolver {
    maze: Maze,
    open_set: BinaryHeap<CostState>,
    in_open_set: HashSet<Point>,
    came_from: HashMap<Point, Point>,
    g_score: HashMap<Point, usize>,
    state: SolverState,
}

impl AStarSolver {
    fn heuristic(&self, node: Point) -> usize {
        node.get_distance(&self.maze.get_end())
    }

    pub fn new(maze: Maze) -> Self {
        let open_set = BinaryHeap::new();
        let came_from = HashMap::new();
        let g_score = HashMap::with_capacity(maze.width * maze.height);
        let in_open_set = HashSet::new();
        Self {
            maze,
            open_set,
            came_from,
            g_score,
            in_open_set,
            state: SolverState::Clear,
        }
    }

    pub fn set_maze(&mut self, maze: Maze) {
        self.maze = maze;
    }

    fn initialize(&mut self) {
        let start = self.maze.get_start();
        self.open_set
            .push(CostState::new(self.heuristic(start), start));
        self.in_open_set.insert(start);

        for x in 0..self.maze.width {
            for y in 0..self.maze.height {
                self.g_score.insert(Point { x, y }, std::usize::MAX);
            }
        }
        self.g_score.insert(start, 0);
        self.state = SolverState::Initialised;
    }

    fn next_step(&mut self) -> Option<Path> {
        if self.state == SolverState::Clear {
            self.initialize()
        }

        self.state = SolverState::InProgress;

        if self.open_set.is_empty() {
            self.state = SolverState::Done;
            return None;
        }

        let goal = self.maze.get_end();
        let current = self.open_set.pop().unwrap();
        self.in_open_set.remove(&current.position);

        if current.position == goal {
            return Some(self.reconstruct_path(&current.position));
        }

        let maze = &self.maze;
        for &neighbor in current
            .position
            .get_neighbors(self.maze.width, self.maze.height)
            .iter()
            .filter(|&p| !maze.is_wall_present(&current.position, p))
        {
            let tentative_gscore = self.g_score.get(&current.position).unwrap() + 1;
            // 1 because distance between node and neighbor is 1
            if tentative_gscore < *self.g_score.get(&neighbor).unwrap() {
                self.came_from.insert(neighbor, current.position);
                self.g_score.insert(neighbor, tentative_gscore);

                if !self.in_open_set.contains(&neighbor) {
                    self.open_set.push(CostState::new(
                        tentative_gscore + self.heuristic(neighbor),
                        neighbor,
                    ));
                    self.in_open_set.insert(neighbor);
                }
            }
        }
        None
    }

    pub fn solve(&mut self) -> Option<Path> {
        while self.state != SolverState::Done {
            if let Some(path) = self.next_step() {
                return Some(path);
            }
        }
        None
    }

    pub fn get_current_cost_map(&self) -> &HashMap<Point, usize> {
        &self.g_score
    }

    fn reconstruct_path(&self, node: &Point) -> Path {
        let mut path = vec![*node];
        let mut current = *node;
        while let Some(&prev) = self.came_from.get(&current) {
            current = prev;
            path.push(current);
        }
        path.reverse();
        path
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::Point;
    use crate::solving::CostState;
    use std::collections::BinaryHeap;

    #[test]
    fn min_heap() {
        let mut heap: BinaryHeap<CostState> = BinaryHeap::new();

        let point = Point { x: 0, y: 0 };

        heap.push(CostState::new(5, point));
        heap.push(CostState::new(1, point));
        heap.push(CostState::new(2, point));
        heap.push(CostState::new(25, point));
        heap.push(CostState::new(10, point));

        assert_eq!(heap.pop(), Some(CostState::new(1, point)));
        assert_eq!(heap.pop(), Some(CostState::new(2, point)));
        assert_eq!(heap.pop(), Some(CostState::new(5, point)));
        assert_eq!(heap.pop(), Some(CostState::new(10, point)));
        assert_eq!(heap.pop(), Some(CostState::new(25, point)));
    }
}
