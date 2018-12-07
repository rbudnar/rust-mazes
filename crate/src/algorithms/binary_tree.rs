use rng::RngWrapper;
use grid::*;
use cell::*;
use std::rc::{Rc};

pub struct BinaryTree;

impl BinaryTree {
    pub fn on(grid: &Grid, rng_generator: &RngWrapper) {
        for (_, cell) in grid.each_cell().iter().enumerate() {
            let mut neighbors: Vec<CellLinkStrong> = vec![];
            
            if let Some(north) = cell.borrow().north.clone() {
                neighbors.push(Rc::clone(&north.upgrade().unwrap()));
            };

            if let Some(east) = cell.borrow().east.clone() {
                neighbors.push(Rc::clone(&east.upgrade().unwrap()));
            }

            let length =  neighbors.len();
            if length > 0 {
                let index = rng_generator.gen_range(0, length);
                let neighbor: CellLinkStrong = Rc::clone(&neighbors[index]);
                link(Rc::clone(cell), neighbor, true);
            }
        }
    }
}