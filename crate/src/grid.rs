use rand::prelude::*;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use cell::*;

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<CellLinkStrong>>,
    pub rows: usize, 
    pub columns: usize
}

impl Grid {
    pub fn new(rows: usize, columns: usize)-> Grid {
        Grid {
            cells: Vec::new(),
            rows, columns            
        }
    }

    pub fn size(&self) -> usize {
        self.rows * self.columns
    }

    pub fn random_cell(&self) -> Option<CellLinkStrong> {
        let mut rng = thread_rng();
        // if rng.gen() {
            let row: usize = rng.gen_range(0, self.rows);
            let col: usize = rng.gen_range(0, self.columns);
            println!("{} {}", row, col);
            return self.get_cell(row, col);
        // }
        // None
    }

    pub fn each_row(&self) {

    }

    // pub fn new_cell(&mut self, row: usize, column: usize) {
    //     let cell = Rc::new(RefCell::new(Cell::new(row, column)));
    //     self.cells.push(cell);
    // }

    pub fn configure_cells(&mut self) {
        for (_, row) in &mut self.cells.iter().enumerate() {
            for (_, cell) in &mut row.iter().enumerate() {            
                // can't subtract from a usize of 0 apparently
                let cell_row = cell.borrow().row;
                if cell_row > 0 {
                    let north = self.get_cell(cell_row - 1, cell.borrow().column);
                    if north.is_some() {
                        cell.borrow_mut().north = Some(Rc::downgrade(&Rc::clone(&north.unwrap())));
                    }
                }

                let south = self.get_cell(cell.borrow().row + 1, cell.borrow().column);
                if south.is_some() {
                    cell.borrow_mut().south = Some(Rc::downgrade(&Rc::clone(&south.unwrap())));
                }

                let east = self.get_cell(cell.borrow().row, cell.borrow().column + 1);
                if east.is_some() {
                    cell.borrow_mut().east = Some(Rc::downgrade(&Rc::clone(&east.unwrap())));
                }

                let cell_column = cell.borrow().column;
                if cell_column > 0 {
                    let west = self.get_cell(cell.borrow().row, cell_column - 1);
                    if west.is_some() {
                        cell.borrow_mut().west = Some(Rc::downgrade(&Rc::clone(&west.unwrap())));
                    }
                }
            }
        }
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Option<CellLinkStrong> {
        if row >= self.rows || column >= self.columns {
            return None;
        }

        return Some(Rc::clone(&self.cells[row][column]));
    }

    pub fn prepare_grid(&mut self) {
        for i in 0..self.rows {
            let mut row: Vec<CellLinkStrong> = Vec::new();
            for j in 0..self.columns {
                row.push(Rc::new(RefCell::new(Cell::new(i as usize, j as usize))));
            }
            self.cells.push(row);
        }   
    }
}

pub fn link(_self: CellLinkStrong, other: CellLinkStrong, bidir: bool) {    
    let newlink: Weak<RefCell<Cell>> = Rc::downgrade(&other);
    _self.borrow_mut().links.push(newlink);
    if bidir {
        link(Rc::clone(&other), Rc::clone(&_self), false);
    }
}


pub fn unlink(_self: CellLinkStrong, other: CellLinkStrong, bidir: bool) {
    let index = _self.borrow().index_of_other(Rc::clone(&other));

    if let Some(i) = index {
        _self.borrow_mut().links.remove(i);
    }

    if bidir {
        unlink(Rc::clone(&other), Rc::clone(&_self), false);
    }
}
