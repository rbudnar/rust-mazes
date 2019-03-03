use crate::algorithms::MazeAlgorithm;
use crate::rng::RngWrapper;
use crate::grid::{Grid};
// use wasm_bindgen::prelude::JsValue;

#[derive(Debug)]
pub struct AldousBroder;

/// Aldous Broder Algorithm
/// #1 start by pick a random cell in the grid
/// #2 from that cell, randomly choose a neighbor. 
/// #3a if that neighbor has not been visited, link and set the neighbor as the current cell
/// #3b if that neighbor HAS been visited, set the neighbor as the current cell, but do not link.
/// Repeat until every cell has been visited.
impl MazeAlgorithm for AldousBroder {
    fn on(&self, grid: &dyn Grid, rng_generator: &dyn RngWrapper) {

        let mut cell = grid.random_cell(rng_generator).unwrap();
       
        // this only works becuase the maze is "perfect"
        let mut unvisited_cells = grid.size() - 1;
        while unvisited_cells > 0 {
            // web_sys::console::log_1(&JsValue::from_str(&format!("{}", unvisited_cells)));
            let neighbors = cell.borrow().neighbors_i();
            let rand_neighbor = rng_generator.gen_range(0, neighbors.len());

            if let Some(next_neighbor) = grid.get_cell_at_index(neighbors[rand_neighbor]) {
                let is_empty = next_neighbor.borrow().links().is_empty();

                if is_empty {
                    cell.borrow_mut().link(next_neighbor.borrow().index());
                    next_neighbor.borrow_mut().link(cell.borrow().index());
                    unvisited_cells -= 1;
                }
                cell = next_neighbor.clone();
            }
        }
    }
}