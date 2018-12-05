use grid::*;
use cell::*;
use std::rc::{Rc};
// use wbg_rand::{Rng, wasm_rng};
// use rand::prelude::*;
use rand::Rng;

pub struct Sidewinder;

impl Sidewinder {
    pub fn on(grid: &Grid) {
        let mut rng = rand::thread_rng();
        for (_, row) in grid.cells.iter().enumerate() {
            let mut run: Vec<CellLinkStrong> = vec![];
        
            for (_, cell) in row.iter().enumerate() {
                run.push(Rc::clone(&cell));
                let at_eastern_boundary = !cell.borrow().east.is_some();
                let at_northern_boundary = !cell.borrow().north.is_some();
                let should_close_out = at_eastern_boundary || (!at_northern_boundary && rng.gen_range(0, 2) == 0);
                // let should_close_out = at_eastern_boundary || (!at_northern_boundary && wasm_rng().gen_range(0, 1) == 0);

                if should_close_out {
                    // let index = wasm_rng().gen_range(0, run.len());
                    let index = rng.gen_range(0, run.len());
                    let member = run[index].clone();
                    if member.borrow().north.is_some() {
                        let north = cell.borrow().north.clone().unwrap().upgrade().unwrap();
                        link(Rc::clone(&cell), Rc::clone(&north), true);
                    }
                    run.clear();
                }
                else {
                    let east = cell.borrow().east.clone().unwrap().upgrade().unwrap();
                    link(Rc::clone(&cell), Rc::clone(&east), true);
                }                
            }
        }
    }
}