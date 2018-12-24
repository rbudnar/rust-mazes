use crate::rng::RngWrapper;
use crate::grid::cell::CellLinkStrong;

pub mod cell;
pub mod grid_web;
pub mod distances;
pub mod mask;
pub mod masked_grid;
pub mod standard_grid;
pub mod grid_base;
pub mod canvas;

pub trait CellFormatter {
    fn contents_of(&self, cell: &CellLinkStrong) -> String;
    fn background_color(&self, cell: &CellLinkStrong) -> String;
}

pub trait Grid {
    fn prepare_grid(&mut self);
    fn random_cell(&self, rng: &dyn RngWrapper) -> Option<CellLinkStrong>;
    fn each_cell(&self) -> Vec<Option<CellLinkStrong>>;
    fn rows(&self) -> usize;
    fn columns(&self) -> usize;
    fn cells(&self) -> &Vec<Vec<Option<CellLinkStrong>>>;
    fn get_cell(&self, row: usize, column: usize) -> Option<CellLinkStrong>;
    fn to_string(&self, contents: &dyn CellFormatter) -> String;
    fn size(&self) -> usize;
}