use crate::grid::cell::ICellStrong;
use crate::rng::RngWrapper;
use crate::grid::cell::ICell;

pub mod cell;
pub mod grid_web;
// pub mod distances;
pub mod mask;
// pub mod masked_grid;
pub mod standard_grid;
pub mod grid_base;
// pub mod canvas;
// pub mod polar_grid;
// pub mod polar_cell;

pub trait CellFormatter {
    fn contents_of(&self, cell: &dyn ICell) -> String;
    fn background_color(&self, cell: &dyn ICell) -> String;
}

pub trait Grid {
    fn prepare_grid(&mut self);
    fn random_cell(&self, rng: &dyn RngWrapper) -> Option<ICellStrong>;
    fn each_cell(&self) -> Vec<Option<ICellStrong>>;
    fn rows(&self) -> usize;
    fn columns(&self) -> usize;
    fn cells(&self) -> Vec<Vec<Option<ICellStrong>>>;
    fn get_cell(&self, row: usize, column: usize) -> Option<ICellStrong>;
    fn to_string(&self, contents: &dyn CellFormatter) -> String;
    fn size(&self) -> usize;
}