use crate::grid::grid_base::cell_coords_with_inset;
use wasm_bindgen::{prelude::JsValue, JsCast};
use std::cell::RefCell;
use std::any::Any;
use std::rc::{Rc};
use super::{grid_base::GridBase, Grid, CellFormatter};
use crate::grid::canvas::{setup_grid_canvas, DrawMode, draw_line, remove_old_canvas, set_canvas_size, draw_shape};
use crate::cells::{ICellStrong, cell::Cell, over_cell::{OverCellLinkStrong, OverCell}};
use crate::rng::RngWrapper;

pub static WEAVE_GRID: &str = "weave_grid";

#[derive(Debug)]
pub struct WeaveGrid {
    pub cells: Vec<Vec<Option<OverCellLinkStrong>>>,
    pub rows: usize, 
    pub columns: usize,
    pub under_cells: Vec<Option<OverCellLinkStrong>>,
    _cells: Option<Vec<Vec<Option<ICellStrong>>>>,
    self_rc: Option<Rc<RefCell<WeaveGrid>>>
}

impl WeaveGrid {
    pub fn new(rows: usize, columns: usize) -> Rc<RefCell<WeaveGrid>> {
        let mut weave_grid = WeaveGrid {
             cells: Vec::new(),
            rows, columns,
            _cells: None,
            under_cells: vec![],
            self_rc: None
        };

        let rc = Rc::new(RefCell::new(weave_grid));
        rc.borrow_mut().self_rc = Some(Rc::clone(&rc));

        rc.borrow_mut().prepare_grid();
        rc.borrow_mut().configure_cells();

        rc
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

    fn each_over_cell(&self) -> Vec<Option<OverCellLinkStrong>> {
        self.cells.iter()
            .flatten()
            .chain(&self.under_cells)                
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

    pub fn get_cell_link_strong(&self, row: usize, column: usize) -> Option<OverCellLinkStrong> {
        if row >= self.rows || column >= self.columns {
            return None;
        }
        
        self.cells[row][column].clone()
    }

    fn create_cells(&mut self) {
        if self._cells.is_some() {
            return;
        }

        self._cells = Some(
            self.cells.iter().map(|row| 
                row.iter().map(|c| {
                    if let Some(c) = c {
                        return Some(Rc::clone(&c) as ICellStrong);
                    }
                    None
                }).collect()
            ).collect());
    }

    
    pub fn tunnel_under(&mut self, over_cell: &OverCellLinkStrong) {
        let under_cell = OverCell::new(0, 0, Some(Rc::clone(&over_cell)), Rc::clone(self.self_rc.as_ref().unwrap()));
        self.under_cells.push(Some(under_cell));
    }
}

impl Grid for WeaveGrid {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn new_cell(&self, row: usize, column: usize) -> ICellStrong {
        Cell::new(row, column)
    }

    fn prepare_grid(&mut self) {
        for i in 0..self.rows {
            let mut row: Vec<Option<OverCellLinkStrong>> = Vec::new();
            
            for j in 0..self.columns {
                row.push(Some(OverCell::new(i as usize, j as usize, None, Rc::clone(self.self_rc.as_ref().unwrap()))));
            }
            self.cells.push(row);
        }   
    }

    fn random_cell(&self, rng: &dyn RngWrapper<Shuffle=ICellStrong>) -> Option<ICellStrong> {
        let row: usize = rng.gen_range(0, self.rows);
        let col: usize = rng.gen_range(0, self.columns);
        self.get_cell(row, col)
    }

    fn each_cell(&self) -> Vec<Option<ICellStrong>> {
        self.cells.iter()
            .flatten()
            .chain(&self.under_cells)                
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

    fn rows(&self) -> usize {
        self.columns
    }

    fn columns(&self) -> usize {
        self.rows
    }

    fn cells(&self) -> &Vec<Vec<Option<ICellStrong>>> {
        self._cells.as_ref().unwrap()
    }
    
    fn get_cell(&self, row: usize, column: usize) -> Option<ICellStrong> {
        if row >= self.rows || column >= self.columns {
            return None;
        }
        if let Some(cell) = self.cells[row][column].clone() {
            return Some(Rc::clone(&cell) as ICellStrong);
        }
        None
    }

    fn to_string(&self, contents: &dyn CellFormatter) -> String {
        "".to_string()
    }

    fn size(&self) -> usize {
        self.rows * self.columns
    }
    
    fn braid(&self, p: f64, rng: &dyn RngWrapper<Shuffle=ICellStrong>) {
        // self.braid(p, rng);
    }

    fn to_web(&self, formatter: &dyn CellFormatter, colorize: bool, inset: f64) {
        let inset = 0.1;
        remove_old_canvas(WEAVE_GRID);
        let context = setup_grid_canvas(WEAVE_GRID).unwrap();
        let size = 75;
        
        set_canvas_size(WEAVE_GRID, size * self.columns, size * self.rows);
        let size = size as f64;
        context.set_fill_style(&JsValue::from_str("black"));
        context.set_stroke_style(&JsValue::from_str("black"));

        for mode in [DrawMode::Background, DrawMode::Line].iter() {
            for cell in self.each_over_cell() {    
                if let Some(cell) = cell {
                    let x = (cell.borrow().column as f64) * size;
                    let y = (cell.borrow().row as f64) * size;
                    if inset > 0.0 {
                        draw_with_inset(cell, &context, size, mode, formatter, colorize, x, y, inset * size);
                    }
                    else {
                        draw_no_inset(cell, &context, size, mode, formatter, colorize, x, y);
                    }
                }
            }
        }
    }
}

fn draw_with_inset(cell: OverCellLinkStrong, context: &web_sys::CanvasRenderingContext2d, size: f64, mode: &DrawMode, formatter: &dyn CellFormatter, colorize: bool, x: f64, y: f64, inset: f64)  {
    let (x1, x2, x3, x4, y1, y2, y3, y4) = cell_coords_with_inset(x, y, size, inset);
    let is_under_cell = cell.borrow().is_under_cell;
    if is_under_cell {
        let cell = cell.borrow();
        if cell.vertical_passage() {
            draw_line(&context, x2, y1, x2, y2);
            draw_line(&context, x3, y1, x3, y2);
            draw_line(&context, x2, y3, x2, y4);
            draw_line(&context, x3, y3, x3, y4);
        } else {
            draw_line(&context, x1, y2, x2, y2);
            draw_line(&context, x1, y3, x2, y3);
            draw_line(&context, x3, y2, x4, y2);
            draw_line(&context, x3, y3, x4, y3);
        }

        return;
    }

    match mode {
        DrawMode::Background => {
            // nooooot quite right...
            // let points = vec![(x2, y2), (x3, y2), (x3, y3), (x2, y3)];
            // let ics: ICellStrong =  Rc::clone(&cell) as ICellStrong;
            // let color = formatter.background_color(&ics);
            // draw_shape(&context, points, &color);
        },
        DrawMode::Line => {
            let north = &cell.as_ref().borrow().north;            
            if north.is_some() && cell.borrow().is_linked(north.as_ref().unwrap().upgrade().unwrap()) {
                draw_line(&context, x2, y1, x2, y2);
                draw_line(&context, x3, y1, x3, y2);
            } else {
                draw_line(&context, x2, y2, x3, y2);
            }
            
            let south = &cell.as_ref().borrow().south;            
            if south.is_some() && cell.borrow().is_linked(south.as_ref().unwrap().upgrade().unwrap()) {
                draw_line(&context, x2, y3, x2, y4);
                draw_line(&context, x3, y3, x3, y4);
            } else {
                draw_line(&context, x2, y3, x3, y3);
            }

            let west = &cell.as_ref().borrow().west;            
            if west.is_some() && cell.borrow().is_linked(west.as_ref().unwrap().upgrade().unwrap()) {
                draw_line(&context, x1, y2, x2, y2);
                draw_line(&context, x1, y3, x2, y3);
            } else {
                draw_line(&context, x2, y2, x2, y3);
            }
            
            let east = &cell.as_ref().borrow().east;            
            if east.is_some() && cell.borrow().is_linked(east.as_ref().unwrap().upgrade().unwrap()) {
                draw_line(&context, x3, y2, x4, y2);
                draw_line(&context, x3, y3, x4, y3);
            } else {
                draw_line(&context, x3, y2, x3, y3);
            }
        }
    }

}

fn draw_no_inset(cell: OverCellLinkStrong, context: &web_sys::CanvasRenderingContext2d, size: f64, mode: &DrawMode, formatter: &dyn CellFormatter, colorize: bool, x: f64, y: f64) {
    let x1 = x;
    let y1 = y;
    let x2 = x1 + size;
    let y2 = y1 + size;

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


pub fn to_cls(vec: &Vec<ICellStrong>) -> Vec<OverCellLinkStrong> {
    vec.iter()
        .map(|x| { 
            let cell = x.borrow();
            let cell = cell.as_any().downcast_ref::<OverCell>().unwrap();
            Rc::clone(&cell.self_rc.upgrade().unwrap())            
        }).collect()
}