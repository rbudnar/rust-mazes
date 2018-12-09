use rng::RngWrapper;
use grid::Grid;

pub mod binary_tree;
pub mod sidewinder;
pub mod aldous_broder;

pub trait GridAlgorithm {
    fn on(&self,  grid: &Grid, rng_generator: &RngWrapper);
}
