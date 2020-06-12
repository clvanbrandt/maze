use graphics;
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use crate::maze;
use crate::maze::MazeGenerator;

//const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];
const BACK_COLOR: Color = [0.9, 0.9, 0.9, 1.0];
const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];

pub struct App {
    pub gl: GlGraphics,
    // App Space
    resolution: [u32; 2],
    maze_generator: maze::BacktrackingGenerator,
    delay_between_steps: f64,
    timer: f64,
}

impl App {
    pub fn new(gl: GlGraphics, resolution: [u32; 2]) -> Self {
        let mut maze_generator = maze::BacktrackingGenerator::new(120, 60).start(1, 1);
        maze_generator.initialize();
        App {
            gl,
            resolution,
            maze_generator,
            timer: 0.0,
            delay_between_steps: 0.02,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.resolution = args.draw_size;

        let maze = self.maze_generator.get_maze();

        let cell_size_x = self.resolution[0] as f64 / maze.width as f64;
        let cell_size_y = self.resolution[1] as f64 / maze.height as f64;
        let cell_size = if cell_size_x < cell_size_y { cell_size_x } else { cell_size_y };

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACK_COLOR, gl);
            maze.draw(&c, gl, cell_size);
            //draw_borders(width, height, &c, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.timer += args.dt;
        if self.timer >= self.delay_between_steps {
            self.maze_generator.next();
            self.timer -= self.delay_between_steps;
        }
    }
}

fn draw_borders(width: &u32, height: &u32, c: &graphics::Context, gl: &mut GlGraphics) {
    use graphics::rectangle;

    let border_width = 20.0;

    let transform = c.transform;
    rectangle(BORDER_COLOR, [0.0, 0.0, *width as f64, border_width], transform, gl);
    rectangle(BORDER_COLOR, [0.0, 0.0, border_width, *height as f64], transform, gl);
    rectangle(BORDER_COLOR, [0.0, *height as f64 - border_width, *width as f64, border_width], transform, gl);
    rectangle(BORDER_COLOR, [*width as f64 - border_width, 0.0, border_width, *height as f64], transform, gl);
}