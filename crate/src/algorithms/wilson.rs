use crate::grid::cell::ICellStrong;
use crate::grid::{cell::CellLinkStrong, Grid, cell::Cell};
use crate::rng::RngWrapper;
use crate::algorithms::{MazeAlgorithm, rand_element};

#[derive(Debug)]
pub struct Wilson;

impl MazeAlgorithm for Wilson {
    fn on(&self, grid: &dyn Grid, rng_generator: &dyn RngWrapper) {
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

            let cell = rand_element(&unvisited, rng_generator).clone();
            path.push(cell.clone());

            while unvisited.contains(&cell) {
                let neighbors = cell.borrow().neighbors_i();
                // cell = rand_element(&neighbors, rng_generator).upgrade().unwrap().clone();
                let cell_index = rand_element(&neighbors, rng_generator);
                if let Some(cell) = grid.get_cell_at_index(*cell_index) {
                    if let Some(position) = path.iter().position(|c| c.borrow().row() == cell.borrow().row() && c.borrow().column() == cell.borrow().column()) {
                        path = path[0..=position].to_vec();
                    } 
                    else {
                        path.push(cell.clone());
                    }
                }
            }

            let end = path.len() - 1;
            for i in 0..end {
                path[i].borrow_mut().link(path[i + 1].borrow().index());
                path[i+1].borrow_mut().link(path[i].borrow().index());
                unvisited.remove_item(&path[i]);
            }
        }
    }
}