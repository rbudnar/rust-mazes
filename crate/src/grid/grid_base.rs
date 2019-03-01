use std::collections::HashMap;
use crate::grid::cell::{Cell, ICell};
use crate::grid::grid_web::add_bg_color;
use web_sys::{HtmlElement, Node};
use wasm_bindgen::JsCast;
use crate::grid::grid_web::add_class;
use web_sys::Element;
use web_sys::Document;
use crate::grid::cell::ICellStrong;
use crate::grid::{cell::CellLinkStrong, CellFormatter};
use crate::rng::RngWrapper;
use std::rc::{Rc};
use wasm_bindgen::prelude::JsValue;
use math::round;

#[derive(Debug)]
pub struct GridBase {
    pub cells: Vec<Vec<Option<CellLinkStrong>>>,
    pub cells_h: HashMap<usize, HashMap<usize, Option<CellLinkStrong>>>,
    pub rows: usize, 
    pub columns: usize
}

impl GridBase {
    pub fn new(rows: usize, columns: usize)-> GridBase {
        GridBase {
            cells: Vec::new(),
            cells_h: HashMap::new(),
            rows, columns            
        }
    }

    pub fn to_string(&self, contents: &dyn CellFormatter) -> String {
        let mut output = String::new();
        output += "\r";

        for (i, row) in self.cells.iter().enumerate() {
            let mut top = String::from("|");
            let mut bottom = String::from("+");
            for (j, cell) in row.iter().enumerate() {
                if let Some(cell) = cell {
                    if i == 0 {
                        output += "+---";
                        if j == self.columns -1 {
                            output += "+";
                        }
                    }                    

                    // let body = format!(" {} ", contents.contents_of(&cell));
                    let body = format!("   ");
                    let e = cell.borrow();
                    let east = e.east_i.as_ref();
                    let east_border = if east.is_some() && cell.borrow().is_linked_i(*east.unwrap()) {
                        " "
                    }
                    else {
                        "|"
                    };

                    top += &body;
                    top += east_border;

                    let south = e.south_i.as_ref();
                    let south_border = if south.is_some() && cell.borrow().is_linked_i(*south.unwrap()) {
                        "   "
                    }
                    else {
                        "---"
                    };
                    let corner = String::from("+");
                    bottom += south_border;
                    bottom += &corner;
                } else {
                    if i == 0 {
                        // very top of grid
                        if j > 0 && self.cells[i][j-1].is_some() {
                            output += "+   ";
                        } else {
                            output += "    ";
                        }                 
                    }

                    if j == 0 {
                        top = String::from(" ");
                    }                    
                    
                    // Handle sides 
                    if j == self.columns - 1 || (j > 0 && self.cells[i][j + 1].is_none()) {
                        top += "    ";
                    }
                    else {
                        top += "   |"; 
                    }

                    if j == 0 && (i == self. rows- 1 || (i < self.rows - 1 && self.cells[i+1][j].is_none())) {
                        bottom = String::from(" ");
                    }

                    // Here we need to check the next row/column and see if we need to render the bottom
                    // 1) Check if last row OR if before the last row and the cell below it is None
                    if i == self.rows -1 || i < self.rows - 1 && self.cells[i+1][j].is_none() {
                        // 2) If we aren't in the last column
                        if j < self.columns - 1 {
                            // 3) Check if the next cell to the right or the cell to the south east is present and render the corner
                            if self.cells[i][j+1].is_some() || self.cells[i+1][j+1].is_some() {
                                bottom += "   +";                                    
                            } else {
                                bottom += "    ";    
                            }

                        }
                        else {
                            bottom += "    ";
                        }
                    } else {
                        bottom += "---+";
                    }
                }
            }

            if i == 0 {
                output += "\r\n";
            }

            output += &format!("{}\r\n", &top);
            output += &format!("{}\r\n", &bottom);

        }
        
        output
    }


    pub fn random_cell(&self, rng: &dyn RngWrapper) -> Option<ICellStrong> {
        let row: usize = rng.gen_range(0, self.rows);
        let col: usize = rng.gen_range(0, self.columns);
        self.get_cell(row, col)
    }

