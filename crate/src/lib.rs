// #![feature(use_extern_macros)]
#![feature(const_vec_new)]
#![feature(range_contains)]
#![feature(vec_remove_item)]
#![feature(test)]
#![allow(dead_code)]
#[macro_use]
extern crate cfg_if;
extern crate rand;
extern crate wbg_rand;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;
extern crate math;
extern crate test;

use wasm_bindgen::prelude::*;
mod cell;
mod grid;
mod algorithms;
mod grid_web;
mod distances;
mod rng;
mod mask;
mod masked_grid;
use grid::*;
use distances::*;
use algorithms::{MazeAlgorithm, binary_tree::*, sidewinder::*, aldous_broder::*, wilson::*, hunt_and_kill::*, recursive_backtracker::*};
use rng::{wasm_rng};
use mask::*;


cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// Definitions of the functionality available in JS, which wasm-bindgen will
// generate shims for today (and eventually these should be near-0 cost!)
//
// These definitions need to be hand-written today but the current vision is
// that we'll use WebIDL to generate this `extern` block into a crate which you
// can link and import. There's a tracking issue for this at
// https://github.com/rustwasm/wasm-bindgen/issues/42
//
// In the meantime these are written out by hand and correspond to the names and
// signatures documented on MDN, for example
// #[wasm_bindgen]
// extern "C" {
//     type HTMLDocument;
//     static document: HTMLDocument;
//     #[wasm_bindgen(method)]
//     fn createElement(this: &HTMLDocument, tagName: &str) -> Element;
//     #[wasm_bindgen(method, getter)]
//     fn body(this: &HTMLDocument) -> Element;

//     type Element;
//     #[wasm_bindgen(method, setter = innerHTML)]
//     fn set_inner_html(this: &Element, html: &str);
//     #[wasm_bindgen(method, js_name = appendChild)]
//     fn append_child(this: &Element, other: Element);
//     fn alert(s: &str);
// }
static mut GRID: Grid = Grid {
    cells: Vec::new(),
    rows: 1, columns: 1
};

static mut COLORIZE: bool = true;


/****** ALGORITHMS ******/

#[wasm_bindgen]
pub fn basic_binary_tree(rows: usize, columns: usize) {
    build_and_display_grid(BinaryTree, rows, columns);    
}

#[wasm_bindgen]
pub fn sidewinder(rows: usize, columns: usize) {
    build_and_display_grid(Sidewinder, rows, columns);
}

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
    unsafe {
        let distance_grid = prepare_distance_grid(&GRID);
        grid_web::grid_to_web(&GRID, &distance_grid, COLORIZE);
    }
}

#[wasm_bindgen]
pub fn on_colorize_change(colorize: bool) {
    unsafe {
        COLORIZE = colorize;
        let distance_grid = prepare_distance_grid(&GRID);
        grid_web::grid_to_web(&GRID, &distance_grid, COLORIZE);
    }
}

/****** HELPERS ******/

fn build_and_display_grid(alg: impl MazeAlgorithm, rows: usize, columns: usize) {
    unsafe {        
        GRID = Grid::new(rows, columns, &StandardGridBuilder);
        let wasm_generator = wasm_rng::WasmRng;
        alg.on(&GRID, &wasm_generator);

        let distance_grid = prepare_distance_grid(&GRID);
        grid_web::grid_to_web(&GRID, &distance_grid, COLORIZE);
    }
}

