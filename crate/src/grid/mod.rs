use crate::rng::RngWrapper;
use crate::cells::{ICellStrong};
use web_sys::{Element, Document};

pub mod distances;
pub mod mask;
pub mod masked_grid;
pub mod standard_grid;
pub mod grid_base;
pub mod canvas;
pub mod polar_grid;
pub mod hex_grid;
pub mod triangle_grid;

pub trait CellFormatter {
    fn contents_of(&self, cell: &ICellStrong) -> String;
    fn background_color(&self, cell: &ICellStrong) -> String;
}

pub trait Grid {
    fn new_cell(&self, row: usize, column: usize) -> ICellStrong;
    fn prepare_grid(&mut self);
    fn random_cell(&self, rng: &dyn RngWrapper) -> Option<ICellStrong>;
    fn each_cell(&self) -> Vec<Option<ICellStrong>>;
    fn rows(&self) -> usize;
    fn columns(&self) -> usize;
    fn cells(&self) -> &Vec<Vec<Option<ICellStrong>>>;
    fn get_cell(&self, row: usize, column: usize) -> Option<ICellStrong>;
    fn to_string(&self, contents: &dyn CellFormatter) -> String;
    fn size(&self) -> usize;
    fn to_web(&self, formatter: &dyn CellFormatter, colorize: bool);
}

pub enum GridType {
    StandardGrid,
    PolarGrid,
    HexGrid,
    TriangleGrid
}