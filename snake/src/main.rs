extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

struct Game {
    gl: GlGraphics,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        let blue: [f32; 4] = [0.0, 0.0, 0.5, 1.0];

        self.gl
            .draw(arg.viewport(), |_c, gl| graphics::clear(blue, gl))
    }
}

fn main() {
    let opengl = OpenGL::V3_0;

    let mut window: GlutinWindow = WindowSettings::new("snake", [200, 200])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }
    }
}
