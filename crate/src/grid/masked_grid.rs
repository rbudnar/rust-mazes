use crate::grid::cell::ICell;
use crate::grid::CellFormatter;
use crate::grid::grid_base::GridBase;
use crate::rng::RngWrapper;
use crate::grid::{cell::CellLinkStrong, Grid, mask::Mask, cell::Cell};
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::{Element, Document};

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
    fn new_cell(&self, row: usize, column: usize, index: usize) -> ICellStrong {
        Cell::new(row, column, index)
    }

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

    fn random_cell(&self, rng: &dyn RngWrapper) -> Option<Box<ICell>> {
        let (row, col) = self.mask.borrow().rand_location(rng);
        Some(Box::new(*self.grid.cells[row][col].clone().unwrap().borrow()) as Box<ICell>)
    }

    fn each_cell(&self) -> Vec<Option<Box<ICell>>> {
        self.grid.each_cell().iter()
            .map(|c| Some(Box::new(*c.unwrap().borrow()) as Box<ICell>)).collect()
    }

    fn rows(&self) -> usize {
        self.grid.columns
    }

    fn columns(&self) -> usize {
        self.grid.rows
    }

    fn cells(&self) -> &Vec<Vec<Option<Box<ICell>>>> {
        &self.grid.cells.iter().map(|row| 
            row.iter().map(|c| Some(Box::new(*c.unwrap().borrow()) as Box<ICell>)).collect()
        ).collect()
    }

    fn get_cell(&self, row: usize, column: usize) -> Option<Box<ICell>> {
        let cell = self.grid.get_cell(row, column);

        Some(Box::new(*cell.unwrap().borrow()) as Box<ICell>)
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