

use cell::CellLinkStrong;
use grid::GridBuilder;
use grid::Grid;
use cell::Cell;
use std::rc::Rc;
use std::cell::RefCell;
use mask::Mask;

pub struct MaskedGrid {
    mask: Rc<Mask>,
    pub grid: Grid
}

impl MaskedGrid {
    pub fn new(mask: Mask) -> MaskedGrid {
        
        let rows = mask.rows;
        let columns = mask.columns;
        let mask_rc = Rc::new(mask);
        let builder = MaskedGridBuilder::new(mask_rc.clone());

        MaskedGrid {
            mask: mask_rc.clone(), 
            grid: Grid::new(rows, columns, &builder)
        }
    }
}

pub struct MaskedGridBuilder {
    mask: Rc<Mask>
}

impl MaskedGridBuilder {
    pub fn new(mask: Rc<Mask>) -> MaskedGridBuilder {
        MaskedGridBuilder {
            mask
        }
    }
}

impl GridBuilder for MaskedGridBuilder {
    fn prepare_grid(&self, grid: &mut Grid) {
        for i in 0..grid.rows {
            let mut row: Vec<Option<CellLinkStrong>> = Vec::new();
            for j in 0..grid.columns {
                if self.mask.get(i, j) {
                    row.push(Some(Rc::new(RefCell::new(Cell::new(i as usize, j as usize)))));
                } else {
                    row.push(None);
                }
            }
            grid.cells.push(row);
        }   
    }
}