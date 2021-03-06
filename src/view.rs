extern crate piston_window;
extern crate interpolation;

use piston_window::*;

use self::interpolation::{Spatial, lerp};

use model::PressureField;

struct Color {
    color: [f32; 4],
}

impl Spatial for Color{
    type Scalar = f32;
    fn add(&self, other: &Self) -> Self {
        let mut new_color = Color{color: [0.0; 4]};
        for (i, c) in self.color.iter().enumerate(){
            new_color.color[i] = c + other.color[i];
        }
        return new_color;
    }

    fn sub(&self, other: &Self) -> Self {
        let mut new_color = Color{color: [0.0; 4]};
        for (i, c) in self.color.iter().enumerate(){
            new_color.color[i] = c - other.color[i];
        }
        return new_color;
    }

    fn scale(&self, scalar: &Self::Scalar) -> Self {
        let mut new_color = Color{color: [0.0; 4]};
        for (i, c) in self.color.iter().enumerate(){
            new_color.color[i] = c * scalar;
        }
        return new_color;
    }
}

fn get_color(value: f64, max: f64, min: f64) -> [f32; 4] {
    let scalar = ((value - min)/(max - min)) as f32; // scale the value from 0 to 1

    let low_color = Color{color: [0.537, 0.0, 0.808, 1.0]};
    let high_color = Color{color: [0.141, 0.957, 0.741, 1.0]};

    let new_color = lerp::<Color>(&low_color, &high_color, &scalar);
    return new_color.color;
}

pub fn render(window: &mut PistonWindow, e: &Event, field: &PressureField) {
    let window_width = window.window.size().width;
    let window_height = window.window.size().height;
    window.draw_2d(e, |c, g| {
        for (i, col) in field.field.iter().enumerate(){
            for (j, cell) in col.iter().enumerate(){
                //let color = get_color(field.changes[i][j], field.max_change, field.min_change);
                let color = get_color(*cell, field.max_pressure, field.min_pressure);
                let h_unit = window_width / field.width as u32;
                let v_unit = window_height / field.height as u32;
                
                let (h_unit, v_unit, i, j) = (h_unit as f64, v_unit as f64, i as f64, j as f64);
                rectangle(color,
                          [h_unit * i, v_unit * j, h_unit, v_unit], // position and dimensions f64
                          c.transform, g);
            }
        }
    });
}
