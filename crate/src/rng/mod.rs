pub mod wasm_rng;
pub mod thread_rng;

// This allows me to run both `cargo test` and `npm start` without having to switch out RNG implementations
pub trait RngWrapper {
    fn gen_range(&self, min: usize, max: usize) -> usize;
}