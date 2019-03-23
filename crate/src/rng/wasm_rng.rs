
use crate::cells::ICellStrong;
use wbg_rand::{Rng, wasm_rng};
use crate::rng::*;

pub struct WasmRng;
impl RngWrapper for WasmRng {
    type Shuffle = ICellStrong;
    fn gen_range(&self, min: usize, max: usize) -> usize {
        wasm_rng().gen_range(min, max)
    }
    
    fn shuffle(&self, vec: &mut Vec<Self::Shuffle>) {
        wasm_rng().shuffle(vec);
    }
}