use std::collections::HashMap;
use cell::*;
use grid::*;
use std::rc::{Rc};
use std::char;

pub struct Distances {
    cells: HashMap<(usize, usize), u32>,
    root: (usize, usize)
}

impl Distances {
    pub fn new(cell: &CellLinkStrong) -> Distances {
        let mut d = Distances {
            cells: HashMap::new(),
            root: (cell.borrow().row, cell.borrow().column)
        };
        d.insert(cell.borrow().row, cell.borrow().column, 0);
        d.build_distances(cell);
        d
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
}

pub struct DistanceGrid {
    distances: Distances
}

impl DistanceGrid {
    pub fn new(root: &CellLinkStrong) -> DistanceGrid {
        DistanceGrid {
            distances: Distances::new(root)
        }
    }

    fn contents_of(&self, cell: &CellLinkStrong) -> String {
        let c = cell.borrow();
        let distance = self.distances.get_distance(c.row, c.column);
        let d = distance.unwrap().clone();
        if distance.is_some() && d > 0 {
            let content = String::from(format!("{}", d));
            return content;
        }

        " ".to_owned()
    }
}

impl CellContents for DistanceGrid {
    fn contents_of(&self, cell: &CellLinkStrong) -> String {
        let c = cell.borrow();
        let distance = self.distances.get_distance(c.row, c.column);
        let d = distance.unwrap().clone();
        if distance.is_some() && d >= 0 {
            let c = char::from_u32(d + 97);
            let content = String::from(format!("{}", c.unwrap()));
            return content;
        }

        " ".to_owned()
    }
}