    pub fn each_cell(&self) -> Vec<Option<ICellStrong>> { 
        self.cells.iter()
            .flatten()                
            .map(|x| {
                if let Some(x) = x {
                    Some(Rc::clone(x) as ICellStrong)
                }
                else {
                    None
                }
            })
            .collect()        
    }

    pub fn neighbors(&self, row: usize, column: usize) {
        let mut neighbors: Vec<ICellStrong> = vec![];
        if let Some(cell) = self.get_cell_h_rc(row, column) {
            for c in cell.borrow().neighbors_i().iter() {
                neighbors.push(Rc::clone(&self.get_cell_h(c).unwrap()))
            }
        }

    }

    pub fn get_cell_h_rc(&self, row: usize, column: usize) -> Option<CellLinkStrong> {
        if let Some(r) = self.cells_h.get(&row) {
            if let Some(cell) = r.get(&column) {
                if let Some(cell) = cell {
                    return Some(Rc::clone(&cell));
                }
            }
        }
        None
    }

    pub fn configure_cells_i(&mut self) {
        for row in &mut self.cells.iter() {
            for cell in &mut row.iter() {
                if let Some(cell) = cell {
                    // can't subtract from a usize of 0 apparently
                    let cell_row = cell.borrow().row;
                    if cell_row > 0 {
                        let north = self.get_cell(cell_row - 1, cell.borrow().column);
                        if north.is_some() {
                            cell.borrow_mut().north_i = Some(north.unwrap().borrow().index());
                        }
                    }

                    let south = self.get_cell(cell.borrow().row + 1, cell.borrow().column);
                    if south.is_some() {
                        cell.borrow_mut().south_i = Some(south.unwrap().borrow().index());
                    }

                    let east = self.get_cell(cell.borrow().row, cell.borrow().column + 1);
                    if east.is_some() {
                        cell.borrow_mut().east_i = Some(east.unwrap().borrow().index());
                    }

                    let cell_column = cell.borrow().column;
                    if cell_column > 0 {
                        let west = self.get_cell(cell.borrow().row, cell_column - 1);
                        if west.is_some() {
                            cell.borrow_mut().west_i = Some(west.unwrap().borrow().index());
                        }
                    }
                }
            }
        }

        for (row_key, row_h) in &mut self.cells_h.iter() {
            for (cell_key, cell) in &mut row_h.iter() {
                if let Some(cell) = cell {
                    // can't subtract from a usize of 0 apparently
                    let cell_row = cell.borrow().row;
                    if cell_row > 0 {
                        let north = self.get_cell(cell_row - 1, cell.borrow().column);
                        if north.is_some() {
                            cell.borrow_mut().north_i = Some(north.unwrap().borrow().index());
                        }
                    }

                    let south = self.get_cell(cell.borrow().row + 1, cell.borrow().column);
                    if south.is_some() {
                        cell.borrow_mut().south_i = Some(south.unwrap().borrow().index());
                    }

                    let east = self.get_cell(cell.borrow().row, cell.borrow().column + 1);
                    if east.is_some() {
                        cell.borrow_mut().east_i = Some(east.unwrap().borrow().index());
                    }

                    let cell_column = cell.borrow().column;
                    if cell_column > 0 {
                        let west = self.get_cell(cell.borrow().row, cell_column - 1);
                        if west.is_some() {
                            cell.borrow_mut().west_i = Some(west.unwrap().borrow().index());
                        }
                    }
                }
            }
        }
    }

