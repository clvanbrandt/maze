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

#[derive(Clone, Eq, Copy, PartialEq)]
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
    cells_state: HashMap<Point, BacktrackingCellState>,
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
        let mut cells_state = HashMap::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                cells_state.insert(Point { x, y }, BacktrackingCellState::Unvisited);
            }
        }
        cells_state.insert(current, BacktrackingCellState::Current);

        Self {
            stack,
            maze,
            current,
            state,
            width,
            height,
            cells_state,
        }
    }

    pub fn get_maze_ref(&self) -> &Maze {
        &self.maze
    }

    pub fn restart(&mut self) {
        self.maze = Maze::new(self.width, self.height);
        self.stack.clear();
        self.state = GeneratorState::Clear;
    }

    fn initialize(&mut self) {
        let start = Point { x: 0, y: 0 };
        self.current = start;
        self.stack.push(start);
        self.state = GeneratorState::Initialised;
        for y in 0..self.height {
            for x in 0..self.width {
                self.cells_state
                    .insert(Point { x, y }, BacktrackingCellState::Unvisited);
            }
        }
        self.cells_state
            .insert(start, BacktrackingCellState::Current);
    }

    pub fn next_step(&mut self) {
        if GeneratorState::Clear == self.state {
            self.initialize();
        }

        if self.stack.is_empty() {
            self.state = GeneratorState::Done;
        } else {
            self.state = GeneratorState::InProgress;

            self.cells_state
                .insert(self.current, BacktrackingCellState::Visited);
            self.current = self.stack.pop().unwrap();
            self.cells_state
                .insert(self.current, BacktrackingCellState::Current);

            if let Some(next) = self.get_random_unvisited_neighbor(self.current) {
                self.stack.push(self.current);

                let direction = self.current.get_relative_direction(&next);
                let other_direction = direction.opposite();

                self.maze
                    .get_cell_mut(&self.current)
                    .remove_wall(&direction);
                self.maze.get_cell_mut(&next).remove_wall(&other_direction);

                self.cells_state
                    .insert(next, BacktrackingCellState::Visited);
                self.stack.push(next);
            }
        }
    }

    pub fn get_cells_state(&self) -> HashMap<Point, BacktrackingCellState> {
        self.cells_state.clone()
    }

    fn get_random_unvisited_neighbor(&self, coord: Point) -> Option<Point> {
        let neighbors: Vec<Point> = coord
            .get_neighbors(self.width, self.height)
            .iter()
            .filter_map(|&x| {
                if *self.cells_state.get(&x).unwrap() == BacktrackingCellState::Unvisited {
                    Some(x)
                } else {
                    None
                }
            })
            .collect();

        match neighbors.choose(&mut rand::thread_rng()) {
            Some(&n) => Some(n),
            None => None,
        }
    }

    pub fn generate(&mut self) -> Maze {
        while self.state != GeneratorState::Done {
            self.next_step();
        }
        self.maze.clone()
    }

    pub fn is_done(&self) -> bool {
        self.state == GeneratorState::Done
    }
}
