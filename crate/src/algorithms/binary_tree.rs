use crate::rng::RngWrapper;
use crate::grid::{Grid};
use crate::cells::*;
use std::rc::{Rc};
use crate::algorithms::{MazeAlgorithm, rand_element};

#[derive(Debug)]
pub struct BinaryTree;

impl MazeAlgorithm for BinaryTree {
    fn on(&self, grid: &dyn Grid, rng_generator: &dyn RngWrapper) {
        for cell in grid.each_cell().iter() {
            if let Some(cell) = cell {
                let mut neighbors: Vec<ICellStrong> = vec![];
                if cell.borrow().row() > 0 {                    
                    if let Some(north) = grid.get_cell(cell.borrow().row() - 1, cell.borrow().column()) {
                        neighbors.push(Rc::clone(&north));
                    };
                }

                if cell.borrow().column() < grid.columns() - 1 {
                    if let Some(east) = grid.get_cell(cell.borrow().row(), cell.borrow().column() + 1) {
                        neighbors.push(Rc::clone(&east));
                    };
                }

                let length =  neighbors.len();
                if length > 0 {
                    let neighbor: ICellStrong = rand_element(&neighbors, rng_generator).clone();
                    cell.borrow_mut().link(Rc::clone(&neighbor));
                    neighbor.borrow_mut().link(Rc::clone(cell));
                }
            }
        }
    }
}