use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod app;
mod maze;

fn main() {
    let resolution = [1200, 600];
    let opengl = OpenGL::V3_2;

    // Create a window
    let mut window: Window = WindowSettings::new("Rust Maze Generator", resolution)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(true)
        .build()
        .unwrap();

    let mut app = app::App::new(
        GlGraphics::new(opengl),
        resolution,
    );

    // Event loop
    let mut events = Events::new(EventSettings::new());

    while let Some(event) = events.next(&mut window) {
        // Catch the events of the keyboard

        if let Some(args) = event.render_args() {
            app.render(&args);
        }

        if let Some(args) = event.update_args() {
            app.update(&args);
        }
    }
}