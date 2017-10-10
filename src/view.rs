extern crate piston_window;
extern crate interpolation;

use piston_window::*;

use self::interpolation::{Spatial, lerp};


struct Color {
    color: [f32; 4],
}

impl Color {
    fn red(&self) -> f32{
        self.color[0]
    }
    fn set_red(&mut self, val: f32){
        self.color[0] = val;
    }

    fn green(&self) -> f32{
        self.color[1]
    }
    fn set_green(&mut self, val: f32){
        self.color[1] = val;
    }

    fn blue(&self) -> f32{
        self.color[2]
    }
    fn set_blue(&mut self, val: f32){
        self.color[2] = val;
    }

    fn alpha(&self) -> f32{
        self.color[3]
    }
    fn set_alpha(&mut self, val: f32){
        self.color[3] = val;
    }
}

impl Spatial for Color{
    type Scalar = f32;
    fn add(&self, other: &Self) -> Self {
        let mut newColor = Color{color: [0.0; 4]};
        for (i, c) in self.color.iter().enumerate(){
            newColor.color[i] = c + other.color[i];
        }
        return newColor;
    }

    fn sub(&self, other: &Self) -> Self {
        let mut newColor = Color{color: [0.0; 4]};
        for (i, c) in self.color.iter().enumerate(){
            newColor.color[i] = c - other.color[i];
        }
        return newColor;
    }

    fn scale(&self, scalar: &Self::Scalar) -> Self {
        let mut newColor = Color{color: [0.0; 4]};
        for (i, c) in self.color.iter().enumerate(){
            newColor.color[i] = c * scalar;
        }
        return newColor;
    }
}

fn get_color(value: f32, max: f32, min: f32) -> [f32; 4] {
    let scalar = (value - min)/(max - min); // scale the value from 0 to 1

    let low_color = Color{color: [0.537, 0.0, 0.808, 1.0]};
    let high_color = Color{color: [0.141, 0.957, 0.741, 1.0]};

    let new_color = lerp::<Color>(&low_color, &high_color, &scalar);
    return new_color.color;
}

pub fn render(window: &mut PistonWindow, e: &Event, field: &Vec<Vec<f32>>, max: f32, min: f32) {
    window.draw_2d(e, |c, g| {
        clear([1.0; 4], g);
        for col in field.iter(){
            for cell in col.iter(){
                let color = get_color(*cell, max, min);
                rectangle(color,
                          [0.0, 0.0, 100.0, 100.0], // position and dimensions
                          c.transform, g);
            }
        }
    });
}
