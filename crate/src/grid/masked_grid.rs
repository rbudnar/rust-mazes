use std::rc::Rc;
use std::cell::RefCell;
use super::{Grid, mask::Mask, CellFormatter, grid_base::GridBase};
use crate::cells::{ICellStrong, cell::{CellLinkStrong, Cell}};
use crate::rng::RngWrapper;

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
    fn new_cell(&self, row: usize, column: usize) -> ICellStrong {
        Cell::new(row, column)
    }

    fn prepare_grid(&mut self) {
        for i in 0..self.grid.rows {
            let mut row: Vec<Option<CellLinkStrong>> = Vec::new();
            for j in 0..self.grid.columns {
                if self.mask.borrow().get(i, j) {
                    row.push(Some(Cell::new(i as usize, j as usize)));
                } else {
                    row.push(None);
                }
            }
            self.grid.cells.push(row);
        }   
    }

    fn random_cell(&self, rng: &dyn RngWrapper<Shuffle=ICellStrong>) -> Option<ICellStrong> {
        let (row, col) = self.mask.borrow().rand_location(rng);
        
        if let Some(c) = self.grid.cells[row][col].clone() {
            return Some(c as ICellStrong);
        }
        None
    }

    fn each_cell(&self) -> Vec<Option<ICellStrong>> {
        self.grid.each_cell()
    }

    fn rows(&self) -> usize {
        self.grid.columns
    }

    fn columns(&self) -> usize {
        self.grid.rows
    }

    fn cells(&self) -> &Vec<Vec<Option<ICellStrong>>> {
        self.grid.cells()
    }

    fn get_cell(&self, row: usize, column: usize) -> Option<ICellStrong> {
        self.grid.get_cell(row, column)
    }

    fn to_string(&self, contents: &dyn CellFormatter) -> String {
        self.grid.to_string(contents)
    }

    fn size(&self) -> usize {
        self.mask.borrow().count()
    }

    fn braid(&self, p: f64, rng: &dyn RngWrapper<Shuffle=ICellStrong>) {
        self.grid.braid(p, rng);
    }

    fn to_web(&self, formatter: &dyn CellFormatter, colorize: bool) {
        self.grid.to_web(formatter, colorize);
    }
}