fn prepare_distance_grid(grid: &Grid) -> DistanceGrid {
    let root = grid.cells[grid.rows / 2][grid.columns / 2].clone();
    DistanceGrid::new(&root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use rng::{thread_rng};
    use std::collections::HashMap;

    fn test_std_grid(alg: impl MazeAlgorithm) -> Grid {
        let grid = Grid::new(5,5, &StandardGridBuilder);
        
        let thread_rng = thread_rng::ThreadRng;
        alg.on(&grid, &thread_rng);

        // Prints normal grid without distances.
        let std_grid = StandardGrid;
        println!("{}", grid.to_string(&std_grid));
        grid
    }

    #[test]
    fn binary_tree() {
        let grid = Grid::new(5,5, &StandardGridBuilder);

        let thread_rng = thread_rng::ThreadRng;
        BinaryTree.on(&grid, &thread_rng);

        // This prints the grid with Dijkstra's distances inside, rendered as characters a,b,c, etc. 
        // Will probably need to adjust for really large grids if I really want to display them with distances.
        // grabs first cell of first row
        let root = grid.cells.first().unwrap().first().unwrap();
        let last = grid.cells.last().unwrap().first().unwrap();
        let mut distance_grid = DistanceGrid::new(root);
        
        
        // builds a path to the first cell of the last row
        distance_grid.build_path_to(last);
        println!("{}", grid.to_string(&distance_grid));
        distance_grid.set_show_path_only(true);
        
        // shows the shortest path from root (NW) to SW corner as constructed above
        println!("{}", grid.to_string(&distance_grid));

        // rebuilds path grid to determine and show the longest path
        distance_grid.build_longest_path(&grid);
        println!("{}", grid.to_string(&distance_grid));
    }

    #[test]
    fn sidewinder() {
        test_std_grid(Sidewinder);
    }

    #[test]
    fn aldous_broder() {
        test_std_grid(AldousBroder);
    }

    #[test]
    fn wilson() {
        test_std_grid(Wilson);
    }

    #[test]
    fn recursive_backtracker() {
        test_std_grid(RecursiveBacktracker);
    }
    
    #[test]
    fn kill_cells() {
        let grid = Grid::new(5,5, &StandardGridBuilder);
        let first = grid.cells[0][0].borrow();
        
        let first_e = first.east.clone().unwrap().upgrade().unwrap();
        first_e.borrow_mut().west = None;
        let first_s = first.south.clone().unwrap().upgrade().unwrap();
        first_s.borrow_mut().north = None;

        let last = &grid.cells[4][4].borrow();
        let last_w = last.west.clone().unwrap().upgrade().unwrap();
        last_w.borrow_mut().east = None;
        let last_n = last.north.clone().unwrap().upgrade().unwrap();
        last_n.borrow_mut().south = None;


        let thread_rng = thread_rng::ThreadRng;
        RecursiveBacktracker.on(&grid, &thread_rng);

        // Prints normal grid without distances.
        let std_grid = StandardGrid;
        println!("{}", grid.to_string(&std_grid));
    }

    #[bench]
    fn hunt_and_kill(b: &mut Bencher) {
        let grid = Grid::new(5,5, &StandardGridBuilder);
        
        let thread_rng = thread_rng::ThreadRng;
        b.iter(|| HuntAndKill.on(&grid, &thread_rng));

        let std_grid = StandardGrid;
        println!("{}", grid.to_string(&std_grid));
    }


    #[test]
    #[ignore]
    fn dead_ends() {
        let algorithms: Vec<Box<MazeAlgorithm>> =
            vec![Box::new(BinaryTree), Box::new(Sidewinder), Box::new(AldousBroder), Box::new(Wilson), Box::new(HuntAndKill)];

        let tries = 100;
        let size = 20;

        let thread_rng = thread_rng::ThreadRng;
        let mut averages: HashMap<String, f64> = HashMap::new();

        for alg in algorithms.iter() {
            let mut dead_end_counts: Vec<usize> = vec![];
            println!("Running {:?}", alg);
            
            for _ in 0..tries {
                let mut grid = Grid::new(size,size, &StandardGridBuilder);
                alg.on(&grid, &thread_rng);
                dead_end_counts.push(grid.dead_ends().len())
            }

            let total_deadends = dead_end_counts.iter().fold(0, |acc, x| acc + x);
            averages.insert(format!("{:?}", alg), total_deadends as f64 / dead_end_counts.len() as f64);
        }

        let total_cells = size * size;
        println!("Average dead-ends per {}x{} maze ({} cells):", size, size, total_cells);

        for (alg, avg) in averages.iter() {
            let formatted = format!("{:.*}", 1, (*avg/total_cells as f64) * 100.0);
            println!("{} : {} / {} ({:02}%)", alg, avg, total_cells, formatted);
        }        
    }


    #[test]
    fn colors() {
        let grid = Grid::new(5,5, &StandardGridBuilder);

        let thread_rng = thread_rng::ThreadRng;
        BinaryTree.on(&grid, &thread_rng);

        // This prints the grid with Dijkstra's distances inside, rendered as characters a,b,c, etc. 
        // Will probably need to adjust for really large grids if I really want to display them with distances.
        // grabs first cell of first row
        let root = grid.cells.first().unwrap().first().unwrap();
        let distance_grid = DistanceGrid::new(root);
        let color = distance_grid.background_color(&root);
        assert_eq!(color, "rgb(255,255,255)");

        for row in grid.cells.iter() {
            for cell in row.iter() {
                println!("{}", distance_grid.background_color(&cell))    
            }
        }
    }

    #[test]
    fn mask() {
        let mut mask = Mask::new(5, 5);

        mask.set(0,2, false);
        mask.set(1,2, false);
        mask.set(2,2, false);
        mask.set(3,2, false);
        mask.set(4,2, false);
        println!("{}", mask.get(1,2));
        println!("{:#?}", mask);
        println!("{}", mask.count());
        println!("{:?}", mask.rand_location(&thread_rng::ThreadRng));
    }
}
