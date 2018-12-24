use crate::grid::{grid_base::GridBase, Grid, CellFormatter, cell::{CellLinkStrong, Cell}};
use crate::rng::RngWrapper;
use std::rc::{Rc};
use std::cell::RefCell;

pub struct StandardGrid {
    pub grid: GridBase,
}

impl StandardGrid {
    pub fn new(rows: usize, columns: usize) -> StandardGrid {
        let grid = GridBase::new(rows, columns); 
        let mut std_grid = StandardGrid {
            grid
        };
        std_grid.prepare_grid();
        std_grid.grid.configure_cells();

        std_grid
    }
}

impl Grid for StandardGrid {
    fn prepare_grid(&mut self) {
        for i in 0..self.grid.rows {
            let mut row: Vec<Option<CellLinkStrong>> = Vec::new();
            for j in 0..self.grid.columns {
                row.push(Some(Rc::new(RefCell::new(Cell::new(i as usize, j as usize)))));
            }
            self.grid.cells.push(row);
        }   
    }

    fn random_cell(&self, rng: &dyn RngWrapper) -> Option<CellLinkStrong> {
        let row: usize = rng.gen_range(0, self.grid.rows);
        let col: usize = rng.gen_range(0, self.grid.columns);
        self.grid.get_cell(row, col)
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

    fn to_string(&self, contents: &dyn CellFormatter) -> String {
        self.grid.to_string(contents)
    }

    fn size(&self) -> usize {
        self.grid.rows * self.grid.columns
    }
}