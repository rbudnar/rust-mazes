use crate::algorithms::{rand_element, MazeAlgorithm};
use crate::grid::{cell::*, Grid};
use std::rc::{Rc};
use crate::rng::RngWrapper;

#[derive(Debug)]
pub struct HuntAndKill;

/// Hunt and Kill Algorithm
/// 1) Pick a random unvisited cell.
/// 2) Randomly pick and visit (link) a neighbor until you arrive at a cell with no unvisited neighbors
/// 3) From top left corner of maze, scan left to right for the first unvisited cell
/// 4) Link and continue until all cells are visited
impl MazeAlgorithm for HuntAndKill {
    fn on(&self, grid: &dyn Grid, rng_generator: &dyn RngWrapper) {
        let mut current = grid.random_cell(rng_generator);
        while current.is_some() {
            let c = current.clone().unwrap();
            let neighbor_indexes = c.borrow().neighbors_i();
            let unvisited_neighbors: Vec<ICellStrong> = grid.each_cell().iter()
                .filter(|c| {
                    if let Some(c) = c {
                        return neighbor_indexes.contains(&c.borrow().index()) && c.borrow().links().is_empty();
                    }
                    return false;
                })
                .map(|c| Rc::clone(&c.as_ref().unwrap()))
                .collect();
            
            if !unvisited_neighbors.is_empty() {
                let neighbor = rand_element(&unvisited_neighbors, rng_generator);
                c.borrow_mut().link(neighbor.borrow().index());
                neighbor.borrow_mut().link(c.borrow().index());
                current = Some(neighbor.clone());
            }
            else {
                current = None;

                for cell in grid.each_cell().iter() {
                    if let Some(cell) = cell {
                        let neighbors_i = cell.borrow().neighbors_i();
                        let visited_neighbors: Vec<ICellStrong> = grid.each_cell().iter()
                            .filter(|c| {
                                if let Some(c) = c {
                                    return neighbors_i.contains(&c.borrow().index()) && !c.borrow().links().is_empty();
                                }
                                return false;
                            })
                            .map(|c| Rc::clone(&c.as_ref().unwrap()))
                            .collect();

                        if cell.borrow().links().is_empty() && !visited_neighbors.is_empty() {
                            current = Some(cell.clone());

                            let neighbor = rand_element(&visited_neighbors, rng_generator);
                            cell.borrow_mut().link(neighbor.borrow().index());
                            neighbor.borrow_mut().link(cell.borrow().index());
                            break;
                        }
                    }
                }
            }
        }
    }
}