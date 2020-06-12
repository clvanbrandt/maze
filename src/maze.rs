use std::collections::HashMap;

use graphics;
use graphics::modular_index::next;
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use rand;
use rand::seq::SliceRandom;

const WALL_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const END_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
const START_COLOR: Color = [0.0, 1.0, 0.0, 1.0];
const VISITED_COLOR: Color = [0.0, 0.0, 1.0, 1.0];

#[derive(Hash, Eq, PartialEq, Debug)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::NORTH => Direction::SOUTH,
            Direction::SOUTH => Direction::NORTH,
            Direction::EAST => Direction::WEST,
            Direction::WEST => Direction::EAST,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
}

struct Cell {
    coordinates: Point,
    walls: HashMap<Direction, bool>,
    visited: bool,
    is_start: bool,
    is_end: bool,
}

pub struct Maze {
    pub width: u32,
    pub height: u32,
    cells: HashMap<Point, Cell>,
    start: Point,
    end: Point,
}

pub struct BacktrackingGenerator {
    stack: Vec<Point>,
    maze: Maze,
    current: Point,
    done: bool,
}

pub trait MazeGenerator {
    fn initialize(&mut self);
    fn next(&mut self);
    fn generate(&mut self);
}

impl BacktrackingGenerator {
    pub fn new(maze_width: u32, maze_height: u32) -> Self {
        let stack = Vec::new();
        let maze = Maze::new(maze_width, maze_height);
        let current = maze.start;
        BacktrackingGenerator {
            stack,
            maze,
            current,
            done: false,
        }
    }

    pub fn start(mut self, x: i32, y: i32) -> Self {
        self.set_start(x, y);
        self
    }

    pub fn set_start(&mut self, x: i32, y: i32) {
        self.maze.set_start(Point { x, y });
    }

    pub fn end(mut self, x: i32, y: i32) -> Self {
        self.set_end(x, y);
        self
    }

    pub fn set_end(&mut self, x: i32, y: i32) {
        self.maze.set_end(Point { x, y });
    }

    pub fn get_maze(&self) -> &Maze {
        &self.maze
    }
}

impl MazeGenerator for BacktrackingGenerator {
    fn initialize(&mut self) {
        self.current = self.maze.start;
        self.stack.push(self.current);
        self.maze.cells.get_mut(&self.maze.start).unwrap().visited = true;
    }

    fn next(&mut self) {
        if self.stack.is_empty() {
            self.done = true;
            return;
        }
        self.current = self.stack.pop().unwrap();
        let next_cell_coord = match self.maze.get_random_unvisited_neighbor(&self.current) {
            Some(c) => {
                self.stack.push(self.current);
                c
            }
            None => return
        };
        let current_cell = self.maze.cells.get(&self.current).unwrap();
        let next_cell = self.maze.cells.get(&next_cell_coord).unwrap();
        let direction = current_cell.get_relative_direction(&next_cell);
        let other_direction = direction.opposite();

        let current_cell = self.maze.cells.get_mut(&self.current).unwrap();
        current_cell.walls.insert(direction, false);
        let next_cell = self.maze.cells.get_mut(&next_cell_coord).unwrap();
        next_cell.walls.insert(other_direction, false);
        next_cell.visited = true;
        self.stack.push(next_cell_coord);
    }

    fn generate(&mut self) {
        self.initialize();
        while !self.done { self.next(); }
    }
}

impl Maze {
    pub fn new(width: u32, height: u32) -> Self {
        let start = Point { x: 0, y: 0 };
        let end = Point { x: width as i32 - 1, y: height as i32 - 1 };
        let mut cells = HashMap::new();

        for x in 0..width as i32 {
            for y in 0..height as i32 {
                let coordinates = Point { x, y };
                let is_start = coordinates == start;
                let is_end = coordinates == end;
                cells.insert(coordinates, Cell::new(x, y, is_start, is_end));
            }
        }
        Maze {
            width,
            height,
            cells,
            start,
            end,
        }
    }

