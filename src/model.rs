pub struct PressureField {
    pub field: Vec<Vec<f64>>,
    pub changes: Vec<Vec<f64>>,
    pub width: usize,
    pub height: usize,
    pub max_pressure: f64,
    pub min_pressure: f64,
    pub max_change: f64,
    pub min_change: f64,
    pub transfer_loss: f64
}

impl PressureField {
    pub fn new(width: usize, height: usize) -> PressureField {
        let mut field: Vec<Vec<f64>> = Vec::with_capacity(width);
        let mut changes: Vec<Vec<f64>> = Vec::with_capacity(width);
        for x in 0..width {
            let x = x as usize;
            field.push(Vec::with_capacity(height));
            changes.push(Vec::with_capacity(height));
            for _ in 0..height{
                field[x].push(0.0);
                changes[x].push(0.0);
            }
        }

        PressureField{
            field,
            changes,
            width,
            height,
            transfer_loss: 0.01,
            min_change: 0.0,
            max_change: 0.0,
            max_pressure: 0.0,
            min_pressure: 0.0
        }
    }

    pub fn with_dot(width: usize, height: usize) -> PressureField {
        let mut field: Vec<Vec<f64>> = Vec::with_capacity(width);
        let mut changes: Vec<Vec<f64>> = Vec::with_capacity(width);
        for x in 0..width {
            let x = x as usize;
            field.push(Vec::with_capacity(height));
            changes.push(Vec::with_capacity(height));
            for _ in 0..height{
                field[x].push(0.0);
                changes[x].push(0.0);
            }
        }

        field[width/2][height/2] = 50.0;

        PressureField{
            field,
            changes,
            width,
            height,
            transfer_loss: 0.1,
            min_change: 0.0,
            max_change: 0.0,
            max_pressure: 0.0,
            min_pressure: 0.0
        }
    }
    
    fn in_bounds(&self, x: i32, y: i32, i: i32, j: i32) -> bool{ // change to just take x and y
        let check_x = x + i;
        let check_y = y + j;
        let width = self.width as i32;
        let height = self.height as i32;
        if check_x < width && check_x >= 0 {
            if check_y < height && check_y >= 0 {
                return true
            }
        }
        false
    }

    pub fn ripple(&mut self, x: f64, y: f64, win_x: u32, win_y: u32){
        let h_unit = win_x / self.width as u32;
        let v_unit = win_y / self.height as u32;

        let x_cell = (x as u32 / h_unit) as usize;
        let y_cell = (y as u32 / v_unit) as usize;
        
        if self.in_bounds(x_cell as i32, y_cell as i32, 0, 0){
            self.changes[x_cell][y_cell] += 50.0;
        }

    }

    pub fn update(&mut self){
        // scan the field and record changes
        for (x, col) in self.field.iter().enumerate(){
            for (y, cell) in col.iter().enumerate(){
                // for each element in the field
                for i in -1..2 {
                    for j in -1..2 {
                        // for each neighbor
                        if i != 0 || j != 0 {
                            //exclude the current cell

                            // cast to integers before comparison to allow out of bounds detection
                            let x = x as i32;
                            let y = y as i32;
                            if self.in_bounds(x, y, i, j){
                                let n_x = x  + i;
                                let n_y = y + j;

                                // cast back to usize for indexing
                                let (x, y, n_x, n_y) = (x as usize, y as usize, n_x as usize, n_y as usize);

                                let neighbor = self.field[n_x][n_y];
                                self.changes[x][y] += (neighbor - cell)*self.transfer_loss;
                            }
                        }
                    }
                }
            }
        }

        // apply the changes
        for (x, col) in self.field.iter_mut().enumerate(){
            for (y, cell) in col.iter_mut().enumerate(){
                
                *cell += self.changes[x][y]; // check that this works

                *cell *= 0.995; // add some damping

                // keep track of the min and max pressure and change
                self.max_pressure = if self.max_pressure < *cell { *cell } else { self.max_pressure };
                self.min_pressure = if self.min_pressure > *cell { *cell } else { self.min_pressure };
                self.max_change = if self.max_change < *cell { *cell } else { self.max_change };
                self.min_change = if self.min_change > *cell { *cell } else { self.min_change };
            }
        }
    }
}
