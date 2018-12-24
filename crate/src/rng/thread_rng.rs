use crate::rng::*;
use rand::Rng;

pub struct ThreadRng;

impl RngWrapper for ThreadRng {
    fn gen_range(&self, min: usize, max: usize) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(min, max)
    }
}