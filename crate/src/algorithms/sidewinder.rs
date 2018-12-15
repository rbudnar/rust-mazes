use rng::RngWrapper;
use grid::*;
use cell::*;
use std::rc::{Rc};
use algorithms::{MazeAlgorithm, rand_element};

#[derive(Debug)]
pub struct Sidewinder;

impl MazeAlgorithm for Sidewinder {
    fn on(&self, grid: &Grid, rng_generator: &RngWrapper) {
        for row in grid.cells.iter() {
            let mut run: Vec<CellLinkStrong> = vec![];
        
            for cell in row.iter() {
                if let Some(cell) = cell {
                    run.push(Rc::clone(&cell));
                    let at_eastern_boundary = cell.borrow().east.is_none();
                    let at_northern_boundary = cell.borrow().north.is_none();
                    let should_close_out = at_eastern_boundary || (!at_northern_boundary && rng_generator.gen_range(0, 2) == 0);

                    if should_close_out {
                        let member = rand_element(&run, rng_generator).clone();

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
}