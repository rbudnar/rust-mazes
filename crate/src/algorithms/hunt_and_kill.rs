use algorithms::{rand_element, MazeAlgorithm};
use grid::{cell::*, Grid};
use std::rc::{Rc};
use rng::RngWrapper;

#[derive(Debug)]
pub struct HuntAndKill;

/// Hunt and Kill Algorithm
/// 1) Pick a random unvisited cell.
/// 2) Randomly pick and visit (link) a neighbor until you arrive at a cell with no unvisited neighbors
/// 3) From top left corner of maze, scan left to right for the first unvisited cell
/// 4) Link and continue until all cells are visited
impl MazeAlgorithm for HuntAndKill {
    fn on(&self, grid: &Grid, rng_generator: &RngWrapper) {
        let mut current = grid.random_cell(rng_generator);
        while current.is_some() {
            let c = current.clone().unwrap();
            let unvisited_neighbors: Vec<CellLinkStrong> = c.borrow().neighbors()
                                .iter()
                                .filter(|&c| {
                                    let cref = c.upgrade().unwrap();
                                    let nb = cref.borrow();
                                    nb.links().is_empty() 
                                })
                                .map(|x| Rc::clone(&x.upgrade().unwrap()))
                                .collect();

            if !unvisited_neighbors.is_empty() {
                let neighbor = rand_element(&unvisited_neighbors, rng_generator);
                Cell::link(c, neighbor.clone(), true);
                current = Some(neighbor.clone());
            }
            else {
                current = None;

                for cell in grid.each_cell().iter() {
                    if let Some(cell) = cell {
                        let visited_neighbors: Vec<CellLinkStrong> = cell.borrow().neighbors()
                                    .iter()
                                    .filter(|&c| {
                                        let cref = c.upgrade().unwrap();
                                        let nb = cref.borrow();
                                        !nb.links().is_empty() 
                                    })
                                    .map(|x| Rc::clone(&x.upgrade().unwrap()))
                                    .collect();

                        if cell.borrow().links().is_empty() && !visited_neighbors.is_empty() {
                            current = Some(cell.clone());

                            let neighbor = rand_element(&visited_neighbors, rng_generator);
                            Cell::link(cell.clone(), neighbor.clone(), true);
                            break;
                        }
                    }
                }
            }
        }
    }
}