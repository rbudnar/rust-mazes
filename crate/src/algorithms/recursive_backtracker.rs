use crate::grid::{Grid, cell::Cell, cell::CellLinkStrong};
use crate::algorithms::rand_element;
use crate::rng::RngWrapper;
use crate::algorithms::MazeAlgorithm;

#[derive(Debug)]
pub struct RecursiveBacktracker;


/// 1) Start at a random cell, add to stack
/// 2) Randomly pick an unvisited neighbor, move to it and add to stack
/// 3) When a cell with no unvisited neighbors is reached, pop stack until you get to a cell with unvisited neighbors
/// 4) repeat until stack is empty
impl MazeAlgorithm for RecursiveBacktracker {
    fn on(&self, grid: &dyn Grid, rng_generator: &dyn RngWrapper) {
        let cells = grid.each_cell();
        let mut stack: Vec<CellLinkStrong> = vec![];
        let mut start = rand_element(&cells, rng_generator);
        while start.is_none() {
            start = rand_element(&cells, rng_generator);
        }

        stack.push(start.clone().unwrap());

        while !stack.is_empty() {
            let current = stack.last().unwrap().clone();
            let neighbors = current.borrow().neighbors();
            
            let unvisited: Vec<CellLinkStrong> = neighbors.iter()
                                     .map(|n| n.upgrade().unwrap())
                                     .filter(|n| n.borrow().links().is_empty())
                                     .collect();
            
            if !unvisited.is_empty() {
                let rand_neighbor = rand_element(&unvisited, rng_generator);
                Cell::link(current.clone(), rand_neighbor.clone(), true);
                stack.push(rand_neighbor.clone());
            }
            else {
                stack.pop();
            }
        }         
    }
}