    fn get_random_unvisited_neighbor(&self, coord: &Point) -> Option<Point> {
        let cell = self.cells.get(coord).unwrap();
        let neighbors: Vec<Point> = cell.get_neighbors(self.width, self.height)
            .iter()
            .map(|x| self.cells.get(x).unwrap())
            .filter(|&c| !c.visited)
            .map(|c| c.coordinates)
            .collect();
        match neighbors.choose(&mut rand::thread_rng()) {
            Some(&n) => Some(n),
            None => None
        }
    }

    fn set_start(&mut self, new_start: Point) {
        self.cells.get_mut(&self.start).unwrap().is_start = false;
        self.cells.get_mut(&new_start).unwrap().is_start = true;
        self.start = new_start;
    }

    fn set_end(&mut self, new_end: Point) {
        self.cells.get_mut(&self.end).unwrap().is_end = false;
        self.cells.get_mut(&new_end).unwrap().is_end = true;
        self.end = new_end;
    }

    pub fn draw(&self, c: &graphics::Context, gl: &mut opengl_graphics::GlGraphics, cell_size: f64) {
        for (_, cell) in self.cells.iter() {
            cell.draw(c, gl, cell_size);
        }
    }
}

impl Cell {
    pub fn new(x: i32, y: i32, is_start: bool, is_end: bool) -> Self {
        let mut walls = HashMap::new();

        walls.insert(Direction::NORTH, true);
        walls.insert(Direction::SOUTH, true);
        walls.insert(Direction::EAST, true);
        walls.insert(Direction::WEST, true);

        Cell {
            coordinates: Point {
                x,
                y,
            },
            walls,
            visited: false,
            is_start,
            is_end,
        }
    }

    fn get_neighbors(&self, maze_width: u32, maze_height: u32) -> Vec<Point> {
        let mut potential_neighbor = Vec::new();
        for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)].iter() {
            potential_neighbor.push(Point { x: self.coordinates.x + *dx, y: self.coordinates.y + *dy });
        }
        potential_neighbor
            .into_iter()
            .filter(|p| p.x >= 0 && p.x < maze_width as i32 && p.y >= 0 && p.y < maze_height as i32)
            .collect()
    }

    fn get_relative_direction(&self, other: &Cell) -> Direction {
        let dx = other.coordinates.x - self.coordinates.x;
        if dx > 0 {
            return Direction::EAST;
        } else if dx < 0 {
            return Direction::WEST;
        }

        let dy = other.coordinates.y - self.coordinates.y;
        if dy > 0 {
            Direction::SOUTH
        } else {
            Direction::NORTH
        }
    }

    fn to_gui_coordinates(p: i32, cell_size: f64) -> f64 {
        p as f64 * cell_size
    }

    pub fn draw(&self, c: &graphics::Context, gl: &mut opengl_graphics::GlGraphics, cell_size: f64) {
        use graphics::*;
        let x = Cell::to_gui_coordinates(self.coordinates.x, cell_size);
        let y = Cell::to_gui_coordinates(self.coordinates.y, cell_size);

        let wall_thickness = cell_size / 10.0 / 2.0;

        if self.is_start {
            rectangle(START_COLOR, [x, y, cell_size, cell_size], c.transform, gl);
        } else if self.is_end {
            rectangle(END_COLOR, [x, y, cell_size, cell_size], c.transform, gl);
        } else if self.visited {
            rectangle(VISITED_COLOR, [x, y, cell_size, cell_size], c.transform, gl);
        }

        for (dir, present) in self.walls.iter().filter(|(k, v)| **v) {
            match dir {
                Direction::NORTH => {
                    rectangle(WALL_COLOR, [x, y, cell_size, wall_thickness], c.transform, gl);
                }
                Direction::SOUTH => {
                    rectangle(WALL_COLOR, [x, y + cell_size - wall_thickness, cell_size, wall_thickness], c.transform, gl);
                }
                Direction::EAST => {
                    rectangle(WALL_COLOR, [x + cell_size - wall_thickness, y, wall_thickness, cell_size], c.transform, gl);
                }
                Direction::WEST => {
                    rectangle(WALL_COLOR, [x, y, wall_thickness, cell_size], c.transform, gl);
                }
            }
        }
    }
}
