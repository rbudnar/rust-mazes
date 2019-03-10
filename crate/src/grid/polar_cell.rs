
use crate::grid::cell::ICellStrong;
use std::any::Any;
use crate::grid::cell::ICell;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub type PolarCellLinkWeak = Weak<RefCell<PolarCell>>;
pub type PolarCellLinkStrong = Rc<RefCell<PolarCell>>;

pub struct PolarCell {
    pub cw: Option<PolarCellLinkWeak>,
    pub ccw: Option<PolarCellLinkWeak>,
    pub inward: Option<PolarCellLinkWeak>,
    pub outward: Vec<Option<PolarCellLinkWeak>>,
    pub links: Vec<Option<PolarCellLinkWeak>>,
    pub row: usize,
    pub column: usize,
    self_rc: PolarCellLinkWeak
}

impl PartialEq for PolarCell {
    fn eq(&self, other: &PolarCell) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl ICell for PolarCell {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn neighbors(&self) -> Vec<ICellStrong> {
        let mut vec: Vec<ICellStrong> = vec![];

        if let Some(ref cw) = self.cw {
            let cw = cw.upgrade().unwrap();
            vec.push(cw as ICellStrong);
        }

        if let Some(ref ccw) = self.ccw {
            let ccw = ccw.upgrade().unwrap();
            vec.push(ccw as ICellStrong);
        }

        if let Some(ref inward) = self.inward {
            let inward = inward.upgrade().unwrap();
            vec.push(inward as ICellStrong);
        }

        for out in self.outward.iter() {
            if let Some(out) = out {
                let out = out.upgrade().unwrap();
                vec.push(out as ICellStrong);
            }
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
        if let Some(nl) = other.borrow().as_any().downcast_ref::<PolarCell>() {
            let _other: PolarCellLinkWeak = Rc::downgrade(&Rc::clone(&nl.self_rc.upgrade().unwrap()));
            self.links.push(Some(_other));
        }
    }

    fn row(&self) -> usize {
        self.row
    }

    fn column(&self) -> usize {
        self.column
    }
}

impl PolarCell {
    pub fn new(row: usize, column: usize) -> PolarCellLinkStrong {
        let cell = PolarCell {
            cw: None,
            ccw: None,
            inward: None,
            outward: vec![],
            links: vec![],
            row,
            column,
            self_rc: Weak::new(),
        };

        let rc = Rc::new(RefCell::new(cell));
        rc.borrow_mut().self_rc = Rc::downgrade(&rc);

        rc
    }


    pub fn is_linked(&self, other: PolarCellLinkStrong) -> bool {
        self.index_of_other(Rc::clone(&other)).is_some()        
    }

    pub fn index_of_other(&self, other: PolarCellLinkStrong) -> Option<usize> {
        let other_row: usize = other.borrow().row;
        let other_col: usize = other.borrow().column;
        self.links.iter().position(|ref s| {
            if let Some(st) = s {
                let strong : PolarCellLinkStrong = st.upgrade().unwrap();
                let c = strong.borrow();
                c.row == other_row && c.column == other_col
            }
            else {
                false
            }
        })
    }
}