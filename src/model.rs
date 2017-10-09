struct PressureCell{
    value: f32,
    changes: f32
}

struct PressureField {
    field: Vec<Vec<PressureCell>>,
    width: i32,
    height: i32,
    max_pressure: f32,
    min_pressure: f32,
    transfer_loss: f32
}

impl PressureField {
    fn in_bounds(x: i32, y: i32, i: i32, j: i32, width: i32, height: i32) -> bool{
        true
    }

    fn update(&mut self){
        for (x, col) in self.field.iter_mut().enumerate(){
            for (y, cell) in col.iter_mut().enumerate(){
                // for each element in the field
                for i in -1..2 {
                    for j in -1..2 {
                        // for each neighbor
                        if i != 0 || j != 0 {
                            //exclude the current cell
                            if PressureField::in_bounds(x as i32, y as i32, i, j, self.width, self.height){
                                let neighbor = PressureCell{ value: self.field[x][y].value, changes: 0.0 };
                                cell.changes += (neighbor.value - cell.value)*self.transfer_loss;
                            }
                        }
                    }
                }
            }
        }
    }

}
