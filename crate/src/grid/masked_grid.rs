use web_sys::Element;
use web_sys::Document;
use crate::grid::cell::ICellStrong;
use crate::grid::cell::ICell;
use crate::grid::CellFormatter;
use crate::grid::grid_base::GridBase;
use crate::rng::RngWrapper;
use crate::grid::{cell::CellLinkStrong, Grid, mask::Mask, cell::Cell};
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
        masked_grid.grid.configure_cells_i();
        masked_grid
    }    
}

impl Grid for MaskedGrid {

    fn new_cell(&self, row: usize, column: usize, index: usize) -> ICellStrong {
        Cell::new(row, column, index)
    }

    fn prepare_grid(&mut self) {
        for i in 0..self.grid.rows {
            let mut row: Vec<Option<CellLinkStrong>> = Vec::new();
            let mut index = 0;
            for j in 0..self.grid.columns {
                if self.mask.borrow().get(i, j) {
                    row.push(Some(Cell::new(i as usize, j as usize, index)));
                    index += 1;
                } else {
                    row.push(None);
                }
            }
            self.grid.cells.push(row);
        }   
    }

    fn random_cell(&self, rng: &dyn RngWrapper) -> Option<ICellStrong> {
        self.grid.random_cell(rng)
    }

    fn each_cell(&self) -> Vec<Option<ICellStrong>> {
        self.grid.each_cell()
    }

    fn get_cell_at_index(&self, index: usize) -> ICellStrong {
        return self.grid.get_cell_at_index(index);
    }

    fn get_cell_links(&self, index: usize) -> Vec<ICellStrong> {
        return self.grid.get_cell_links(index);
    }

    fn rows(&self) -> usize {
        self.grid.columns
    }

    fn columns(&self) -> usize {
        self.grid.rows
    }

    fn cells(&self) -> Vec<Vec<Option<ICellStrong>>> {
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

    fn to_web(&self, document: &Document, grid_container: &Element, formatter: &dyn CellFormatter, colorize: bool) {
        self.grid.to_web(document, grid_container, formatter, colorize);
    }
}