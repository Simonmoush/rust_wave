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
    let mut model = PressureField::new(50, 50);

    //event loop
    while let Some(e) = window.next(){
        if let Event::Loop(l) = e {
            if let Loop::Render(r) = l {
                render(&mut window, &e, &model);
            } else if let Loop::Update(u) = l {
                model.update();
            }
        }else if let Event::Input(i) = e {
            if let Input::Move(m) = i {
                if let Motion::MouseCursor(x, y) = m {
                    model.ripple(x, y, window.window.size().width, window.window.size().height);
                }
            }else if let Input::Button(b) = i {

            }
        }
    }
}
