extern crate piston_window;

mod model;
mod view;

use view::*;
use model::*;
use piston_window::*;

fn main() {
    // make the window
    let mut window: PistonWindow = WindowSettings::new("hey hey!", [500; 2])
        .exit_on_esc(true).build().unwrap();

    // make the model
    let mut m = PressureField::new(100, 100);

    //event loop
    while let Some(e) = window.next(){
        m.update();
        render(&mut window, &e, &m.changes, m.max_change, m.min_change);
    }
}
