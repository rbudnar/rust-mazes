use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use crate::cells::{ICellStrong, ICell};

pub type HexCellStrong = Rc<RefCell<HexCell>>;
pub type HexCellWeak = Weak<RefCell<HexCell>>;

pub struct HexCell {
    self_rc: HexCellWeak,
    pub row: usize,
    pub column: usize,
    pub north: Option<HexCellWeak>,
    pub south: Option<HexCellWeak>,
    pub northeast: Option<HexCellWeak>,
    pub northwest: Option<HexCellWeak>,
    pub southeast: Option<HexCellWeak>,
    pub southwest: Option<HexCellWeak>,
    pub links: Vec<Option<HexCellWeak>>
}

impl PartialEq for HexCell {
    fn eq(&self, other: &HexCell) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl ICell for HexCell {
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

        if let Some(ref northeast) = self.northeast {
            let northeast = northeast.upgrade().unwrap();
            vec.push(northeast as ICellStrong);
        }

        if let Some(ref northwest) = self.northwest {
            let northwest = northwest.upgrade().unwrap();
            vec.push(northwest as ICellStrong);
        }

        if let Some(ref southeast) = self.southeast {
            let northeast = southeast.upgrade().unwrap();
            vec.push(northeast as ICellStrong);
        }

        if let Some(ref southwest) = self.southwest {
            let southwest = southwest.upgrade().unwrap();
            vec.push(southwest as ICellStrong);
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
        if let Some(nl) = other.borrow().as_any().downcast_ref::<HexCell>() {
            let _other: HexCellWeak = Rc::downgrade(&Rc::clone(&nl.self_rc.upgrade().unwrap()));
            self.links.push(Some(_other));
        }
    }
}

impl HexCell {
    pub fn new(row: usize, column: usize) -> HexCellStrong {
        let c = HexCell {
            row, column,
            north: None, 
            south: None, 
            northeast: None, 
            northwest: None, 
            southeast: None, 
            southwest: None,
            links: Vec::new(), 
            self_rc: Weak::new(),
        };

        let rc = Rc::new(RefCell::new(c));
        rc.borrow_mut().self_rc = Rc::downgrade(&rc);

        rc
    }

    pub fn is_linked(&self, other: HexCellStrong) -> bool {
        self.index_of_other(Rc::clone(&other)).is_some()        
    }

    pub fn index_of_other(&self, other: HexCellStrong) -> Option<usize> {
        let other_row: usize = other.borrow().row;
        let other_col: usize = other.borrow().column;
        self.links.iter().position(|ref s| {
            if let Some(st) = s {
                let strong : HexCellStrong = st.upgrade().unwrap();
                let c = strong.borrow();
                c.row == other_row && c.column == other_col
            }
            else {
                false
            }
        })
    }
}