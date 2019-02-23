use crate::grid::cell::{ICell, ICellStrong};
use std::rc::Rc;
use crate::grid::{Grid};
use crate::algorithms::rand_element;
use crate::rng::RngWrapper;
use crate::algorithms::MazeAlgorithm;
use wasm_bindgen::prelude::JsValue;

#[derive(Debug)]
pub struct RecursiveBacktracker;


/// 1) Start at a random cell, add to stack
/// 2) Randomly pick an unvisited neighbor, move to it and add to stack
/// 3) When a cell with no unvisited neighbors is reached, pop stack until you get to a cell with unvisited neighbors
/// 4) repeat until stack is empty
impl MazeAlgorithm for RecursiveBacktracker {
    fn on(&self, grid: &dyn Grid, rng_generator: &dyn RngWrapper) {
        let cells = grid.each_cell();
        let mut stack: Vec<ICellStrong> = vec![];
        let mut start = rand_element(&cells, rng_generator);
        while start.is_none() {
            start = rand_element(&cells, rng_generator);
        }
        web_sys::console::log_1(&JsValue::from_str(&format!("{}", 1)));
        stack.push(start.clone().unwrap());
        web_sys::console::log_1(&JsValue::from_str(&format!("{}", 2)));
        while !stack.is_empty() {
            web_sys::console::log_1(&JsValue::from_str(&format!("{}", stack.len())));
            // web_sys::console::log_1(&JsValue::from_str(&format!("{}", 3)));
            let current = stack.last().unwrap().clone();          
            let neighbors = current.borrow().neighbors_i();

            let neighbors_: Vec<ICellStrong> = grid.each_cell().iter()
                .filter(|ref c| {
                    // return c.is_some();
                    if let Some(c) = c.as_ref() {
                        return c.borrow().neighbors_i().contains(&current.borrow().index());
                    }
                    return false;
                }).map(|c| Rc::clone(&c.as_ref().unwrap())).collect();
                        
            // let unvisited: Vec<ICellStrong> = 
            //             grid.each_cell().iter().filter(|ref c| {
            //                 web_sys::console::log_1(&JsValue::from_str(&format!("{}", 5)));
            //                 // let c = c.as_ref().unwrap().borrow();
            //                 if let Some(c) = c.as_ref() {
            //                     let c = c.borrow();
            //                     web_sys::console::log_1(&JsValue::from_str(&format!("{}", 6)));
            //                     let include = neighbors.contains(&c.index()) && c.links().is_empty();
            //                     web_sys::console::log_1(&JsValue::from_str(&format!("{}", 7)));
            //                     return include;
            //                 }
            //                 return false;
            //                 // neighbors.contains(&c.index()) && c.links().is_empty()
            //             })
            //             .map(|c2| Rc::clone(c2.as_ref().unwrap()))
            //             .collect();

            let unvisited: Vec<ICellStrong> = 
                neighbors_.iter().filter(|ref c| {
                    // let c = c.as_ref().unwrap().borrow();
                    // if let Some(c) = c.as_ref() {
                    let c = c.borrow();
                    // let include = neighbors.contains(&c.index()) && c.links().is_empty();
                    let include = c.links().is_empty();
                    return include;
                    // }
                    // return false;
                    // neighbors.contains(&c.index()) && c.links().is_empty()
                })
                .map(|c2| Rc::clone(c2))
                .collect();
            // web_sys::console::log_1(&JsValue::from_str(&format!("{}", 8)));

            if !unvisited.is_empty() {
                // web_sys::console::log_1(&JsValue::from_str(&format!("{}", "not empty")));
                let rand_neighbor = rand_element(&unvisited, rng_generator);
                current.borrow_mut().link(rand_neighbor.borrow().index());
                rand_neighbor.borrow_mut().link(current.borrow().index());
                stack.push(Rc::clone(&rand_neighbor));
            }
            else {
                // web_sys::console::log_1(&JsValue::from_str(&format!("{}", "empty")));
                stack.pop();
            }
        }         
    }
}