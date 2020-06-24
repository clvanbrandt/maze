use std::collections::HashMap;

use graphics::types::Color;
use opengl_graphics::GlGraphics;
use piston::input::Button::Keyboard;
use piston::input::{ButtonArgs, ButtonState, Key, RenderArgs, UpdateArgs};
use piston::window;

use maze::generation;
use maze::generation::BacktrackingCellState;
use maze::maze::{Cell, Direction, Maze, Point};
use maze::solving::{AStarSolver, Path};

//const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];
const BACK_COLOR: Color = [0.9, 0.9, 0.9, 1.0];
//const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];

const VISITED_COLOR: Color = [0.0, 0.0, 1.0, 1.0];
const CURRENT_COLOR: Color = [1.0, 1.0, 0.0, 1.0];
const END_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
const START_COLOR: Color = [0.0, 1.0, 0.0, 1.0];
const WALL_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const PATH_COLOR: Color = [0.0, 48.0, 78.0, 1.0];

pub struct App {
    pub gl: GlGraphics,
    // App Space
    resolution: window::Size,
    maze_generator: generation::BacktrackingGenerator,
    maze_drawer: MazeDrawer,
    maze_solver: Option<AStarSolver>,
    delay_between_steps: f64,
    timer: f64,
    paused: bool,
    path: Option<Path>,
}

impl App {
    fn cell_size(&self) -> f64 {
        let cell_size_x = self.resolution.width as f64 / self.maze_generator.width as f64;
        let cell_size_y = self.resolution.height as f64 / self.maze_generator.height as f64;
        if cell_size_x < cell_size_y {
            cell_size_x
        } else {
            cell_size_y
        }
    }

    pub fn new(gl: GlGraphics, resolution: window::Size) -> Self {
        let maze_generator = generation::BacktrackingGenerator::new(30, 20);
        let maze_drawer = MazeDrawer::new();

        let mut app = Self {
            gl,
            resolution,
            maze_generator,
            timer: 0.0,
            delay_between_steps: 0.003,
            maze_drawer,
            paused: false,
            maze_solver: None,
            path: None,
        };
        app.maze_drawer.set_cell_size(app.cell_size());
        app
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.resolution = window::Size::from(args.window_size);
        let cell_size = self.cell_size();

        let maze = self.maze_generator.get_maze_ref();
        let start = maze.get_start();
        let end = maze.get_end();

        let maze_drawer = &mut self.maze_drawer;
        maze_drawer.set_cell_size(cell_size);

        let state = self.maze_generator.get_cells_state();
        let mut color_map: HashMap<Point, Option<Color>> = state
            .iter()
            .map(|(&a, b)| {
                if a == start {
                    (a, Some(START_COLOR))
                } else if a == end {
                    (a, Some(END_COLOR))
                } else {
                    match b {
                        BacktrackingCellState::Unvisited => (a, None),
                        BacktrackingCellState::Visited => (a, Some(VISITED_COLOR)),
                        BacktrackingCellState::Current => (a, Some(CURRENT_COLOR)),
                    }
                }
            })
            .collect();

        if let Some(path) = &self.path {
            for node in path.iter().filter(|&&p| p != start && p != end) {
                color_map.insert(*node, Some(PATH_COLOR));
            }
        }

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACK_COLOR, gl);

            maze_drawer.draw_maze(&c, gl, maze, color_map);
            //draw_borders(width, height, &c, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if !self.paused {
            self.timer += args.dt;
            if self.timer >= self.delay_between_steps {
                let number_of_steps = (self.timer / self.delay_between_steps) as i32;
                for _ in 0..number_of_steps {
                    self.timer -= self.delay_between_steps;
                    self.maze_generator.next_step();
                    if self.maze_generator.is_done() {
                        break;
                    }
                }
            }
        }
    }

    pub fn input(&mut self, args: &ButtonArgs) {
        if let Keyboard(key) = args.button {
            if let Key::R = key {
                if args.state == ButtonState::Press {
                    self.maze_generator.restart();
                    self.path = None;
                    self.timer = 0.0;
                }
            } else if let Key::P = key {
                if args.state == ButtonState::Press {
                    self.paused = !self.paused
                }
            } else if let Key::S = key {
                if args.state == ButtonState::Press {
                    let mut maze_solver =
                        AStarSolver::new(self.maze_generator.get_maze_ref().clone());
                    self.path = maze_solver.solve();
                    self.maze_solver = Some(maze_solver);
                }
            }
        }
    }
}

pub struct MazeDrawer {
    x_offset: f64,
    y_offset: f64,
    cell_size: f64,
}

#[allow(dead_code)]
impl MazeDrawer {
    pub fn new() -> Self {
        Self {
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

    pub fn draw_maze(
        &self,
        c: &graphics::Context,
        gl: &mut opengl_graphics::GlGraphics,
        maze: &Maze,
        color_map: HashMap<Point, Option<Color>>,
    ) {
        for row in maze.get_cells().iter() {
            for cell in row.iter().map(|rc| rc) {
                self.draw_cell(c, gl, &cell, *color_map.get(&cell.position).unwrap());
            }
        }
    }

    fn to_gui_coordinates(&self, cell: &Cell) -> (f64, f64) {
        (
            cell.position.x as f64 * self.cell_size,
            cell.position.y as f64 * self.cell_size,
        )
    }

    fn draw_cell(
        &self,
        c: &graphics::Context,
        gl: &mut opengl_graphics::GlGraphics,
        cell: &Cell,
        color: Option<Color>,
    ) {
        use graphics::*;

        let cell_size = self.cell_size;
        let (x, y) = self.to_gui_coordinates(cell);

        let wall_thickness = cell_size / 15.0;

        if let Some(color) = color {
            rectangle(color, [x, y, cell_size, cell_size], c.transform, gl);
        }

        for dir in cell.get_walls().iter() {
            match dir {
                Direction::North => {
                    rectangle(
                        WALL_COLOR,
                        [x, y, cell_size, wall_thickness],
                        c.transform,
                        gl,
                    );
                }
                Direction::South => {
                    rectangle(
                        WALL_COLOR,
                        [x, y + cell_size - wall_thickness, cell_size, wall_thickness],
                        c.transform,
                        gl,
                    );
                }
                Direction::East => {
                    rectangle(
                        WALL_COLOR,
                        [x + cell_size - wall_thickness, y, wall_thickness, cell_size],
                        c.transform,
                        gl,
                    );
                }
                Direction::West => {
                    rectangle(
                        WALL_COLOR,
                        [x, y, wall_thickness, cell_size],
                        c.transform,
                        gl,
                    );
                }
            }
        }
    }
}

/* fn draw_borders(width: &u32, height: &u32, c: &graphics::Context, gl: &mut GlGraphics) {
    use graphics::rectangle;

    let border_width = 20.0;

    let transform = c.transform;
    rectangle(BORDER_COLOR, [0.0, 0.0, *width as f64, border_width], transform, gl);
    rectangle(BORDER_COLOR, [0.0, 0.0, border_width, *height as f64], transform, gl);
    rectangle(BORDER_COLOR, [0.0, *height as f64 - border_width, *width as f64, border_width], transform, gl);
    rectangle(BORDER_COLOR, [*width as f64 - border_width, 0.0, border_width, *height as f64], transform, gl);
} */
