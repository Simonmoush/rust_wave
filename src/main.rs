extern crate piston_window;

use piston_window::*;


fn main() {
    // make the window
    let mut window: PistonWindow = WindowSettings::new("hey hey!", [500; 2])
        .build().unwrap();

    //event loop
    while let Some(e) = window.next(){
        render(&mut window, e);
    }
}

fn render(window: &mut PistonWindow, e: &Event) {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle([1.0, 1.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0], // position and dimensions
                      c.transform, g);
        });
}
