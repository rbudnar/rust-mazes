use grid::link;
use cell::CellLinkStrong;
use rng::RngWrapper;
use algorithms::MazeAlgorithm;
use grid::Grid;
pub struct Wilson;

impl MazeAlgorithm for Wilson {
    fn on(&self, grid: &Grid, rng_generator: &RngWrapper) {
        let mut unvisited: Vec<CellLinkStrong> = vec![];

        for cell in grid.each_cell().iter() {
            unvisited.push(cell.clone());
        }

        let first = rng_generator.gen_range(0, unvisited.len());
        unvisited.remove(first);

        while !unvisited.is_empty() {
            let mut path: Vec<CellLinkStrong> = vec![];
            let index = rng_generator.gen_range(0, unvisited.len());
            let mut cell = unvisited[index].clone();
            path.push(cell.clone());

            while unvisited.contains(&cell) {
                let neighbors = cell.borrow().neighbors();
                let rand_neighbor_index = rng_generator.gen_range(0, neighbors.len());
                cell = neighbors[rand_neighbor_index].upgrade().unwrap().clone();
                
                if let Some(position) = path.iter().position(|&ref c| c.borrow().row == cell.borrow().row && c.borrow().column == cell.borrow().column) {
                    path = path[0..=position].to_vec();
                } 
                else {
                    path.push(cell.clone());
                }
            }
            let end = path.len() - 1;
            for i in 0..end {
                link(path[i].clone(), path[i + 1].clone(), true);
                unvisited.remove_item(&path[i]);
            }
        }
    }
}