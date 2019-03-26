#![feature(const_vec_new)]
#![feature(vec_remove_item)]
#![feature(test)]
#![allow(dead_code)]
#[macro_use]
use std::cell::RefCell;
use crate::rng::wasm_rng;
use wasm_bindgen::prelude::*;

mod algorithms;
mod rng;
mod grid;
mod cells;
mod tests;

use crate::grid::{Grid,
    standard_grid::StandardGrid,
    distances::DistanceGrid, 
    canvas::*,
    polar_grid::*,
    hex_grid::*,
    triangle_grid::*,
    GridType,
    mask_canvas::{clear_mask, append_mask_canvas}
};
use crate::algorithms::{MazeAlgorithm, recursive_backtracker::*, aldous_broder::*, hunt_and_kill::*, wilson::*};

cfg_if::cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if::cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}


thread_local! {
    static GRID: RefCell<Option<Box<dyn Grid>>> = RefCell::new(None);
    pub static COLORIZE: RefCell<bool> = RefCell::new(true);
    pub static GRID_TYPE: RefCell<GridType> = RefCell::new(GridType::StandardGrid);
}


/****** ALGORITHMS ******/
// This is still broken for the polar grid.
// #[wasm_bindgen]
// pub fn basic_binary_tree(rows: usize, columns: usize) {
//     build_and_display_grid(BinaryTree, rows, columns);    
// }

// #[wasm_bindgen]
// pub fn sidewinder(rows: usize, columns: usize) {
//     build_and_display_grid(Sidewinder, rows, columns);
// }

#[wasm_bindgen]
pub fn aldous_broder(rows: usize, columns: usize) {
    build_and_display_grid(AldousBroder, rows, columns);
}

#[wasm_bindgen]
pub fn wilson(rows: usize, columns: usize) {
    build_and_display_grid(Wilson, rows, columns);
}

#[wasm_bindgen]
pub fn hunt_and_kill(rows: usize, columns: usize) {
    build_and_display_grid(HuntAndKill, rows, columns);
}

#[wasm_bindgen]
pub fn recursive_backtracker(rows: usize, columns: usize) {
    build_and_display_grid(RecursiveBacktracker, rows, columns);
}

/****** OTHER FEATURES ******/

#[wasm_bindgen]
pub fn redisplay_grid() {
    GRID.with(|slf| match slf.borrow() {
        grid_ref => {
            let grid = grid_ref.as_ref().unwrap();

            let distance_grid = prepare_distance_grid(&**grid);
            grid.to_web(&distance_grid, get_colorize());
        }
    })
}

#[wasm_bindgen]
pub fn on_colorize_change(colorize: bool) {
    COLORIZE.with(|c| {
        *c.borrow_mut() = colorize;
    });
    redisplay_grid();
}

#[wasm_bindgen]
pub fn on_grid_type_change(grid_type: &str) {
    clear_mask();
    cleanup_canvas(&get_grid_type());

    let grid_type = match grid_type {
        "standard" => GridType::StandardGrid,
        "polar" => GridType::PolarGrid,
        "hex" => GridType::HexGrid,
        "triangle" => GridType::TriangleGrid,
        _ => GridType::PolarGrid
    };

    set_grid_type(grid_type);
}

#[wasm_bindgen]
pub fn add_mask_canvas() {
    append_mask_canvas();
}

/****** HELPERS ******/

fn build_and_display_grid(alg: impl MazeAlgorithm, rows: usize, columns: usize) {
    set_panic_hook();

    GRID.with(|grid| match grid.borrow_mut() {
        mut grid_mut_ref => {
            *grid_mut_ref = match get_grid_type() {
                GridType::PolarGrid => Some(Box::new(PolarGrid::new(rows, columns))),
                GridType::HexGrid => Some(Box::new(HexGrid::new(rows, columns))),
                GridType::TriangleGrid => Some(Box::new(TriangleGrid::new(rows, columns))),
                GridType::StandardGrid => Some(Box::new(StandardGrid::new(rows, columns))),
            };
            render_grid(&**grid_mut_ref.as_ref().unwrap(), alg);
        }
    });
}

pub fn get_colorize() -> bool {
    COLORIZE.with(|c| *c.borrow())
}

pub fn get_grid_type() -> GridType {
    GRID_TYPE.with(|t| t.borrow().clone())
}

pub fn set_grid_type(new_type: GridType) {
    GRID_TYPE.with(|t| {
        *t.borrow_mut() = new_type;
    });
}

fn render_grid(grid: &dyn Grid, alg: impl MazeAlgorithm) {
    let wasm_generator = wasm_rng::WasmRng;
    alg.on(grid, &wasm_generator);
    grid.braid(0.1f64, &wasm_generator);
    let distance_grid = prepare_distance_grid(grid);
    
    grid.to_web(&distance_grid, get_colorize());
}

fn prepare_distance_grid(grid: &dyn Grid) -> DistanceGrid {   
    if let Some(root) = grid.cells()[grid.rows() / 2][grid.columns() / 2].clone() {
        DistanceGrid::new(&root)
    }
    else {
        let root = grid.random_cell(&wasm_rng::WasmRng).unwrap();
        DistanceGrid::new(&root)
    }
}

