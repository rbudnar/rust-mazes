use std::collections::HashMap;
use crate::grid::cell::{Cell};
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


#[derive(Debug)]
pub struct GridBase {
    pub cells: Vec<Vec<Option<CellLinkStrong>>>,
    pub rows: usize, 
    pub columns: usize
}

impl GridBase {
    pub fn new(rows: usize, columns: usize)-> GridBase {
        GridBase {
            cells: Vec::new(),
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
                    let east = e.east.as_ref();
                    let east_border = if east.is_some() && cell.borrow().is_linked(east.as_ref().unwrap().upgrade().unwrap()) {
                        " "
                    }
                    else {
                        "|"
                    };

                    top += &body;
                    top += east_border;

                    let south = e.south.as_ref();
                    let south_border = if south.is_some() && cell.borrow().is_linked(south.as_ref().unwrap().upgrade().unwrap()) {
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

    pub fn random_cell(&self, rng: &dyn RngWrapper) -> Option<CellLinkStrong> {
        let row: usize = rng.gen_range(0, self.rows);
        let col: usize = rng.gen_range(0, self.columns);
        self.get_cell(row, col)
    }

    pub fn each_cell(&self) -> Vec<Option<CellLinkStrong>> { 
        self.cells.iter()
            .flatten()                
            .map(|x| {
                if let Some(x) = x {
                    Some(Rc::clone(x))
                }
                else {
                    None
                }
            })
            .collect()        
    }

    pub fn configure_cells(&mut self) {
        for row in &mut self.cells.iter() {
            for cell in &mut row.iter() {
                if let Some(cell) = cell {
                    // can't subtract from a usize of 0 apparently
                    let cell_row = cell.borrow().row;
                    if cell_row > 0 {
                        let north = self.get_cell(cell_row - 1, cell.borrow().column);
                        if north.is_some() {
                            cell.borrow_mut().north = Some(Rc::downgrade(&Rc::clone(&north.unwrap())));
                        }
                    }

                    let south = self.get_cell(cell.borrow().row + 1, cell.borrow().column);
                    if south.is_some() {
                        cell.borrow_mut().south = Some(Rc::downgrade(&Rc::clone(&south.unwrap())));
                    }

                    let east = self.get_cell(cell.borrow().row, cell.borrow().column + 1);
                    if east.is_some() {
                        cell.borrow_mut().east = Some(Rc::downgrade(&Rc::clone(&east.unwrap())));
                    }

                    let cell_column = cell.borrow().column;
                    if cell_column > 0 {
                        let west = self.get_cell(cell.borrow().row, cell_column - 1);
                        if west.is_some() {
                            cell.borrow_mut().west = Some(Rc::downgrade(&Rc::clone(&west.unwrap())));
                        }
                    }
                }
            }
        }
    }

    pub fn cells(&self) -> Vec<Vec<Option<ICellStrong>>> {
        self.cells.iter().map(|row| 
            row.iter().map(|c| Some(Rc::clone(&c.as_ref().unwrap()) as ICellStrong)).collect()
        ).collect()
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Option<CellLinkStrong> {
        if row >= self.rows || column >= self.columns {
            return None;
        }
        
        self.cells[row][column].clone()
    }

    pub fn dead_ends(&self) -> Vec<Option<CellLinkStrong>> {
        self.each_cell().iter()
            .filter(|c| {
                if let Some(c) = c {
                    c.borrow().links.len() == 1
                } else {
                    false
                }
            })
            .map(|x| {
                if let Some(x) = x {
                    Some(Rc::clone(x))
                }
                else {
                    None
                }
            })
            .collect()
    }

    pub fn to_web(&self, document: &Document, grid_container: &Element, formatter: &dyn CellFormatter, colorize: bool) {
        for (i, row) in self.cells().iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {    
                if let Some(cell) = cell {
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
                        let east = c.east.as_ref();
                        if !(east.is_some() && c.is_linked(east.unwrap().upgrade().unwrap())) {
                            add_class(&html_cell, "br");            
                        }

                        let south = c.south.as_ref();
                        if !(south.is_some() && c.is_linked(south.unwrap().upgrade().unwrap())) {
                            add_class(&html_cell, "bb");            
                        }
                    }

                    let c = html_cell.dyn_ref::<HtmlElement>().unwrap().clone();
                    if colorize {
                        add_bg_color(&c, cell, formatter);
                    }
                    grid_container.append_child(&Node::from(html_cell)).unwrap();
                }
                else {
                    // web_sys::console::log_1(&JsValue::from_str("else no cell"));
                    let html_cell = document.create_element("div").unwrap();
                    add_class(&html_cell, "cell");
                
                    html_cell.dyn_ref::<HtmlElement>().unwrap().style().set_property("background-color", "white").unwrap();
                    grid_container.append_child(&Node::from(html_cell)).unwrap();
                }
            }
        }
    }
}