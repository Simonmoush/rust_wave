extern crate piston_window;

mod model;
mod view;

use view::*;
use model::*;
use piston_window::*;

fn main() {
    // make the window
    let mut window: PistonWindow = WindowSettings::new("waves", [500; 2])
        .exit_on_esc(true).build().unwrap();

    // make the model
    let mut m = PressureField::with_dot(50, 50);

    //event loop
    while let Some(e) = window.next(){
        match e {
            Event::Loop(l) => {
                match l {
                    Loop::Render(r) => render(&mut window, &e, &m),
                    Loop::Update(u) => m.update(),
                    _ => (),
                }
            },
            _ => (),
        }
    }
}
