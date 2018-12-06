use std::collections::HashMap;
use cell::*;
use grid::*;
use std::rc::{Rc};
use std::cell::RefCell;
use std::char;

#[derive(Debug)]
pub struct Distances {
    // (row, column)
    cells: HashMap<(usize, usize), u32>,
    root: (usize, usize)
}

impl Distances {
    pub fn new(cell: &CellLinkStrong, build_distances: bool) -> Distances {
        let mut d = Distances {
            cells: HashMap::new(),
            root: (cell.borrow().row, cell.borrow().column)
        };
        d.insert(cell.borrow().row, cell.borrow().column, 0);
        if build_distances {
            d.build_distances(cell);
        }

        d
    }

    fn new_from_root(&self) -> Distances {
        Distances {
            cells: HashMap::new(),
            root: (self.root.0, self.root.1)
        }
    }

    pub fn insert(&mut self, row: usize, column: usize, distance: u32) {
        self.cells.insert((row, column), distance);
    }

    pub fn is_visited(&self, row: usize, column: usize) -> bool {
        self.get_distance(row, column).is_some()
    }

    pub fn get_distance(&self, row: usize, column: usize) -> Option<&u32> {
        self.cells.get(&(row, column))
    }

    // Dijkstra's algorithm. Determines distance to each other cell from a root cell.
    pub fn build_distances(&mut self, root: &CellLinkStrong) {
        let distances = self;

        let mut frontier: Vec<CellLinkStrong> = vec![];
        frontier.push(Rc::clone(&root));

        while !frontier.is_empty() {
            let mut new_frontier: Vec<CellLinkStrong> = vec![];

            for fcell in frontier.iter() {
                let distance = distances.get_distance(fcell.borrow().row, fcell.borrow().column).unwrap().clone();
                for linked in fcell.borrow().links.iter() {
                    let cls = linked.upgrade().unwrap();
                    let c = cls.borrow();

                    if !distances.is_visited(c.row, c.column) {
                        distances.insert(c.row, c.column, distance + 1);
                        new_frontier.push(Rc::clone(&cls));
                    }
                }
            }

            frontier = new_frontier;        
        }
    }

    pub fn path_to(&self, goal: &CellLinkStrong) -> Distances {
        let mut current: CellLinkStrong = Rc::clone(goal);
        let mut breadcrumbs = self.new_from_root();       
        self.insert_dist(&mut breadcrumbs, goal);   

        while !(current.borrow().row == self.root.0 && current.borrow().column == self.root.1) {
            let current_distance = *self.get_distance(current.borrow().row, current.borrow().column).unwrap();
            let mut next_current: CellLinkStrong = Rc::new(RefCell::new(Cell::new(0, 0))); 

            for n in current.borrow().links.iter() {
                let neighbor = n.upgrade().unwrap();
                let n_ref = neighbor.borrow();

                let neighbor_distance = *self.get_distance(n_ref.row, n_ref.column).unwrap();
                if neighbor_distance < current_distance {
                    breadcrumbs.insert(n_ref.row, n_ref.column, neighbor_distance);
                    next_current = Rc::clone(&neighbor);
                    break;
                }
            }

            current = next_current;
        }
        
        breadcrumbs
    }

    fn insert_dist(&self, distances: &mut Distances, cell: &CellLinkStrong) {
        let current_distance = *self.get_distance(cell.borrow().row, cell.borrow().column).unwrap();
        distances.insert(cell.borrow().row, cell.borrow().column, current_distance);
    }

    fn max(&self) -> ((usize, usize) , u32) {
        let mut max_distance = 0;
        let mut max_cell = self.root;

        for (cell, distance) in self.cells.iter() {
            if *distance > max_distance {
                max_cell = *cell;
                max_distance = *distance;
            }
        }

        (max_cell, max_distance)
    }    
}

#[derive(Debug)]
pub struct DistanceGrid {
    distances: Distances,
    path_grid: Distances,
    show_path_only: bool,
}

impl DistanceGrid {
    pub fn new(root: &CellLinkStrong) -> DistanceGrid {
        DistanceGrid {
            distances: Distances::new(root, true),
            path_grid: Distances::new(root, false),
            show_path_only: false,
        }
    }

    pub fn build_path_to(&mut self, cell: &CellLinkStrong) {
        self.path_grid = self.distances.path_to(cell);
    }

    pub fn set_show_path_only(&mut self, show_path_only: bool) {
        self.show_path_only = show_path_only;
    }

    pub fn build_longest_path(&mut self, grid: &Grid) {
        let (max_cell, _) = self.distances.max();

        let new_root = grid.get_cell(max_cell.0, max_cell.1).unwrap();
        let new_distances = Distances::new(&new_root, true);

        let (new_max_cell, _) = new_distances.max();
        
        let goal = grid.get_cell(new_max_cell.0, new_max_cell.1).unwrap();
        self.path_grid = new_distances.path_to(&goal);
    }
}

impl CellContents for DistanceGrid {
    fn contents_of(&self, cell: &CellLinkStrong) -> String {
        let c = cell.borrow();

        let distance = if self.show_path_only {
            self.path_grid.get_distance(c.row, c.column)
        }
        else {
            self.distances.get_distance(c.row, c.column)
        };

        if distance.is_some() {
            let d = distance.unwrap().clone();
            let c = if d > 9 {
                char::from_u32(d - 10 + 97).unwrap()
            }
            else {
                char::from_digit(d, 10).unwrap()
            };

            return String::from(format!("{}", c));
        }

        " ".to_owned()
    }
}
