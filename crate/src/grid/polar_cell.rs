
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
    pub column: usize
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

    fn neighbors(&self) -> Vec<Box<ICell>> {
        let mut vec: Vec<PolarCellLinkWeak> = vec![];

        if let Some(ref cw) = self.cw {
            vec.push(cw.clone());
        }

        if let Some(ref ccw) = self.ccw {
            vec.push(ccw.clone());
        }

        if let Some(ref inward) = self.inward {
            vec.push(inward.clone());
        }

        let o: Vec<PolarCellLinkWeak>  = self.outward.iter()
            .fold(vec![], |mut acc, x| {
                if let Some(x) = x {
                    acc.push(Rc::downgrade(&Rc::clone(&x.upgrade().unwrap())));
                }
                acc
            });

        let v: Vec<Box<ICell>> = vec.iter().chain(o.iter())
            .map(|c| {
                    let cell = *c.upgrade().unwrap().borrow();
                    Box::new(cell) as Box<ICell>
                }).collect();
        v
    }

    fn links(&self) -> &Vec<Option<Box<ICell>>> {
        &self.links.iter().map(|&c| {
            let cell = *c.as_ref().unwrap().upgrade().unwrap().borrow();
            Some(Box::new(cell) as Box<ICell>)
            }).collect()
    }
  
    fn link(&mut self, other: Box<ICell>, bidir: bool) {        
        if let nl = other.as_any().downcast_ref::<PolarCellLinkStrong>() {
            let newlink: PolarCellLinkWeak = Rc::downgrade(&nl.unwrap());
            self.links.push(Some(newlink));

            if bidir {
                self.link(other, false);
            }
        }
    }
}

impl PolarCell {
    pub fn new(row: usize, column: usize) -> PolarCell {
        PolarCell {
            cw: None,
            ccw: None,
            inward: None,
            outward: vec![],
            links: vec![],
            row,
            column
        }
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

    // pub fn link(_self: PolarCellLinkStrong, other: PolarCellLinkStrong, bidir: bool) {    
    //     let newlink: PolarCellLinkWeak = Rc::downgrade(&other);
    //     _self.borrow_mut().links.push(Some(newlink));
    //     if bidir {
    //         PolarCell::link(Rc::clone(&other), Rc::clone(&_self), false);
    //     }
    // }

    // pub fn neighbors(&self) -> Vec<PolarCellLinkWeak> {
    //     let mut vec: Vec<PolarCellLinkWeak> = vec![];

    //     if let Some(ref cw) = self.cw {
    //         vec.push(cw.clone());
    //     }

    //     if let Some(ref ccw) = self.ccw {
    //         vec.push(ccw.clone());
    //     }

    //     if let Some(ref inward) = self.inward {
    //         vec.push(inward.clone());
    //     }

    //     let o: Vec<PolarCellLinkWeak>  = self.outward.iter()
    //         .fold(vec![], |mut acc, x| {
    //             if let Some(x) = x {
    //                 acc.push(Rc::downgrade(&Rc::clone(&x.upgrade().unwrap())));
    //             }
    //             acc
    //         });

    //     let v: Vec<PolarCellLinkWeak> = vec.iter().chain(o.iter())
    //         .map(|x| {
    //                 Rc::downgrade(&Rc::clone(&x.upgrade().unwrap()))
    //             }).collect();
    //     v
    // }
}