use std::collections::HashMap;

use graphics::types::Color;
use opengl_graphics::GlGraphics;
use piston::input::{ButtonArgs, ButtonState, Key, RenderArgs, UpdateArgs};
use piston::input::Button::Keyboard;
use piston::window;

use crate::maze;
use crate::maze::BacktrackingCellState;
use crate::maze::Point;

//const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];
const BACK_COLOR: Color = [0.9, 0.9, 0.9, 1.0];
//const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];

const VISITED_COLOR: Color = [0.0, 0.0, 1.0, 1.0];
const CURRENT_COLOR: Color = [1.0, 1.0, 0.0, 1.0];
const END_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
const START_COLOR: Color = [0.0, 1.0, 0.0, 1.0];

pub struct App {
    pub gl: GlGraphics,
    // App Space
    resolution: window::Size,
    maze_generator: maze::BacktrackingGenerator,
    maze_drawer: maze::Drawer,
    delay_between_steps: f64,
    timer: f64,
}

impl App {
    fn cell_size(&self) -> f64 {
        let cell_size_x = self.resolution.width as f64 / self.maze_generator.width as f64;
        let cell_size_y = self.resolution.height as f64 / self.maze_generator.height as f64;
        if cell_size_x < cell_size_y { cell_size_x } else { cell_size_y }
    }

    pub fn new(gl: GlGraphics, resolution: window::Size) -> Self {
        let maze_generator = maze::BacktrackingGenerator::new(100, 60);
        let maze = maze_generator.get_maze();
        let maze_drawer = maze::Drawer::new(maze);

        let mut app = Self {
            gl,
            resolution,
            maze_generator,
            timer: 0.0,
            delay_between_steps: 0.003,
            maze_drawer,
        };
        app.maze_drawer.set_cell_size(app.cell_size());
        app
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.resolution = window::Size::from(args.draw_size);
        let cell_size = self.cell_size();

        let maze_drawer = &mut self.maze_drawer;
        maze_drawer.set_cell_size(cell_size);

        let state = self.maze_generator.get_cells_state();
        let color_map: HashMap<Point, Option<Color>> = state
            .iter()
            .map(|(&a, b)| {
                if a == maze_drawer.get_maze().borrow().get_start() {
                    (a, Some(START_COLOR))
                } else if a == maze_drawer.get_maze().borrow().get_end() {
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

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACK_COLOR, gl);

            maze_drawer.draw_maze(&c, gl, color_map);
            //draw_borders(width, height, &c, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.timer += args.dt;
        if self.timer >= self.delay_between_steps {
            let number_of_steps = (self.timer / self.delay_between_steps) as i32;
            for _ in 0..number_of_steps {
                self.timer -= self.delay_between_steps;
                self.maze_generator.next_iter();
                if self.maze_generator.is_done() {
                    break;
                }
            }
        }
    }

    pub fn input(&mut self, args: &ButtonArgs) {
        if let Keyboard(key) = args.button {
            if let Key::R = key {
                if args.state == ButtonState::Press {
                    self.maze_generator.restart();
                    self.timer = 0.0;
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