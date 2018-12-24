
use wbg_rand::{Rng, wasm_rng};
use crate::rng::*;

pub struct WasmRng;
impl RngWrapper for WasmRng {
    fn gen_range(&self, min: usize, max: usize) -> usize {
        wasm_rng().gen_range(min, max)
    }
}