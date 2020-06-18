use graphics::types::Color;
use opengl_graphics::GlGraphics;
use piston::input::{ButtonArgs, ButtonState, Key, RenderArgs, UpdateArgs};
use piston::input::Button::Keyboard;
use piston::window;

use crate::maze;

//const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];
const BACK_COLOR: Color = [0.9, 0.9, 0.9, 1.0];
//const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];

pub struct App {
    pub gl: GlGraphics,
    // App Space
    resolution: window::Size,
    maze_generator: maze::BacktrackingGenerator,
    delay_between_steps: f64,
    timer: f64,
}

impl App {
    pub fn new(gl: GlGraphics, resolution: window::Size) -> Self {
        let mut maze_generator = maze::BacktrackingGenerator::new(100, 60);
        maze_generator.initialize();
        App {
            gl,
            resolution,
            maze_generator,
            timer: 0.0,
            delay_between_steps: 0.001,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.resolution = window::Size::from(args.draw_size);

        let maze = self.maze_generator.get_maze_ref();

        let cell_size_x = self.resolution.width as f64 / maze.width as f64;
        let cell_size_y = self.resolution.height as f64 / maze.height as f64;
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
            let number_of_steps = (self.timer / self.delay_between_steps) as i32;
            for _ in 0..number_of_steps {
                self.timer -= self.delay_between_steps;
                if self.maze_generator.next() == Ok(maze::GeneratorState::Done) {
                    break;
                }
            }
        }
    }

    pub fn input(&mut self, args: &ButtonArgs) {
        if let Keyboard(key) = args.button {
            if let Key::R = key {
                if args.state == ButtonState::Press {
                    self.maze_generator.restart()
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