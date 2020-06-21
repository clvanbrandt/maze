use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use derive_more::{Add, Sub};
use graphics::types::Color;
use rand::seq::SliceRandom;

const WALL_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

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
pub struct Point {
    x: usize,
    y: usize,
}

impl std::convert::From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point { x, y }
    }
}

impl std::convert::Into<(usize, usize)> for Point {
    fn into(self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl Point {
    fn get_neighbors(&self, x_limit: usize, y_limit: usize) -> Vec<Point> {
        let mut neighbors = Vec::new();
        for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)].iter() {
            let n_x = self.x as i32 + *dx;
            let n_y = self.y as i32 + *dy;
            if n_x >= 0 && n_x < x_limit as i32 && n_y >= 0 && n_y < y_limit as i32 {
                neighbors.push(Point { x: n_x as usize, y: n_y as usize });
            }
        }
        neighbors
    }
}

#[derive(Clone)]
struct Cell {
    position: Point,
    walls: HashSet<Direction>,
}

pub struct Maze {
    pub width: usize,
    pub height: usize,
    cells: Vec<Vec<Cell>>,
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
    stack: RefCell<Vec<Point>>,
    maze: Rc<RefCell<Maze>>,
    current: std::cell::Cell<Point>,
    state: std::cell::Cell<GeneratorState>,
    visited: RefCell<HashSet<Point>>,
    pub width: usize,
    pub height: usize,
}

pub enum BacktrackingCellState {
    Unvisited,
    Visited,
    Current,
}

