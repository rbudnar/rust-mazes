use crate::grid::Grid;
use std::fmt::Debug;
use crate::rng::RngWrapper;

// pub mod binary_tree;
pub mod sidewinder;
pub mod aldous_broder;
pub mod wilson;
pub mod hunt_and_kill;
pub mod recursive_backtracker;

pub trait MazeAlgorithm: Debug {
    fn on(&self, grid: &dyn Grid, rng_generator: &dyn RngWrapper);
}

fn rand_element<'a, T>(list: &'a [T], rng: &dyn RngWrapper) -> &'a T {
    let index = rng.gen_range(0, list.len());
    &list[index]
}