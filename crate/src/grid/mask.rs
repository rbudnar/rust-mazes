use rng::RngWrapper;

#[derive(Debug)]
pub struct Mask {
    pub rows: usize,
    pub columns: usize,
    pub bits: Vec<Vec<bool>>
}

impl Mask {
    pub fn new(rows: usize, columns: usize) -> Mask {
        Mask {
            rows, columns,
            bits: vec![vec![true; columns]; rows]
        }
    }

    pub fn get(&self, row: usize, column: usize) -> bool {
         if (0..self.rows).contains(&row) && (0..self.columns).contains(&column) {
            self.bits[row][column]
        }   
        else{
            false
        }
    }

    pub fn set(&mut self, row: usize, column: usize, is_on: bool) {
        self.bits[row][column] = is_on;
    }

    pub fn count(&self) -> usize {
        self.bits.iter()
            .fold(0, |acc, row| acc + row.iter().fold(0, |acc, &col| acc + if col { 1 } else { 0 }))
    }

    pub fn rand_location(&self, rng: &RngWrapper) -> (usize, usize) {
        loop {
            let row = rng.gen_range(0, self.rows);
            let col = rng.gen_range(0, self.columns);
            
            if self.bits[row][col] {
                return (row, col);
            }
        }        
    }    
}
