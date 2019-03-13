pub mod cell;
pub mod polar_cell;
pub mod hex_cell;
use std::any::Any;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::{Error, Formatter, Debug};

pub type ICellStrong = Rc<RefCell<ICell>>;
pub type ICellWeak = Weak<RefCell<ICell>>;

pub trait ICell {
    fn neighbors(&self) -> Vec<ICellStrong>;
    fn links(&self) -> Vec<Option<ICellStrong>>;
    fn link(&mut self, other: ICellStrong);
    fn as_any(&self) -> &dyn Any;
    fn row(&self) -> usize;
    fn column(&self) -> usize;
}

impl Debug for ICell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "")
    }
}

impl PartialEq for ICell {
    fn eq(&self, rhs: &ICell) -> bool {
        self.row() == rhs.row() && self.column() == rhs.column()
    }
}