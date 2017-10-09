extern crate piston_window;

mod model;
mod view;

use view::*;
use piston_window::*;

fn main() {
    // make the window
    let mut window: PistonWindow = WindowSettings::new("hey hey!", [500; 2])
        .exit_on_esc(true).build().unwrap();

    //event loop
    while let Some(e) = window.next(){
        render(&mut window, &e);
    }
}
