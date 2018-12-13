

use cell::CellLinkStrong;
use grid::GridBuilder;
use grid::StandardGridBuilder;
use grid::Grid;
use cell::Cell;
use std::rc::Rc;
use std::cell::RefCell;
use mask::Mask;

pub struct MaskedGrid {
    mask: Mask,
    grid: Grid
}

impl MaskedGrid {
    pub fn new(mask: Mask) -> MaskedGrid {
        let rows = mask.rows;
        let columns = mask.columns;
        MaskedGrid {
            mask, grid: Grid::new(rows, columns, &StandardGridBuilder)
        }
    }

    pub fn prepare_grid() {

    }
}

pub struct MaskedGridBuilder;
impl GridBuilder for MaskedGridBuilder {
    fn prepare_grid(&self, grid: &mut Grid) {
        for i in 0..grid.rows {
            let mut row: Vec<CellLinkStrong> = Vec::new();
            for j in 0..grid.columns {
                row.push(Rc::new(RefCell::new(Cell::new(i as usize, j as usize))));
            }
            grid.cells.push(row);
        }   
    }
}