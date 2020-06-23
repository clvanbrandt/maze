use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use rand::seq::SliceRandom;

use crate::maze::{Maze, Point};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GeneratorState {
    Clear,
    Initialised,
    InProgress,
    Done,
}

pub enum BacktrackingCellState {
    Unvisited,
    Visited,
    Current,
}

pub struct BacktrackingGenerator {
    stack: Vec<Point>,
    maze: Maze,
    current: Point,
    state: GeneratorState,
    visited: HashSet<Point>,
    pub width: usize,
    pub height: usize,
}

#[allow(dead_code)]
impl BacktrackingGenerator {
    pub fn new(width: usize, height: usize) -> Self {
        let stack = Vec::new();
        let maze = Maze::new(width, height);
        let current = Point { x: 0, y: 0 };
        let state = GeneratorState::Clear;
        let visited = HashSet::new();
        Self {
            stack,
            maze,
            current,
            state,
            width,
            height,
            visited,
        }
    }

    pub fn get_maze_ref(&self) -> &Maze {
        &self.maze
    }

    pub fn restart(&mut self) {
        self.maze = Maze::new(self.width, self.height);
        self.stack.clear();
        self.visited.clear();
        self.state = GeneratorState::Clear;
    }

    fn initialize(&mut self) {
        let start = Point { x: 0, y: 0 };
        self.current = start;
        self.stack.push(start);
        self.visited.insert(start);
        self.state = GeneratorState::Initialised;
    }

    pub fn next_iter(&mut self) {
        if GeneratorState::Clear == self.state {
            self.initialize();
        }

        if self.stack.is_empty() {
            self.state = GeneratorState::Done;
        } else {
            self.state = GeneratorState::InProgress;

            self.current = self.stack.pop().unwrap();

            if let Some(next) = self.get_random_unvisited_neighbor(self.current) {
                self.stack.push(self.current);

                let direction = self.current.get_relative_direction(&next);
                let other_direction = direction.opposite();

                self.maze.get_cell_mut(self.current).remove_wall(&direction);
                self.maze.get_cell_mut(next).remove_wall(&other_direction);

                self.visited.insert(next);
                self.stack.push(next);
            }
        }
    }

    pub fn get_cells_state(&self) -> HashMap<Point, BacktrackingCellState> {
        let mut state_map = HashMap::new();

        for row in self.maze.get_cells().iter() {
            for cell in row.iter() {
                let mut state = BacktrackingCellState::Unvisited;

                if cell.position == self.current {
                    state = BacktrackingCellState::Current;
                } else if self.visited.get(&cell.position).is_some() {
                    state = BacktrackingCellState::Visited;
                }
                state_map.insert(cell.position, state);
            }
        }

        state_map
    }

    fn get_random_unvisited_neighbor(&self, coord: Point) -> Option<Point> {
        let neighbors: Vec<Point> = coord.get_neighbors(self.width, self.height)
            .iter()
            .filter_map(|&x| {
                match self.visited.get(&x) {
                    Some(_) => None,
                    None => Some(x)
                }
            })
            .collect();

        match neighbors.choose(&mut rand::thread_rng()) {
            Some(&n) => Some(n),
            None => None
        }
    }

    pub fn generate(&mut self) {
        while self.state != GeneratorState::Done {
            self.next_iter();
        }
    }

    pub fn is_done(&self) -> bool {
        self.state == GeneratorState::Done
    }
}

