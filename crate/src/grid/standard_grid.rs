use crate::cells::{ICellStrong, cell::{Cell, CellLinkStrong}};
use crate::grid::{grid_base::GridBase, Grid, CellFormatter};
use crate::rng::RngWrapper;
use std::rc::{Rc};
use web_sys::{Element, Document};

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
    fn new_cell(&self, row: usize, column: usize) -> ICellStrong {
        Cell::new(row, column)
    }

    fn prepare_grid(&mut self) {
        for i in 0..self.grid.rows {
            let mut row: Vec<Option<CellLinkStrong>> = Vec::new();
            
            for j in 0..self.grid.columns {
                row.push(Some(Cell::new(i as usize, j as usize)));

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

    fn cells(&self) -> &Vec<Vec<Option<ICellStrong>>> {
        self.grid.cells()
    }
    
    fn get_cell(&self, row: usize, column: usize) -> Option<ICellStrong> {
        let cell = self.grid.get_cell(row, column);

        Some(Rc::clone(&cell.unwrap()) as ICellStrong)
    }

    fn to_string(&self, contents: &dyn CellFormatter) -> String {
        self.grid.to_string(contents)
    }

    fn size(&self) -> usize {
        self.grid.rows * self.grid.columns
    }

    fn to_web(&self, document: &Document, grid_container: &Element, formatter: &dyn CellFormatter, colorize: bool) {
        self.grid.to_web(document, grid_container, formatter, colorize);
    }
}