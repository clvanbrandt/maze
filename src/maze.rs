use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;

use derive_more::{Add, Sub};
use graphics::types::Color;
use rand::seq::SliceRandom;

const WALL_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const END_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
const START_COLOR: Color = [0.0, 1.0, 0.0, 1.0];
const VISITED_COLOR: Color = [0.0, 0.0, 1.0, 1.0];
const CURRENT_COLOR: Color = [0.0, 1.0, 1.0, 1.0];

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug, Add, Sub)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Cell {
    position: Point,
    walls: HashMap<Direction, bool>,
    visited: bool,
    is_start: bool,
    is_end: bool,
    is_current: bool,
}

#[derive(Clone)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    cells: Vec<Vec<RefCell<Cell>>>,
    start: Point,
    end: Point,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GeneratorState {
    Clear,
    Initialised,
    InProgress,
    Done,
}

pub struct BacktrackingGenerator {
    stack: Vec<Point>,
    maze: Maze,
    current: Point,
    state: GeneratorState,
    width: usize,
    height: usize,
}

impl BacktrackingGenerator {
    pub fn new(width: usize, height: usize) -> Self {
        let stack = Vec::new();
        let maze = Maze::new(width, height);
        let current = maze.start;
        Self {
            stack,
            maze,
            current,
            state: GeneratorState::Clear,
            width,
            height,
        }
    }

    pub fn start(mut self, x: usize, y: usize) -> Self {
        self.set_start(x, y);
        self
    }

    pub fn set_start(&mut self, x: usize, y: usize) {
        self.maze.set_start(Point { x, y });
    }

    pub fn end(mut self, x: usize, y: usize) -> Self {
        self.set_end(x, y);
        self
    }

    pub fn set_end(&mut self, x: usize, y: usize) {
        self.maze.set_end(Point { x, y });
    }

    pub fn get_maze(&self) -> Maze {
        self.maze.clone()
    }

    pub fn get_maze_ref(&self) -> &Maze {
        &self.maze
    }

    fn set_current(&mut self, new_current: Point) {
        self.maze.get_cell(self.current).borrow_mut().is_current = false;
        self.current = new_current;
        self.maze.get_cell(new_current).borrow_mut().is_current = true;
    }

    pub fn restart(&mut self) {
        self.maze = Maze::new(self.width, self.height);
        self.initialize();
    }

    pub fn initialize(&mut self) {
        self.current = self.maze.start;
        self.stack.push(self.maze.start);
        self.maze.get_cell(self.maze.start).borrow_mut().visited = true;
        self.state = GeneratorState::Initialised;
    }

    pub fn next(&mut self) -> Result<GeneratorState, ()> {
        if GeneratorState::Clear == self.state {
            Err(())
        } else {
            if self.stack.is_empty() {
                self.state = GeneratorState::Done;
                self.maze.get_cell(self.current).borrow_mut().is_current = false;
            } else {
                self.state = GeneratorState::InProgress;
                let current = self.stack.pop().unwrap();
                self.set_current(current);
                if let Some(next) = self.maze.get_random_unvisited_neighbor(self.current) {
                    self.stack.push(self.current);
                    let mut current_cell = self.maze.get_cell(current).borrow_mut();
                    let mut next_cell = self.maze.get_cell(next).borrow_mut();

                    let direction = current_cell.get_relative_direction(&next_cell);
                    let other_direction = direction.opposite();

                    current_cell.walls.insert(direction, false);
                    next_cell.walls.insert(other_direction, false);
                    next_cell.visited = true;
                    self.stack.push(next);
                }
            }
            Ok(self.state)
        }
    }

    pub fn generate(&mut self) {
        self.initialize();
        while self.next() != Ok(GeneratorState::Done) {}
    }
}


impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let start = Point { x: 0, y: 0 };
        let end = Point { x: width - 1, y: height - 1 };
        let cells = vec![vec![RefCell::new(Cell::new(0, 0, false, false)); height]; width];

        for (x, row) in cells.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                cell.borrow_mut().position = Point { x, y };
            }
        }

        let maze = Self {
            width,
            height,
            cells,
            start,
            end,
        };

        maze.get_cell(start).borrow_mut().is_start = true;
        maze.get_cell(end).borrow_mut().is_end = true;

        maze
    }

    fn get_cell(&self, p: Point) -> &RefCell<Cell> {
        self.cells.get(p.x).unwrap().get(p.y).unwrap()
    }

    fn get_random_unvisited_neighbor(&self, coord: Point) -> Option<Point> {
        let cell = self.get_cell(coord);
        let neighbors: Vec<Point> = cell.borrow().get_neighbors(self.width, self.height)
            .iter()
            .map(|&x| self.get_cell(x).borrow())
            .filter(|c| !c.visited)
            .map(|c| c.position)
            .collect();

        match neighbors.choose(&mut rand::thread_rng()) {
            Some(&n) => Some(n),
            None => None
        }
    }

    fn set_start(&mut self, new_start: Point) {
        self.get_cell(self.start).borrow_mut().is_start = false;
        self.get_cell(new_start).borrow_mut().is_start = true;
        self.start = new_start;
    }

    fn set_end(&mut self, new_end: Point) {
        self.get_cell(self.end).borrow_mut().is_end = false;
        self.get_cell(new_end).borrow_mut().is_end = true;
        self.end = new_end;
    }

    pub fn draw(&self, c: &graphics::Context, gl: &mut opengl_graphics::GlGraphics, cell_size: f64) {
        for row in self.cells.iter() {
            for cell in row.iter().map(|rc| rc.borrow()) {
                cell.draw(c, gl, cell_size);
            }
        }
    }
}

impl Cell {
    pub fn new(x: usize, y: usize, is_start: bool, is_end: bool) -> Self {
        let mut walls = HashMap::new();

        walls.insert(Direction::North, true);
        walls.insert(Direction::South, true);
        walls.insert(Direction::East, true);
        walls.insert(Direction::West, true);

        Self {
            position: Point {
                x,
                y,
            },
            walls,
            visited: false,
            is_start,
            is_end,
            is_current: false,
        }
    }

    fn get_neighbors(&self, maze_width: usize, maze_height: usize) -> Vec<Point> {
        let mut potential_neighbors = Vec::new();
        for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)].iter() {
            let n_x = self.position.x as i32 + *dx;
            let n_y = self.position.y as i32 + *dy;
            if n_x >= 0 && n_x < maze_width as i32 && n_y >= 0 && n_y < maze_height as i32 {
                potential_neighbors.push(Point { x: n_x as usize, y: n_y as usize });
            }
        }
        potential_neighbors
    }

    fn get_relative_direction(&self, other: &Cell) -> Direction {
        match other.position.x.cmp(&self.position.x) {
            Ordering::Greater => Direction::East,
            Ordering::Less => Direction::West,
            Ordering::Equal => {
                match other.position.y.cmp(&self.position.y) {
                    Ordering::Greater => Direction::South,
                    Ordering::Less => Direction::North,
                    Ordering::Equal => panic!("Trying to remove a wall between a cell and itself")
                }
            }
        }
    }

    fn to_gui_coordinates(&self, cell_size: f64) -> (f64, f64) {
        (self.position.x as f64 * cell_size, self.position.y as f64 * cell_size)
    }

    pub fn draw(&self, c: &graphics::Context, gl: &mut opengl_graphics::GlGraphics, cell_size: f64) {
        use graphics::*;
        let (x, y) = self.to_gui_coordinates(cell_size);

        let wall_thickness = cell_size / 10.0 / 2.0;

        if self.is_start {
            rectangle(START_COLOR, [x, y, cell_size, cell_size], c.transform, gl);
        } else if self.is_end {
            rectangle(END_COLOR, [x, y, cell_size, cell_size], c.transform, gl);
        } else if self.is_current {
            rectangle(CURRENT_COLOR, [x, y, cell_size, cell_size], c.transform, gl);
        } else if self.visited {
            rectangle(VISITED_COLOR, [x, y, cell_size, cell_size], c.transform, gl);
        }

        for (dir, _) in self.walls.iter().filter(|(_, &present)| present) {
            match dir {
                Direction::North => {
                    rectangle(WALL_COLOR, [x, y, cell_size, wall_thickness], c.transform, gl);
                }
                Direction::South => {
                    rectangle(WALL_COLOR, [x, y + cell_size - wall_thickness, cell_size, wall_thickness], c.transform, gl);
                }
                Direction::East => {
                    rectangle(WALL_COLOR, [x + cell_size - wall_thickness, y, wall_thickness, cell_size], c.transform, gl);
                }
                Direction::West => {
                    rectangle(WALL_COLOR, [x, y, wall_thickness, cell_size], c.transform, gl);
                }
            }
        }
    }
}
