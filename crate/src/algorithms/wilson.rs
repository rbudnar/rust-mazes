use crate::cells::ICellStrong;
use crate::grid::{Grid};
use crate::rng::RngWrapper;
use crate::algorithms::{MazeAlgorithm, rand_element};
use std::rc::{Rc};

#[derive(Debug)]
pub struct Wilson;

impl MazeAlgorithm for Wilson {
    fn on(&self, grid: &dyn Grid, rng_generator: &dyn RngWrapper<Shuffle=ICellStrong>) {
        let mut unvisited: Vec<ICellStrong> = vec![];

        for cell in grid.each_cell().iter() {
            if let Some(cell) = cell {
                unvisited.push(cell.clone());
            }
        }

        let first = rng_generator.gen_range(0, unvisited.len());
        unvisited.remove(first);

        while !unvisited.is_empty() {
            let mut path: Vec<ICellStrong> = vec![];

            let mut cell = rand_element(&unvisited, rng_generator).clone();
            path.push(cell.clone());

            while unvisited.contains(&cell) {
                let neighbors = cell.borrow().neighbors();
                cell = rand_element(&neighbors, rng_generator).clone();
                
                if let Some(position) = path.iter().position(|c| c.borrow().row() == cell.borrow().row() && c.borrow().column() == cell.borrow().column()) {
                    path = path[0..=position].to_vec();
                } 
                else {
                    path.push(cell.clone());
                }
            }

            let end = path.len() - 1;
            for i in 0..end {
                path[i].borrow_mut().link(Rc::clone(&path[i + 1]));
                path[i + 1].borrow_mut().link(Rc::clone(&path[i]));
                unvisited.remove_item(&path[i]);
            }
        }
    }
}