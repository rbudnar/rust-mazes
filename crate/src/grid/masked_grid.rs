use grid::CellFormatter;
use grid::grid_base::GridBase;
use rng::RngWrapper;
use grid::{CellLinkStrong, Grid, mask::Mask, cell::Cell};
use std::rc::Rc;
use std::cell::RefCell;

pub struct MaskedGrid {
    mask: Rc<RefCell<Mask>>,
    pub grid: GridBase
}

impl MaskedGrid {
    pub fn new(mask: Mask) -> MaskedGrid {        
        let rows = mask.rows;
        let columns = mask.columns;
        let mask_rc = Rc::new(RefCell::new(mask));
        let grid = GridBase::new(rows, columns); 
                
        let mut masked_grid = MaskedGrid {
            mask: mask_rc.clone(), 
            grid
        };

        masked_grid.prepare_grid();
        masked_grid.grid.configure_cells();
        masked_grid
    }    
}

impl Grid for MaskedGrid {
    fn prepare_grid(&mut self) {
        for i in 0..self.grid.rows {
            let mut row: Vec<Option<CellLinkStrong>> = Vec::new();
            for j in 0..self.grid.columns {
                if self.mask.borrow().get(i, j) {
                    row.push(Some(Rc::new(RefCell::new(Cell::new(i as usize, j as usize)))));
                } else {
                    row.push(None);
                }
            }
            self.grid.cells.push(row);
        }   
    }

    fn random_cell(&self, rng: &RngWrapper) -> Option<CellLinkStrong> {
        let (row, col) = self.mask.borrow().rand_location(rng);
        self.grid.cells[row][col].clone()
    }

    fn each_cell(&self) -> Vec<Option<CellLinkStrong>> {
        self.grid.each_cell()
    }

    fn rows(&self) -> usize {
        self.grid.columns
    }

    fn columns(&self) -> usize {
        self.grid.rows
    }

    fn cells(&self) -> &Vec<Vec<Option<CellLinkStrong>>> {
        &self.grid.cells
    }

    fn get_cell(&self, row: usize, column: usize) -> Option<CellLinkStrong> {
        self.grid.get_cell(row, column)
    }

    fn to_string(&self, contents: &CellFormatter) -> String {
        self.grid.to_string(contents)
    }

    fn size(&self) -> usize {
        self.mask.borrow().count()
    }
}