#[allow(dead_code)]
impl BacktrackingGenerator {
    pub fn new(width: usize, height: usize) -> Self {
        let stack = RefCell::new(Vec::new());
        let maze = Rc::new(RefCell::new(Maze::new(width, height)));
        let current = std::cell::Cell::new(Point { x: 0, y: 0 });
        let state = std::cell::Cell::new(GeneratorState::Clear);
        let visited = RefCell::new(HashSet::new());
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

    pub fn get_maze(&self) -> Rc<RefCell<Maze>> {
        self.maze.clone()
    }

    pub fn restart(&self) {
        self.maze.replace(Maze::new(self.width, self.height));
        self.stack.borrow_mut().clear();
        self.visited.borrow_mut().clear();
        self.state.set(GeneratorState::Clear);
    }

    fn initialize(&self) {
        let start = Point { x: 0, y: 0 };
        self.current.replace(start);
        self.stack.borrow_mut().push(start);
        self.visited.borrow_mut().insert(start);
        self.state.set(GeneratorState::Initialised);
    }

    pub fn next_iter(&self) {
        if GeneratorState::Clear == self.state.get() {
            self.initialize();
        }

        let mut stack = self.stack.borrow_mut();

        if stack.is_empty() {
            self.state.set(GeneratorState::Done);
        } else {
            let mut maze = self.maze.borrow_mut();
            self.state.set(GeneratorState::InProgress);

            self.current.replace(stack.pop().unwrap());

            if let Some(next) = self.get_random_unvisited_neighbor(self.current.get()) {
                stack.push(self.current.get());
                let current_cell = maze.get_cell(self.current.get());
                let next_cell = maze.get_cell(next);

                let direction = current_cell.get_relative_direction(&next_cell);
                let other_direction = direction.opposite();

                let current_cell = maze.get_cell_mut(self.current.get());
                current_cell.walls.remove(&direction);

                let next_cell = maze.get_cell_mut(next);
                next_cell.walls.remove(&other_direction);

                self.visited.borrow_mut().insert(next_cell.position);
                stack.push(next);
            }
        }
    }

    pub fn get_cells_state(&self) -> HashMap<Point, BacktrackingCellState> {
        let mut state_map = HashMap::new();

        for row in self.maze.borrow().cells.iter() {
            for cell in row.iter() {
                let mut state = BacktrackingCellState::Unvisited;

                if cell.position == self.current.get() {
                    state = BacktrackingCellState::Current;
                } else if self.visited.borrow().get(&cell.position).is_some() {
                    state = BacktrackingCellState::Visited;
                }
                state_map.insert(cell.position, state);
            }
        }

        state_map
    }

    fn get_random_unvisited_neighbor(&self, coord: Point) -> Option<Point> {
        let visited = self.visited.borrow();

        let neighbors: Vec<Point> = coord.get_neighbors(self.width, self.height)
            .iter()
            .filter_map(|&x| {
                match visited.get(&x) {
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
        while self.state.get() != GeneratorState::Done {
            self.next_iter();
        }
    }

    pub fn is_done(&self) -> bool {
        self.state.get() == GeneratorState::Done
    }
}

#[allow(dead_code)]
impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let start = Point { x: 0, y: 0 };
        let end = Point { x: width - 1, y: height - 1 };
        let mut cells = vec![vec![Cell::new(0, 0); height]; width];

        for (x, row) in cells.iter_mut().enumerate() {
            for (y, cell) in row.iter_mut().enumerate() {
                cell.position = Point { x, y };
            }
        }

        Self {
            width,
            height,
            cells,
            start,
            end,
        }
    }

    fn get_cell_mut(&mut self, p: Point) -> &mut Cell {
        self.cells.get_mut(p.x).unwrap().get_mut(p.y).unwrap()
    }

    fn get_cell(&self, p: Point) -> &Cell {
        self.cells.get(p.x).unwrap().get(p.y).unwrap()
    }

    pub fn set_start(&mut self, x: usize, y: usize) {
        self.start = Point { x, y };
    }

    pub fn set_end(&mut self, x: usize, y: usize) {
        self.end = Point { x, y };
    }

    pub fn get_start(&self) -> Point {
        self.start
    }

    pub fn get_end(&self) -> Point {
        self.end
    }
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Self {
        let mut walls = HashSet::new();

        walls.insert(Direction::North);
        walls.insert(Direction::South);
        walls.insert(Direction::East);
        walls.insert(Direction::West);

        Self {
            position: Point {
                x,
                y,
            },
            walls,
        }
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
}

pub struct Drawer {
    maze: Rc<RefCell<Maze>>,
    x_offset: f64,
    y_offset: f64,
    cell_size: f64,
}

impl Drawer {
    pub fn new(maze: Rc<RefCell<Maze>>) -> Self {
        Self {
            maze,
            x_offset: 0.0,
            y_offset: 0.0,
            cell_size: 0.0,
        }
    }

    pub fn x_offset(mut self, x_offset: f64) -> Self {
        self.x_offset = x_offset;
        self
    }

    pub fn y_offset(mut self, y_offset: f64) -> Self {
        self.y_offset = y_offset;
        self
    }

    pub fn cell_size(mut self, cell_size: f64) -> Self {
        self.set_cell_size(cell_size);
        self
    }

    pub fn set_cell_size(&mut self, cell_size: f64) {
        self.cell_size = cell_size;
    }

    pub fn get_maze(&self) -> Rc<RefCell<Maze>> {
        self.maze.clone()
    }

    pub fn draw_maze(&self, c: &graphics::Context, gl: &mut opengl_graphics::GlGraphics, color_map: HashMap<Point, Option<Color>>) {
        let maze = self.maze.borrow();
        for row in maze.cells.iter() {
            for cell in row.iter().map(|rc| rc) {
                self.draw_cell(c, gl, &cell, *color_map.get(&cell.position).unwrap());
            }
        }
    }

    fn to_gui_coordinates(&self, cell: &Cell) -> (f64, f64) {
        (cell.position.x as f64 * self.cell_size, cell.position.y as f64 * self.cell_size)
    }

    fn draw_cell(&self, c: &graphics::Context, gl: &mut opengl_graphics::GlGraphics, cell: &Cell, color: Option<Color>) {
        use graphics::*;

        let cell_size = self.cell_size;
        let (x, y) = self.to_gui_coordinates(cell);

        let wall_thickness = cell_size / 10.0;

        if let Some(color) = color {
            rectangle(color, [x, y, cell_size, cell_size], c.transform, gl);
        }

        for dir in cell.walls.iter() {
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
