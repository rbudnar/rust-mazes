#![feature(use_extern_macros)]

#[macro_use]
extern crate cfg_if;

extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;
use wasm_bindgen::prelude::*;
use web_sys::Window;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlElement;
use web_sys::Node;
use std::convert::From;
use std::collections::HashSet;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
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
pub fn greet() {
    let window = web_sys::window().unwrap();
    window.alert_with_message(&format!("Woot 123!"));
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() {
    let document = web_sys::window().unwrap().document().unwrap();
    let val = document.create_element("p").unwrap();
    val.set_inner_html("Hello from Rust, WebAssembly, and Webpack 123!");
    let body = Node::from(document.body().unwrap());
    body.append_child(&Node::from(val)).unwrap();
    // document.import_node(val);
}





// impl PartialEq for Cell {
//     fn eq(&self, other: &Cell) -> bool {
//         self.row == other.row && self.column == other.column
//     }
// }
// impl Eq for Cell {}

// impl Hash for Cell {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.row.hash(state);
//         self.column.hash(state);
//     }
// }
type CellLinkWeak = Weak<RefCell<Cell>>; // Think of a better name
type CellLinkStrong = Rc<RefCell<Cell>>;

#[derive(Debug)]
pub struct Grid {
    cells: Vec<CellLinkStrong>,
    rows: u32, 
    columns: u32
}

impl Grid {
    fn new(rows: u32, columns: u32)-> Grid {
        Grid {
            cells: Vec::new(),
            rows, columns            
        }
    }

    fn new_cell(&mut self, row: u32, column: u32) {
        let cell = Rc::new(RefCell::new(Cell::new(row, column)));
        self.cells.push(cell);
    }

    fn link_cells(&mut self) {
        for (i, cell) in &mut self.cells.iter().enumerate() {
            if (i > 0) {
                println!("linking");
            }
        }
    }

    fn get_cell(&self, row: u32, column: u32) -> Option<CellLinkStrong> {
        let mut iter = self.cells.iter();
        iter.find(|ref x| x.borrow().row == row && x.borrow().column == column)
            .map(|ref x| Rc::clone(&x))
    }
}

fn link(_self: CellLinkStrong, other: CellLinkStrong, bidir: bool) {    
    let newlink: Weak<RefCell<Cell>> = Rc::downgrade(&other);
    _self.borrow_mut().links.push(newlink);
    if bidir {
        link(Rc::clone(&other), Rc::clone(&_self), false);
    }
}


fn unlink(_self: CellLinkStrong, other: CellLinkStrong, bidir: bool) {
    let index = _self.borrow().index_of_other(Rc::clone(&other));

    if let Some(i) = index {
        _self.borrow_mut().links.remove(i);
    }

    if bidir {
        unlink(Rc::clone(&other), Rc::clone(&_self), false);
    }
}


#[derive(Debug)]
struct Cell {
    row: u32,
    column: u32,
    links: Vec<CellLinkWeak>,
    north: Option<CellLinkWeak>,
    south: Option<CellLinkWeak>,
    east: Option<CellLinkWeak>,
    west: Option<CellLinkWeak>
}

impl Cell {
    fn new(row: u32, column: u32) -> Cell {
        Cell {
            row, column, 
            north: None, 
            south: None, 
            east: None, 
            west: None, 
            links: Vec::new(), 
        }
    }

    fn display_links(&self) {
        for link in &self.links {
            println!("{:?}", link.upgrade());
        }
    }

    // TODO: check this implementation
    fn neighbors(self) -> Vec<CellLinkWeak> {
        let mut vec: Vec<CellLinkWeak> = vec![];
        if self.north.is_some() {
            let c = self.north.unwrap().upgrade().unwrap();
            vec.push(Rc::downgrade(&c));
        }
        if self.south.is_some() {
            let c = self.south.unwrap().upgrade().unwrap();
            vec.push(Rc::downgrade(&c));
        }
        if self.east.is_some() {
            let c = self.east.unwrap().upgrade().unwrap();
            vec.push(Rc::downgrade(&c));
        }
        if self.west.is_some() {
            let c = self.west.unwrap().upgrade().unwrap();
            vec.push(Rc::downgrade(&c));
        }
        vec
    }

    fn links(&self) -> &Vec<CellLinkWeak> {
        &self.links
    }

    fn is_linked(&self, other: CellLinkStrong) -> bool {
        self.index_of_other(Rc::clone(&other)).is_some()        
    }

    fn index_of_other(&self, other: CellLinkStrong) -> Option<usize> {
        let other_row: u32 = other.borrow().row;
        let other_col: u32 = other.borrow().column;
        self.links.iter().position(|ref s| {
            let strong : CellLinkStrong = s.upgrade().unwrap();
            let c = strong.borrow();
            c.row == other_row && c.column == other_col
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut grid = Grid::new(2,2);
        grid.new_cell(0,0);
        grid.new_cell(0,1);
        grid.new_cell(1,0);
        grid.new_cell(1,1);

        let mut c00 = grid.get_cell(0,0).unwrap();
        let mut c01 = grid.get_cell(0,1).unwrap();

        link(Rc::clone(&c00), Rc::clone(&c01), true);
        println!("c00: {:?}", c00.borrow().display_links());
        println!("c01: {:?}", c01.borrow().display_links());
        println!("c00-c01 islinked {}", c00.borrow().is_linked(Rc::clone(&c01)));
        println!("c01-c00 islinked {}", c01.borrow().is_linked(Rc::clone(&c00)));

        println!("UNLINKING");
        unlink(Rc::clone(&c00), Rc::clone(&c01), true);
        println!("c00: {:?}", c00.borrow().display_links());
        println!("c01: {:?}", c01.borrow().display_links());
        println!("c00-c01 islinked {}", c00.borrow().is_linked(Rc::clone(&c01)));
        println!("c01-c00 islinked {}", c01.borrow().is_linked(Rc::clone(&c00)));

    }
}
