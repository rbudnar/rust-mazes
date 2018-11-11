use std::rc::{Rc, Weak};
use std::cell::RefCell;
use serde::ser::{Serialize, Serializer, SerializeStruct};

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

// Not sure if this needed now. Have to manually serialize at the moment due to CellLinkWeak and CellLinkStrong having `Rc`
impl Serialize for Cell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 7 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Cell", 7)?;
        state.serialize_field("row", &self.row)?;
        state.serialize_field("column", &self.column)?;
        // state.serialize_field("links", &self.links)?;

        state.serialize_field("north", &get_coords(&self.north))?;
        state.serialize_field("south", &get_coords(&self.south))?;
        state.serialize_field("east", &get_coords(&self.east))?;
        state.serialize_field("west", &get_coords(&self.west))?;
        state.end()
    }
}

fn get_coords(cell: &Option<CellLinkWeak>) -> String {
    let c: CellLinkStrong = cell.clone().unwrap().upgrade().unwrap().clone();
    let row = c.borrow().row;
    let col = c.borrow().column;
    String::from(format!("row: {}, column: {}", row, col))
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

    // TODO: check this implementation
    pub fn neighbors(&self) -> Vec<CellLinkWeak> {
        let mut vec: Vec<CellLinkWeak> = vec![];
        if self.north.is_some() {
            // let c = self.north.as_ref().unwrap().upgrade().unwrap();
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
