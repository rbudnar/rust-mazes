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
                let mut neighbors: Vec<CellLinkStrong> = vec![];
                
                if let Some(north) = cell.borrow().north.clone() {
                    neighbors.push(Rc::clone(&north.upgrade().unwrap()));
                };

                if let Some(east) = cell.borrow().east.clone() {
                    neighbors.push(Rc::clone(&east.upgrade().unwrap()));
                };

                let length =  neighbors.len();
                if length > 0 {
                    let neighbor: CellLinkStrong = rand_element(&neighbors, rng_generator).clone();

                    Cell::link(Rc::clone(cell), neighbor, true);
                }
            }
        }
    }
}