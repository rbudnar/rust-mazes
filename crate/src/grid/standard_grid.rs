use std::collections::HashMap;
use web_sys::{Element, Document};
use crate::grid::cell::ICellStrong;
use crate::grid::{grid_base::GridBase, Grid, CellFormatter, cell::{CellLinkStrong, Cell}};
use crate::rng::RngWrapper;

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
        std_grid.grid.configure_cells_i();
        std_grid
    }
}

impl Grid for StandardGrid {
    fn new_cell(&self, row: usize, column: usize, index: usize) -> ICellStrong {
        Cell::new(row, column, index)
    }

    fn prepare_grid(&mut self) {
        let mut index = 0;
        for i in 0..self.grid.rows {
            let mut row: Vec<Option<CellLinkStrong>> = Vec::new();
            let mut row_h: HashMap<usize, Option<CellLinkStrong>> = HashMap::new();
            
            for j in 0..self.grid.columns {
                row_h.insert(j, Some(Cell::new(i as usize, j as usize, index)));
                row.push(Some(Cell::new(i as usize, j as usize, index)));

                index += 1;
            }
            self.grid.cells.push(row);
            self.grid.cells_h.insert(i, row_h);
        }


    }

    fn random_cell(&self, rng: &dyn RngWrapper) -> Option<ICellStrong> {
        self.grid.random_cell(rng)
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

    fn cells(&self) -> Vec<Vec<Option<ICellStrong>>> {
        self.grid.cells()
    }
    
    fn get_cell(&self, row: usize, column: usize) -> Option<ICellStrong> {
        self.grid.get_cell(row, column)
    }

    fn get_cell_links(&self, index: usize) -> Vec<ICellStrong> {
        return self.grid.get_cell_links(index);
    }

    // fn get_neighbor_links(&self, index: usize) -> Vec<ICellStrong> { 

    // }

    fn get_cell_at_index(&self, index: usize) -> ICellStrong {
        return self.grid.get_cell_at_index(index);
  
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