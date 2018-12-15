use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub type CellLinkWeak = Weak<RefCell<Cell>>; // Think of a better name
pub type CellLinkStrong = Rc<RefCell<Cell>>;

#[derive(Debug)]
pub struct Cell {
    pub row: usize,
    pub column: usize,
    pub links: Vec<Option<CellLinkWeak>>,
    pub north: Option<CellLinkWeak>,
    pub south: Option<CellLinkWeak>,
    pub east: Option<CellLinkWeak>,
    pub west: Option<CellLinkWeak>
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl Cell {
    pub fn new(row: usize, column: usize) -> Cell {
        Cell {
            row, column, 
            north: None, 
            south: None, 
            east: None, 
            west: None, 
            links: Vec::new(), 
        }
    }

    pub fn neighbors(&self) -> Vec<CellLinkWeak> {
        let mut vec: Vec<CellLinkWeak> = vec![];

        if let Some(ref north) = self.north {
            vec.push(north.clone());
        }

        if let Some(ref south) = self.south {
            vec.push(south.clone());
        }

        if let Some(ref east) = self.east {
            vec.push(east.clone());
        }

        if let Some(ref west) = self.west {
            vec.push(west.clone());
        }
        vec
    }

    pub fn links(&self) -> &Vec<Option<CellLinkWeak>> {
        &self.links
    }

    pub fn is_linked(&self, other: CellLinkStrong) -> bool {
        self.index_of_other(Rc::clone(&other)).is_some()        
    }

    pub fn index_of_other(&self, other: CellLinkStrong) -> Option<usize> {
        let other_row: usize = other.borrow().row;
        let other_col: usize = other.borrow().column;
        self.links.iter().position(|ref s| {
            if let Some(st) = s {
                let strong : CellLinkStrong = st.upgrade().unwrap();
                let c = strong.borrow();
                c.row == other_row && c.column == other_col
            }
            else {
                false
            }
        })
    }    
}