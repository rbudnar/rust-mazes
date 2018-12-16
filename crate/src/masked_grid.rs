use rng::RngWrapper;
use cell::CellLinkStrong;
use grid::GridBuilder;
use grid::Grid;
use cell::Cell;
use std::rc::Rc;
use std::cell::RefCell;
use mask::Mask;

pub struct MaskedGrid {
    mask: Rc<RefCell<Mask>>,
    pub grid: Grid
}

impl MaskedGrid {
    pub fn new(mask: Mask) -> MaskedGrid {
        
        let rows = mask.rows;
        let columns = mask.columns;
        let mask_rc = Rc::new(RefCell::new(mask));
        let builder = MaskedGridBuilder::new(Rc::clone(&mask_rc));
        let mut grid = Grid::new(rows, columns, &builder); 
        builder.prepare_grid(&mut grid);
        grid.configure_cells();

        MaskedGrid {
            mask: mask_rc.clone(), 
            grid
        }
    }

    fn random_cell(&self, rng: &RngWrapper) -> Option<CellLinkStrong> {
        let (row, col) = self.mask.borrow().rand_location(rng);
        self.grid.cells[row][col].clone()
    }

    fn size(&self) -> usize {
        self.mask.borrow().count()
    }

    fn prepare_grid(&self, grid: &mut Grid) {
        for i in 0..grid.rows {
            let mut row: Vec<Option<CellLinkStrong>> = Vec::new();
            for j in 0..grid.columns {
                if self.mask.borrow().get(i, j) {
                    row.push(Some(Rc::new(RefCell::new(Cell::new(i as usize, j as usize)))));
                } else {
                    row.push(None);
                }
            }
            grid.cells.push(row);
        }   
    }
}

pub struct MaskedGridBuilder {
    mask: Rc<RefCell<Mask>>
}

impl MaskedGridBuilder {
    pub fn new(mask: Rc<RefCell<Mask>>) -> MaskedGridBuilder {
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
                if self.mask.borrow().get(i, j) {
                    row.push(Some(Rc::new(RefCell::new(Cell::new(i as usize, j as usize)))));
                } else {
                    row.push(None);
                }
            }
            grid.cells.push(row);
        }   
    }
}