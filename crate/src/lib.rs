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

// #[derive(Eq, PartialEq, Debug)]
// struct Cell<'a> {
//     row: u32,
//     column: u32,

//     links: HashSet<Box<&'a Cell<'a>>>, //<'a>
//     // north: &'a Box<Cell<'a>>,
//     // north: Option<Box<Cell<'a>>>,
//     // south: Option<Box<Cell<'a>>>,
//     // east: Option<Box<Cell<'a>>>,
//     // west: Option<Box<Cell<'a>>>
// }

// impl<'a> Cell<'a> {
//     fn new(row: u32, column: u32) -> Cell<'a> {
//         Cell {
//             row, column, //north: None, south: None, east: None, west: None, 
//             links: HashSet::new()
//         }
//     }

//     fn link(&'a mut self, other: &'a mut Cell<'a>, bidir: bool) {
//         // let b = Box::new(other);
//         self.links.insert(Box::new(other));
//         if bidir {
//             other.link(self, false);
//         }
//     }

//     fn unlink(&self, other: &Cell, bidir: bool) {
        
//         if bidir {
//             other.unlink(&self, false);
//         }
//     }
// }

// impl<'a> Hash for Cell<'a> {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.row.hash(state);
//         self.column.hash(state);
//     }
// }





#[derive(Debug)]
// #[derive(Eq, PartialEq, Debug)]
struct Cell {
    row: u32,
    column: u32,

    // links1: HashSet<Weak<RefCell<Cell>>>,
    links: Vec<Weak<RefCell<Cell>>>,
    // north: &'a Box<Cell<'a>>,
    // north: Option<Box<Cell<'a>>>,
    // south: Option<Box<Cell<'a>>>,
    // east: Option<Box<Cell<'a>>>,
    // west: Option<Box<Cell<'a>>>
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.row == other.row && self.column == other.column
    }
}
impl Eq for Cell {}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.column.hash(state);
    }
}

// pub struct CellWrapper(Rc<RefCell<Cell>>);
// #[derive(Eq, PartialEq, Debug)]
#[derive(Debug)]
pub struct Grid {
    cells: Vec<Rc<RefCell<Cell>>>,
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

    fn get_cell(&self, row: u32, column: u32) -> Option<Rc<RefCell<Cell>>> {
        let mut iter = self.cells.iter();
        iter.find(|ref x| x.borrow().row == row && x.borrow().column == column)
            .map(|ref x| Rc::clone(&x))
    }
}

fn link(this: Rc<RefCell<Cell>>, other: Rc<RefCell<Cell>>, bidir: bool) {    
    let newlink: Weak<RefCell<Cell>> = Rc::downgrade(&other);
    // let newlink: Rc<RefCell<Cell>> = Rc::clone(&other);
    this.borrow_mut().links.push(newlink);
    if bidir {
        link(Rc::clone(&other), Rc::clone(&this), false);
    }
}

impl Cell {
    fn new(row: u32, column: u32) -> Cell {
        Cell {
            row, column, //north: None, south: None, east: None, west: None, 
            links: Vec::new(),            
            //links: HashSet::new()
        }
    }

    fn display_links(&self) {
        for link in &self.links {
            println!("{:?}", link.upgrade());
        }
    }

    fn unlink(&self, other: &Cell, bidir: bool) {
        
        if bidir {
            other.unlink(&self, false);
        }
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
        println!("c00: {:?}", c00.borrow());
        println!("c01: {:?}", c01.borrow());
        println!("c00: {:?}", c00.borrow().display_links());
        println!("c01: {:?}", c01.borrow().display_links());
    }
}
