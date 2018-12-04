// #![feature(use_extern_macros)]
#![allow(dead_code)]
#[macro_use]
extern crate cfg_if;
extern crate rand;
extern crate wbg_rand;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;
mod cell;
mod grid;
mod algorithms;
mod grid_web;
mod distances;
use grid::*;
use algorithms::{binary_tree::*, sidewinder::*};

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


#[wasm_bindgen]
pub fn binary_tree() -> String {
    let mut grid = Grid::new(4,4);
    grid.prepare_grid();
    grid.configure_cells();

    BinaryTree::on(&grid);
    return serde_json::to_string(&grid).unwrap();
}

#[wasm_bindgen]
pub fn basic_binary_tree(rows: usize, columns: usize) {
    let grid = build_grid(rows, columns);

    BinaryTree::on(&grid);
    grid_web::grid_to_web(&grid);
}

#[wasm_bindgen]
pub fn sidewinder(rows: usize, columns: usize) {
    let grid = build_grid(rows, columns);

    Sidewinder::on(&grid);
    grid_web::grid_to_web(&grid);
}

fn build_grid(rows: usize, columns: usize) -> Grid {
    let mut grid = Grid::new(rows, columns);
    grid.prepare_grid();
    grid.configure_cells();
    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use distances::*;

    #[test]
    fn cell_works() {
        // let mut grid = Grid::new(2,2);
        // // grid.new_cell(0,0);
        // // grid.new_cell(0,1);
        // // grid.new_cell(1,0);
        // // grid.new_cell(1,1);

        // let mut c00 = grid.get_cell(0,0).unwrap();
        // let mut c01 = grid.get_cell(0,1).unwrap();

        // link(Rc::clone(&c00), Rc::clone(&c01), true);
        // // println!("c00: {:?}", c00.borrow().display_links());
        // // println!("c01: {:?}", c01.borrow().display_links());
        // // println!("c00-c01 islinked {}", c00.borrow().is_linked(Rc::clone(&c01)));
        // // println!("c01-c00 islinked {}", c01.borrow().is_linked(Rc::clone(&c00)));

        // // println!("UNLINKING");
        // unlink(Rc::clone(&c00), Rc::clone(&c01), true);
        // println!("c00: {:?}", c00.borrow().display_links());
        // println!("c01: {:?}", c01.borrow().display_links());
        // println!("c00-c01 islinked {}", c00.borrow().is_linked(Rc::clone(&c01)));
        // println!("c01-c00 islinked {}", c01.borrow().is_linked(Rc::clone(&c00)));

        // println!("neighbors: {:?}", c00.borrow().neighbors());
    }

    // #[test]
    // fn grid_works() {
    //     let mut grid = Grid::new(2,2);
    //     grid.prepare_grid();
    //     grid.configure_cells();

    //     println!("{:#?}", grid.random_cell());
    //     let cells = grid.each_cell();

    //     for (i, cell) in cells.iter().enumerate() {
    //         println!("{}: {:#?}", i, cell);
    //     }
    //     println!("{:#?}", grid);
    // }

    #[test]
    fn binary_tree() {
        let mut grid = Grid::new(5,5);
        grid.prepare_grid();
        grid.configure_cells();

        BinaryTree::on(&grid);

        // This prints the grid with Dijkstra's distances inside, rendered as characters a,b,c, etc. 
        // Will probably need to adjust for really large grids if I really want to display them with distances.
        let root = grid.cells.first().unwrap().first().unwrap();
        let distanceGrid = DistanceGrid::new(root);

        println!("{}", grid.to_string(&distanceGrid));
    }

    #[test]
    fn sidewinder() {
        let mut grid = Grid::new(5,5);
        grid.prepare_grid();
        grid.configure_cells();

        Sidewinder::on(&grid);
        // let root = grid.cells.first().unwrap().first().unwrap();
        // let distanceGrid = DistanceGrid::new(root);
        // println!("{}", grid.to_string(&distanceGrid));

        // Prints normal grid without distances.
        let stdGrid = StandardGrid;
        println!("{}", grid.to_string(&stdGrid));
    }
}