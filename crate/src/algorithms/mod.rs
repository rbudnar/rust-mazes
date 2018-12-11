use rng::RngWrapper;
use grid::Grid;

pub mod binary_tree;
pub mod sidewinder;
pub mod aldous_broder;
pub mod wilson;
pub mod hunt_and_kill;

pub trait MazeAlgorithm {
    fn on(&self,  grid: &Grid, rng_generator: &RngWrapper);
}


fn rand_element<'a, T>(list: &'a [T], rng: &RngWrapper) -> &'a T {
    let index = rng.gen_range(0, list.len());
    &list[index]
}