    pub fn get_cell_h(&self, index: &usize) -> Option<ICellStrong> {
        let row = round::floor(*index as f64 / self.rows as f64, 0) as usize;
        let column = (index % self.rows) as usize;
        let row_h = self.cells_h.get(&row);
        if let Some(r) = row_h {
            let cell = r.get(&column);
            if let Some(cell) = cell {
                if let Some(cell) = cell {
                    return Some(Rc::clone(cell) as ICellStrong);
                }
            }
        }
        return None;
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Option<ICellStrong> {
        if row >= self.rows || column >= self.columns {
            return None;
        }
        
        if let Some(c) = self.cells[row][column].clone() {
            return Some(c as ICellStrong)
        }
        return None;
    }

    pub fn cells(&self) -> Vec<Vec<Option<ICellStrong>>> {
        self.cells.iter().map(|row| 
            row.iter().map(|c| Some(Rc::clone(&c.as_ref().unwrap()) as ICellStrong)).collect()
        ).collect()
    }    

    pub fn get_cell_at_index(&self, index: usize) -> ICellStrong {
        let cells = self.each_cell();
        let c = cells.iter().find(|c| {
            if let Some(c) = c {
                return c.borrow().index() == index
            }
            return false;
        }).unwrap();

        return Rc::clone(&c.as_ref().unwrap())
    }

    pub fn get_cell_links(&self, index: usize) -> Vec<ICellStrong> {
        self.each_cell().iter().filter(|&c| {
            if let Some(c) = c {
                return c.borrow().links().contains(&index);
            }
            return false;
        }).map(|c| Rc::clone(&c.as_ref().unwrap())).collect()
    }

    
    // pub fn dead_ends(&self) -> Vec<Option<CellLinkStrong>> {
    //     self.each_cell().iter()
    //         .filter(|c| {
    //             if let Some(c) = c {
    //                 c.borrow().links.len() == 1
    //             } else {
    //                 false
    //             }
    //         })
    //         .map(|x| {
    //             if let Some(x) = x {
    //                 Some(Rc::clone(x))
    //             }
    //             else {
    //                 None
    //             }
    //         })
    //         .collect()
    // }

    pub fn to_web(&self, document: &Document, grid_container: &Element, formatter: &dyn CellFormatter, colorize: bool) {
        web_sys::console::log_1(&JsValue::from_str(&format!("{:?}", self.cells)));

        for (i, row) in self.cells().iter().enumerate() {
            web_sys::console::log_1(&JsValue::from_str(&format!("{}", "row")));

            for (j, cell) in row.iter().enumerate() {    
                web_sys::console::log_1(&JsValue::from_str(&format!("{}", 1)));

                if let Some(cell) = cell {
                    web_sys::console::log_1(&JsValue::from_str(&format!("{:?}", cell)));
                    let html_cell = document.create_element("div").unwrap();
                    add_class(&html_cell, "cell");

                    // Top of maze
                    if i == 0 || (i > 0 && self.cells()[i-1][j].is_none()) {
                        add_class(&html_cell, "bt");
                    }

                    // bottom of maze
                    if i == self.rows - 1 {
                        add_class(&html_cell, "bb");
                    }
                    // left side 
                    if j == 0  || ( j > 0 && self.cells()[i][j-1].is_none()) {
                        add_class(&html_cell, "bl");
                    }

                    // right side
                    if j == self.columns -1 {
                        add_class(&html_cell, "br");
                    }

                    if let Some(c) = cell.borrow().as_any().downcast_ref::<Cell>() {
                        let east = c.east_i;
                        if !(east.is_some() && c.is_linked_i(east.unwrap())) {
                            add_class(&html_cell, "br");            
                        }

                        let south = c.south_i;
                        if !(south.is_some() && c.is_linked_i(south.unwrap())) {
                            add_class(&html_cell, "bb");            
                        }
                    }
                    else {
                        web_sys::console::log_1(&JsValue::from_str(&format!("no e")));
                    }

                    let c = html_cell.dyn_ref::<HtmlElement>().unwrap().clone();
                    if colorize {
                        add_bg_color(&c, cell, formatter);
                    }
                    grid_container.append_child(&Node::from(html_cell)).unwrap();
                }
                else {
                    web_sys::console::log_1(&JsValue::from_str("else no cell"));
                    let html_cell = document.create_element("div").unwrap();
                    add_class(&html_cell, "cell");
                
                    html_cell.dyn_ref::<HtmlElement>().unwrap().style().set_property("background-color", "white").unwrap();
                    grid_container.append_child(&Node::from(html_cell)).unwrap();
                }
            }
        }
    }
}