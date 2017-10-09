extern crate piston_window;
use piston_window::*;

pub fn render(window: &mut PistonWindow, e: &Event) {
        window.draw_2d(e, |c, g| {
            clear([1.0; 4], g);
            rectangle([1.0, 1.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0], // position and dimensions
                      c.transform, g);
        });
}
