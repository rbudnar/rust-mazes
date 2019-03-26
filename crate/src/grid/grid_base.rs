use crate::algorithms::rand_element;
use std::rc::{Rc};
use wbg_rand::{Rng, wasm_rng};
use wasm_bindgen::{prelude::JsValue, JsCast};
use crate::grid::{CellFormatter, standard_grid::STANDARD_GRID, canvas::{setup_grid_canvas, DrawMode, draw_line, remove_old_canvas, set_canvas_size, draw_shape}};
use crate::cells::{ICellStrong, cell::{CellLinkStrong}, ICell};
use crate::rng::RngWrapper;
use crate::cells::cell::Cell;
// Not sure how to get around explicitly using wasm_rng.

#[derive(Debug)]
pub struct GridBase {
    pub cells: Vec<Vec<Option<CellLinkStrong>>>,
    pub rows: usize, 
    pub columns: usize,
    pub cells_: Option<Vec<Vec<Option<ICellStrong>>>>
}

impl GridBase {
    pub fn new(rows: usize, columns: usize)-> GridBase {
        GridBase {
            cells: Vec::new(),
            rows, columns,
            cells_: None
        }
    }

    pub fn to_string(&self, _contents: &dyn CellFormatter) -> String {
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
                    let body = "   ".to_string();
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

    pub fn random_cell(&self, rng: &dyn RngWrapper<Shuffle=ICellStrong>) -> Option<ICellStrong> {
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

    pub fn each_std_cell(&self) -> Vec<Option<CellLinkStrong>> {     
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
                        let north = self.get_cell_link_strong(cell_row - 1, cell.borrow().column);
                        if north.is_some() {
                            cell.borrow_mut().north = Some(Rc::downgrade(&Rc::clone(&north.unwrap())));
                        }
                    }

                    let south = self.get_cell_link_strong(cell.borrow().row + 1, cell.borrow().column);
                    if south.is_some() {
                        cell.borrow_mut().south = Some(Rc::downgrade(&Rc::clone(&south.unwrap())));
                    }

                    let east = self.get_cell_link_strong(cell.borrow().row, cell.borrow().column + 1);
                    if east.is_some() {
                        cell.borrow_mut().east = Some(Rc::downgrade(&Rc::clone(&east.unwrap())));
                    }

                    let cell_column = cell.borrow().column;
                    
                    if cell_column > 0 {
                        let west = self.get_cell_link_strong(cell.borrow().row, cell_column - 1);
                        if west.is_some() {
                            cell.borrow_mut().west = Some(Rc::downgrade(&Rc::clone(&west.unwrap())));
                        }
                    }
                }
            }
        }

        self.create_cells();
    }

    pub fn cells(&self) -> &Vec<Vec<Option<ICellStrong>>> {
        self.cells_.as_ref().unwrap()
    }

    fn create_cells(&mut self) {
        if self.cells_.is_some() {
            return;
        }

        self.cells_ = Some(
            self.cells.iter().map(|row| 
                row.iter().map(|c| {
                    if let Some(c) = c {
                        return Some(Rc::clone(&c) as ICellStrong);
                    }
                    None
                }).collect()
            ).collect());
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Option<ICellStrong> {
        if row >= self.rows || column >= self.columns {
            return None;
        }
        if let Some(cell) = self.cells[row][column].clone() {
            return Some(Rc::clone(&cell) as ICellStrong);
        }
        None
    }

    pub fn get_cell_link_strong(&self, row: usize, column: usize) -> Option<CellLinkStrong> {
        if row >= self.rows || column >= self.columns {
            return None;
        }
        
        self.cells[row][column].clone()
    }

    pub fn dead_ends(&self) -> Vec<ICellStrong> {
        self.each_cell().iter()
            .filter(|c| {
                if let Some(c) = c {
                    c.borrow().links().len() == 1
                } else {
                    false
                }
            })
            .map(|x| {
                Rc::clone(x.as_ref().unwrap())
            })
            .collect()
    }

    // pub fn dead_ends(&self) -> Vec<CellLinkStrong> {
    //     self.each_std_cell().iter()
    //         .filter(|c| {
    //             if let Some(c) = c {
    //                 c.borrow().links.len() == 1
    //             } else {
    //                 false
    //             }
    //         })
    //         .map(|x| {
    //             Rc::clone(&x.as_ref().unwrap())
    //         })
    //         .collect()
    // }

    

    pub fn braid(&self, p: f64, rng: &dyn RngWrapper<Shuffle=ICellStrong>) {
        let mut dead_ends = self.dead_ends();
        rng.shuffle(&mut dead_ends);

        for cell in to_cls(&dead_ends).iter() {
            let rand: f64 = wasm_rng().gen_range(0_f64, 1_f64);

            if cell.borrow().links.len() != 1 || p > rand {
                continue;
            }

            let best = {
                let cell = cell.borrow();
                let neighbors = cell.neighbors_std();
                let neighbors: Vec<CellLinkStrong> = neighbors.iter().filter(|c| {
                    let c = Rc::clone(c);
                    !cell.is_linked(c)
                }).map(|c| Rc::clone(c)).collect();

                let mut best: Vec<CellLinkStrong> = neighbors.iter()
                    .filter(|c| c.borrow().links.len() == 1)
                    .map(|x| Rc::clone(x))
                    .collect();

                if best.is_empty() {
                    best = neighbors;
                }

                best
            };            

            let neighbor_to_link: CellLinkStrong = rand_element(&best, rng).clone();

            // web_sys::console::log_1(&JsValue::from_str(&format!("borrowing")));
            // web_sys::console::log_1(&JsValue::from_str(&format!("borrowed")));
            // web_sys::console::log_1(&JsValue::from_str(&format!("last time")));
            // web_sys::console::log_1(&JsValue::from_str(&format!("done")));

            cell.borrow_mut().link(neighbor_to_link.clone());
            neighbor_to_link.borrow_mut().link(cell.clone());
        }
    }
    

    pub fn to_web(&self, formatter: &dyn CellFormatter, colorize: bool) {
        remove_old_canvas(STANDARD_GRID);
        let context = setup_grid_canvas(STANDARD_GRID).unwrap();
        let size = 15;
        
        set_canvas_size(STANDARD_GRID, size * self.columns, size * self.rows);
        let size = size as f64;
        context.set_fill_style(&JsValue::from_str("black"));
        context.set_stroke_style(&JsValue::from_str("black"));

        for mode in [DrawMode::Background, DrawMode::Line].iter() {
            for cell in self.each_std_cell() {    
                if let Some(cell) = cell {

                    let x1 = (cell.borrow().column as f64) * size;
                    let y1 = (cell.borrow().row as f64) * size;
                    let x2 = (cell.borrow().column as f64) * size + size;
                    let y2 = (cell.borrow().row as f64) * size + size;


                    match mode {
                        DrawMode::Background => { 
                            if colorize {
                                let points = vec![(x1, y1), (x2, y1), (x2, y2), (x1, y2)];
                                let ics: ICellStrong =  Rc::clone(&cell) as ICellStrong;
                                let color = formatter.background_color(&ics);
                                draw_shape(&context, points, &color);
                            }
                        },
                        DrawMode::Line => {
                            if cell.borrow().north.is_none() {
                                draw_line(&context, x1, y1, x2, y1);
                            }
                            
                            if cell.borrow().west.is_none() {
                                draw_line(&context, x1, y1, x1, y2);
                            }

                            if cell.borrow().is_not_linked(&cell.borrow().east) {
                                draw_line(&context, x2, y1, x2, y2);
                            }

                            if cell.borrow().is_not_linked(&cell.borrow().south) {
                                draw_line(&context, x1, y2, x2, y2);
                            }
                        }
                    };
                }
            }
        }
    }
}

pub fn to_cls(vec: &Vec<ICellStrong>) -> Vec<CellLinkStrong> {
    vec.iter()
        .map(|x| { 
            let cell = x.borrow();
            let cell = cell.as_any().downcast_ref::<Cell>().unwrap();
            Rc::clone(&cell.self_rc.upgrade().unwrap())            
        }).collect()
}

fn do_stuff(cell: CellLinkStrong) -> Vec<CellLinkStrong> {
    let cell = cell.borrow();
    let neighbors = cell.neighbors_std();
    let neighbors: Vec<CellLinkStrong> = neighbors.iter().filter(|c| {
        let c = Rc::clone(c);
        !cell.is_linked(c)
    }).map(|c| Rc::clone(c)).collect();

    let mut best: Vec<CellLinkStrong> = neighbors.iter()
        .filter(|c| c.borrow().links.len() == 1)
        .map(|x| Rc::clone(x))
        .collect();

    if best.is_empty() {
        best = neighbors;
    }

    best
}