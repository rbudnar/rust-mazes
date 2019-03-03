use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::any::Any;
pub type ICellStrong = Rc<RefCell<ICell>>;
pub type ICellWeak = Weak<RefCell<ICell>>;


pub trait ICell {
    fn neighbors_i(&self) -> Vec<usize>;
    fn link(&mut self, other: usize);
    fn links(&self) -> &Vec<usize>;
    fn as_any(&self) -> &dyn Any;
    fn row(&self) -> usize;
    fn column(&self) -> usize;
    fn index(&self) -> usize;
}

impl Debug for ICell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "")
    }
}

impl PartialEq for ICell {
    fn eq(&self, rhs: &ICell) -> bool {
        self.index() == rhs.index()
    }
}

pub type CellLinkWeak = Weak<RefCell<Cell>>; // Think of a better name
pub type CellLinkStrong = Rc<RefCell<Cell>>;

#[derive(Debug)]
pub struct Cell {
    pub row: usize,
    pub column: usize,
    pub index: usize,
    pub links: Vec<usize>,
    pub north_i: Option<usize>,
    pub south_i: Option<usize>,
    pub east_i: Option<usize>,
    pub west_i: Option<usize>
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl ICell for Cell {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn row(&self) -> usize {
        self.row
    }
    fn column(&self) -> usize {
        self.column
    }
    fn index(&self) -> usize {
        self.index
    }

    fn link(&mut self, other: usize) {
        self.links.push(other);
    }

    fn links(&self) -> &Vec<usize> {
        &self.links
    }

    fn neighbors_i(&self) -> Vec<usize>{
        let mut vec: Vec<usize> = vec![];
        
        if let Some(north) = self.north_i {
            vec.push(north);
        }

        if let Some(south) = self.south_i {
            vec.push(south);
        }

        if let Some(east) = self.east_i {
            vec.push(east);
        }

        if let Some(west) = self.west_i {
            vec.push(west);
        }

        vec
    }
}

impl Cell {
    pub fn new(row: usize, column: usize, index: usize) -> CellLinkStrong {
        let c = Cell {
            row, column, 
            links: Vec::new(),
            north_i: None,
            south_i: None,
            east_i: None,
            west_i: None,
            index
        };

        Rc::new(RefCell::new(c))
    }

    pub fn is_linked_i(&self, other: usize) -> bool {
        self.index_of_other_i(other).is_some()        
    }

    pub fn index_of_other_i(&self, other: usize) -> Option<&usize> {
        self.links.iter().find(|&&c| c == other)
    }   
}