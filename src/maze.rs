use std::cmp::Ordering;
use std::collections::HashSet;

use derive_more::{Add, Sub};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Self {
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
    pub x: usize,
    pub y: usize,
}

impl std::convert::From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point { x, y }
    }
}

impl Point {
    pub fn get_neighbors(&self, x_limit: usize, y_limit: usize) -> Vec<Point> {
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

    pub fn get_relative_direction(&self, other: &Point) -> Direction {
        match other.x.cmp(&self.x) {
            Ordering::Greater => Direction::East,
            Ordering::Less => Direction::West,
            Ordering::Equal => {
                match other.y.cmp(&self.y) {
                    Ordering::Greater => Direction::South,
                    Ordering::Less => Direction::North,
                    Ordering::Equal => panic!("Trying to remove a wall between a cell and itself")
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Cell {
    pub position: Point,
    walls: HashSet<Direction>,
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

    pub fn get_walls(&self) -> &HashSet<Direction> {
        &self.walls
    }

    pub fn remove_wall(&mut self, direction: &Direction) {
        self.walls.remove(direction);
    }

    pub fn add_wall(&mut self, direction: Direction) {
        self.walls.insert(direction);
    }

    pub fn get_walls_mut(&mut self) -> &mut HashSet<Direction> {
        &mut self.walls
    }
}

#[derive(Clone)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    cells: Vec<Vec<Cell>>,
    start: Point,
    end: Point,
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

    pub fn get_cell_mut(&mut self, p: Point) -> &mut Cell {
        self.cells.get_mut(p.x).unwrap().get_mut(p.y).unwrap()
    }

    pub fn get_cell(&self, p: Point) -> &Cell {
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

    pub fn get_cells(&self) -> &Vec<Vec<Cell>> {
        &self.cells
    }
}
