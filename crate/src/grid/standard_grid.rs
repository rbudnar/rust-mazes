use crate::grid::cell::ICellStrong;
use crate::grid::cell::ICell;
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
        // std_grid.grid.configure_cells();
        std_grid.grid.configure_cells_i();
        std_grid
    }
}

impl Grid for StandardGrid {
    fn prepare_grid(&mut self) {
        let mut index = 0;
        for i in 0..self.grid.rows {
            let mut row: Vec<Option<CellLinkStrong>> = Vec::new();
            
            for j in 0..self.grid.columns {
                row.push(Some(Cell::new(i as usize, j as usize, index)));

                index += 1;
            }
            self.grid.cells.push(row);
        }   
    }

    fn random_cell(&self, rng: &dyn RngWrapper) -> Option<ICellStrong> {
        let row: usize = rng.gen_range(0, self.grid.rows);
        let col: usize = rng.gen_range(0, self.grid.columns);
        self.get_cell(row, col)
    }

    fn each_cell(&self) -> Vec<Option<ICellStrong>> {
        self.grid.each_cell().iter()
            .map(|c| Some(Rc::clone(&c.as_ref().unwrap()) as ICellStrong)).collect()
    }

    fn rows(&self) -> usize {
        self.grid.columns
    }

    fn columns(&self) -> usize {
        self.grid.rows
    }

    fn cells(&self) -> Vec<Vec<Option<ICellStrong>>> {
        self.grid.cells.iter().map(|row| 
            row.iter().map(|c| Some(Rc::clone(&c.as_ref().unwrap()) as ICellStrong)).collect()
        ).collect()
    }
    
    fn get_cell(&self, row: usize, column: usize) -> Option<ICellStrong> {
        let cell = self.grid.get_cell(row, column);

        Some(Rc::clone(&cell.unwrap()) as ICellStrong)
    }

    fn get_cell_at_index(&self, index: usize) -> ICellStrong {
        let cells = self.each_cell();
        let c = cells.iter().find(|c| {
            if let Some(c) = c {
                return c.borrow().index() == index
            }
            return false;
        }).unwrap();

        return Rc::clone(&c.as_ref().unwrap())
    }

    fn to_string(&self, contents: &dyn CellFormatter) -> String {
        self.grid.to_string(contents)
    }

    fn size(&self) -> usize {
        self.grid.rows * self.grid.columns
    }
}