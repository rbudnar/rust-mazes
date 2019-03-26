use crate::cells::{ICellStrong, ICell};
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::any::Any;

pub type CellLinkWeak = Weak<RefCell<Cell>>;
pub type CellLinkStrong = Rc<RefCell<Cell>>;

#[derive(Debug)]
pub struct Cell {
    pub row: usize,
    pub column: usize,
    pub links: Vec<Option<CellLinkWeak>>,
    pub north: Option<CellLinkWeak>,
    pub south: Option<CellLinkWeak>,
    pub east: Option<CellLinkWeak>,
    pub west: Option<CellLinkWeak>,
    pub self_rc: CellLinkWeak,
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
    
    fn neighbors(&self) -> Vec<ICellStrong> {
        let mut vec: Vec<ICellStrong> = vec![];

        if let Some(ref north) = self.north {
            let north = north.upgrade().unwrap();
            vec.push(north as ICellStrong);
        }

        if let Some(ref south) = self.south {
            let south = south.upgrade().unwrap();
            vec.push(south as ICellStrong);
        }

        if let Some(ref east) = self.east {
            let east = east.upgrade().unwrap();
            vec.push(east as ICellStrong);
        }

        if let Some(ref west) = self.west {
            let west = west.upgrade().unwrap();
            vec.push(west as ICellStrong);
        }

        vec
    }

    fn links(&self) -> Vec<Option<ICellStrong>> {
        self.links.iter()
            .map(|c| 
                Some(c.as_ref().unwrap().upgrade().unwrap() as ICellStrong)
            ).collect()
    }

    fn link(&mut self, other: ICellStrong) {       
        if let Some(nl) = other.borrow().as_any().downcast_ref::<Cell>() {
            let _other: CellLinkWeak = Rc::downgrade(&Rc::clone(&nl.self_rc.upgrade().unwrap()));
            self.links.push(Some(_other));
        }
    }
}

impl Cell {
    pub fn new(row: usize, column: usize) -> CellLinkStrong {
        let c = Cell {
            row, column, 
            north: None, 
            south: None, 
            east: None, 
            west: None, 
            links: Vec::new(), 
            self_rc: Weak::new(),
        };

        let rc = Rc::new(RefCell::new(c));
        rc.borrow_mut().self_rc = Rc::downgrade(&rc);

        rc
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

    pub fn is_not_linked(&self, other: &Option<CellLinkWeak>) -> bool {
        if let Some(other) = other.clone() {
            let other = other.upgrade();
            if !self.is_linked(other.unwrap()) {
                return true;
            }
        } else {
            return true;
        }    
        false
    }

    pub fn neighbors_std(&self) -> Vec<CellLinkStrong> {
        let mut vec: Vec<CellLinkStrong> = vec![];

        if let Some(ref north) = self.north {
            let north = north.upgrade().unwrap();
            vec.push(north);
        }

        if let Some(ref south) = self.south {
            let south = south.upgrade().unwrap();
            vec.push(south);
        }

        if let Some(ref east) = self.east {
            let east = east.upgrade().unwrap();
            vec.push(east);
        }

        if let Some(ref west) = self.west {
            let west = west.upgrade().unwrap();
            vec.push(west);
        }

        vec
    }

    fn link(&mut self, other: CellLinkStrong) {       
        let nl = other.borrow();
        let _other: CellLinkWeak = Rc::downgrade(&Rc::clone(&nl.self_rc.upgrade().unwrap()));
        self.links.push(Some(_other));        
    }

    // pub fn unlink(_self: CellLinkStrong, other: CellLinkStrong, bidir: bool) {
    //     let index = _self.borrow().index_of_other(Rc::clone(&other));

    //     if let Some(i) = index {
    //         _self.borrow_mut().links.remove(i);
    //     }

    //     if bidir {
    //         Cell::unlink(Rc::clone(&other), Rc::clone(&_self), false);
    //     }
    // }    
}