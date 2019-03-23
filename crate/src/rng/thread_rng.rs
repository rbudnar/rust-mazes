use crate::cells::ICellStrong;
use crate::rng::*;
use rand::Rng;

pub struct ThreadRng;

impl RngWrapper for ThreadRng {
    type Shuffle = ICellStrong;

    fn gen_range(&self, min: usize, max: usize) -> usize {
        rand::thread_rng().gen_range(min, max)
    }
    fn shuffle(&self, vec: &mut Vec<Self::Shuffle>) {
        rand::thread_rng().shuffle(vec);
    }
}