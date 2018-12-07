use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub type CellLinkWeak = Weak<RefCell<Cell>>; // Think of a better name
pub type CellLinkStrong = Rc<RefCell<Cell>>;

#[derive(Debug)]
pub struct Cell {
    pub row: usize,
    pub column: usize,
    pub links: Vec<CellLinkWeak>,
    pub north: Option<CellLinkWeak>,
    pub south: Option<CellLinkWeak>,
    pub east: Option<CellLinkWeak>,
    pub west: Option<CellLinkWeak>
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

    pub fn display_links(&self) {
        for link in &self.links {
            println!("{:?}", link.upgrade());
        }
    }

    pub fn neighbors(&self) -> Vec<CellLinkWeak> {
        let mut vec: Vec<CellLinkWeak> = vec![];
        if self.north.is_some() {
            let c = self.north.as_ref().unwrap().clone();
            vec.push(c);
        }
        if self.south.is_some() {
            let c = self.south.as_ref().unwrap().clone();
            vec.push(c);
        }
        if self.east.is_some() {
            let c = self.east.as_ref().unwrap().clone();
            vec.push(c);
        }
        if self.west.is_some() {
            let c = self.west.as_ref().unwrap().clone();
            vec.push(c);
        }
        vec
    }

    pub fn links(&self) -> &Vec<CellLinkWeak> {
        &self.links
    }

    pub fn is_linked(&self, other: CellLinkStrong) -> bool {
        self.index_of_other(Rc::clone(&other)).is_some()        
    }

    pub fn index_of_other(&self, other: CellLinkStrong) -> Option<usize> {
        let other_row: usize = other.borrow().row;
        let other_col: usize = other.borrow().column;
        self.links.iter().position(|ref s| {
            let strong : CellLinkStrong = s.upgrade().unwrap();
            let c = strong.borrow();
            c.row == other_row && c.column == other_col
        })
    }    
}