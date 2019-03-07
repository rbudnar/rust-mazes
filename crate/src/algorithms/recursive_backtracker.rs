use crate::grid::cell::{ICellStrong};
use std::rc::Rc;
use crate::grid::{Grid};
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
        let mut stack: Vec<ICellStrong> = vec![];
        let mut start = rand_element(&cells, rng_generator);
        while start.is_none() {
            start = rand_element(&cells, rng_generator);
        }

        stack.push(start.clone().unwrap());

        while !stack.is_empty() {
            let current = stack.last().unwrap().clone();
            let neighbors = current.borrow().neighbors();
            
            let unvisited: Vec<ICellStrong> = neighbors.iter()
                                    .filter(|n| n.borrow().links().is_empty())
                                    .map(|n| Rc::clone(&n))
                                    .collect();

            if !unvisited.is_empty() {
                let rand_neighbor = rand_element(&unvisited, rng_generator);
                current.borrow_mut().link(Rc::clone(rand_neighbor));
                rand_neighbor.borrow_mut().link(Rc::clone(&current));
                stack.push(Rc::clone(&rand_neighbor));
            }
            else {
                stack.pop();
            }
        }         
    }
}