use grid::*;
use cell::*;
use std::rc::{Rc};
use wbg_rand::{Rng, wasm_rng};
// use rand::prelude::*;
// use rand::Rng;

pub struct BinaryTree;

impl BinaryTree {
    pub fn on(grid: &Grid) {
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

            // let mut rng = rand::thread_rng();
            let length =  neighbors.len();
            if length > 0 {
                // let index = rng.gen_range(0, length);
                // TODO: Fix cannot call on non-wasm_bindgen targets
                let index = wasm_rng().gen_range(0, length);
                // let index = length -1;
                let neighbor: CellLinkStrong = Rc::clone(&neighbors[index]);
                link(Rc::clone(cell), neighbor, true);
            }
            else {
                // println!("{:#?}", cell);
            }            
        }
    }
}