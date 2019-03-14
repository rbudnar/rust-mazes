use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use crate::cells::{ICellStrong, ICell};

pub type TriangleCellStrong = Rc<RefCell<TriangleCell>>;
pub type TriangleCellWeak = Weak<RefCell<TriangleCell>>;

pub struct TriangleCell {
    self_rc: TriangleCellWeak,
    pub row: usize,
    pub column: usize,
    pub north: Option<TriangleCellWeak>,
    pub south: Option<TriangleCellWeak>,
    pub east: Option<TriangleCellWeak>,
    pub west: Option<TriangleCellWeak>,
    pub links: Vec<Option<TriangleCellWeak>>
}

impl PartialEq for TriangleCell {
    fn eq(&self, other: &TriangleCell) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl ICell for TriangleCell {
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

        if !self.upright() {
            if let Some(ref north) = self.north {
                let north = north.upgrade().unwrap();
                vec.push(north as ICellStrong);
            }
        }

        if self.upright() {
            if  let Some(ref south) = self.south {
                let south = south.upgrade().unwrap();
                vec.push(south as ICellStrong);
            }
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
        if let Some(nl) = other.borrow().as_any().downcast_ref::<TriangleCell>() {
            let _other: TriangleCellWeak = Rc::downgrade(&Rc::clone(&nl.self_rc.upgrade().unwrap()));
            self.links.push(Some(_other));
        }
    }
}

impl TriangleCell {
    pub fn new(row: usize, column: usize) -> TriangleCellStrong {
        let c = TriangleCell {
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

    pub fn is_linked(&self, other: TriangleCellStrong) -> bool {
        self.index_of_other(Rc::clone(&other)).is_some()        
    }

    pub fn index_of_other(&self, other: TriangleCellStrong) -> Option<usize> {
        let other_row: usize = other.borrow().row;
        let other_col: usize = other.borrow().column;
        self.links.iter().position(|ref s| {
            if let Some(st) = s {
                let strong : TriangleCellStrong = st.upgrade().unwrap();
                let c = strong.borrow();
                c.row == other_row && c.column == other_col
            }
            else {
                false
            }
        })
    }

    pub fn upright(&self) -> bool {
        (self.row + self.column) % 2 == 0
    }

    pub fn is_not_linked(&self, other: &Option<TriangleCellWeak>) -> bool {
        if let Some(other) = other.clone() {
            let other = other.upgrade();
            if !self.is_linked(other.unwrap()) {
                return true;
            }
        } else {
            return true;
        }    
        return false;
    }
}