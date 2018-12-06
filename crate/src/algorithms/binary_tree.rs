use rng::RngWrapper;
use grid::*;
use cell::*;
use std::rc::{Rc};

pub struct BinaryTree;

impl BinaryTree {
    pub fn on(grid: &Grid, rng_generator: &RngWrapper) {
        for (_, cell) in grid.each_cell().iter().enumerate() {
            let mut neighbors: Vec<CellLinkStrong> = vec![];
            if cell.borrow().north.is_some() {
                let north = cell.borrow().north.clone().unwrap().upgrade();
                if north.is_some() {
                    neighbors.push(Rc::clone(&north.unwrap()));
                }
            }
            
            if cell.borrow().east.is_some() {
                let east = cell.borrow().east.clone().unwrap().upgrade();
                if east.is_some() {
                    neighbors.push(Rc::clone(&east.unwrap()));
                }
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