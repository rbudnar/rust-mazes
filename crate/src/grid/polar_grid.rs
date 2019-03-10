use crate::grid::cell::ICellStrong;
use crate::grid::polar_cell::PolarCellLinkStrong;
use crate::grid::CellFormatter;
use crate::rng::RngWrapper;
use crate::grid::polar_cell::PolarCell;
use crate::grid::Grid;
use std::rc::Rc;
use std::f64::consts::PI;
use web_sys::*;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::JsCast;
use math::round;


pub struct PolarGrid {
    pub cells: Vec<Vec<Option<PolarCellLinkStrong>>>,
    pub rows: usize, 
    pub columns: usize,
    pub cells_: Option<Vec<Vec<Option<ICellStrong>>>>
}

impl PolarGrid {
    pub fn new(rows: usize, columns: usize) -> PolarGrid {
        let mut grid = PolarGrid {
            cells: Vec::new(),
            rows, columns,
            cells_: None
        };
        grid.prepare_grid();
        grid.configure_cells();
        grid.create_cells();

        grid        
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

    fn configure_cells(&mut self) {
        for cell in self.each_polar_cell().iter() {
            if let Some(cell) = cell {
                let row = cell.borrow().row;
                let col = cell.borrow().column;
                if row > 0 { 
                    if let Some(cw) = self.get_polar_cell(row, col+1) {
                        cell.borrow_mut().cw = Some(Rc::downgrade(&cw));
                    }
                    if col > 0 {
                        if let Some(ccw) = self.get_polar_cell(row, col-1) {
                            cell.borrow_mut().ccw = Some(Rc::downgrade(&ccw));
                        }
                    } else {
                        let col = self.cells[row].len();
                        if let Some(ccw) = self.get_polar_cell(row, col-1) {
                            cell.borrow_mut().ccw = Some(Rc::downgrade(&ccw));
                        }
                    }
                    let ratio = self.cells[row].len() / self.cells[row - 1].len();
                    if let Some(parent) = &self.get_polar_cell(row - 1, col / ratio) {
                        parent.borrow_mut().outward.push(Some(Rc::downgrade(cell)));
                        cell.borrow_mut().inward = Some(Rc::downgrade(parent));
                    }
                }
            }
        }
    }

    fn get_polar_cell(&self, row: usize, column: usize) -> Option<PolarCellLinkStrong> {
        if row >= self.rows {
            return None;
        }
        self.cells[row][column % self.cells[row].len()].clone()
    }

    fn each_polar_cell(&self) -> Vec<Option<PolarCellLinkStrong>> {
        self.cells.iter()
            .flatten()                
            .map(|x| { 
                if let Some(x) = x {
                    Some(Rc::clone(x) )
                }
                else {
                    None
                }
            })
            .collect()   
    }
}

impl Grid for PolarGrid {
    fn new_cell(&self, row: usize, column: usize) -> ICellStrong {
        PolarCell::new(0,0)
    }

    fn prepare_grid(&mut self) {
        self.cells = vec![vec![]; self.rows];
        self.cells[0].push(Some(PolarCell::new(0,0)));
        let row_height = 1.0 / self.rows as f64;

        for row in 1..self.rows {
            let radius = row as f64 / self.rows as f64;
            let circumference = 2.0 * PI * radius;
            let prev_count = self.cells[row - 1].len();
            let est_cell_width = circumference / prev_count as f64;
            let ratio = round::ceil(est_cell_width / row_height, 0) as usize;
            let cells = prev_count * ratio;

            let mut ring: Vec<Option<PolarCellLinkStrong>> = vec![];
            for col in 0..cells {
                ring.push(Some(PolarCell::new(row,col)));
            }
            self.cells[row] = ring;
        }

    }

    fn random_cell(&self, rng: &dyn RngWrapper) -> Option<ICellStrong> {
        let row: usize = rng.gen_range(0, self.rows);
        let col: usize = rng.gen_range(0, self.cells[row].len());
        self.get_cell(row, col)
    }

    fn each_cell(&self) -> Vec<Option<ICellStrong>> {
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

    fn rows(&self) -> usize {
        self.rows
    }

    fn columns(&self) -> usize {
        self.columns
    }

    fn cells(&self) -> &Vec<Vec<Option<ICellStrong>>> {
        self.cells_.as_ref().unwrap()
    }
    
    fn get_cell(&self, row: usize, column: usize) -> Option<ICellStrong> {        
        if row >= self.rows {
            return None;
        }
        if let Some(cell) = self.cells[row][column % self.cells[row].len()].clone() {
            return Some(Rc::clone(&cell) as ICellStrong);
        }
        web_sys::console::log_1(&JsValue::from_str(&format!("get ICellStrong")));
        None
    }


    fn to_string(&self, contents: &dyn CellFormatter) -> String {
        // Not sure how to make this a text string...
        "".to_string()
    }

    fn size(&self) -> usize {
        self.rows * self.columns
    }

    fn to_web(&self, document: &Document, grid_container: &Element, formatter: &dyn CellFormatter, colorize: bool) {
        let cell_size = 20;

        let img_size = 2 * self.rows * cell_size;
        cleanup_old_canvas();
        let context = setup_canvas().unwrap();
        context.set_fill_style(&JsValue::from_str("black"));
        context.set_stroke_style(&JsValue::from_str("black"));

        let center = img_size / 2;

        for (i, cell) in self.each_polar_cell().iter().enumerate() {
            if let Some(cell) = cell {
                let cell = cell.as_ref().borrow();
                let theta = 2.0 * PI / self.cells[cell.row].len() as f64;
                let inner_radius = (cell.row * cell_size) as f64;
                let outer_radius = ((cell.row + 1) * cell_size) as f64;
                let theta_ccw = cell.column as f64 * theta;
                let theta_cw = (cell.column + 1) as f64 * theta;

                let ax = (center as f64 + (inner_radius * theta_ccw.cos()) ) as f64;
                let ay = (center as f64 + (inner_radius * theta_ccw.sin()) ) as f64;
                let bx = (center as f64 + (outer_radius * theta_ccw.cos()) ) as f64;
                let by = (center as f64 + (outer_radius * theta_ccw.sin()) ) as f64;
                let cx = (center as f64 + (inner_radius * theta_cw.cos()) ) as f64;
                let cy = (center as f64 + (inner_radius * theta_cw.sin()) ) as f64;
                let dx = (center as f64 + (outer_radius * theta_cw.cos()) ) as f64;
                let dy = (center as f64 + (outer_radius * theta_cw.sin()) ) as f64;

                // web_sys::console::log_1(&JsValue::from_str(
                //     &format!("cr: {} {}, t: {}, a: {} {}, b: {} {}, c: {} {}, d: {} {}", cell.row, cell.column, theta, ax, ay, bx, by, cx, cy, dx, dy)));
                
                if cell.cw.is_none() || (cell.cw.is_some() && !cell.is_linked(cell.cw.as_ref().unwrap().upgrade().unwrap().clone())) {
                    context.begin_path();
                    context.move_to(cx, cy);
                    context.line_to(dx, dy);
                    context.stroke();
                }

                // web_sys::console::log_1(&JsValue::from_str(
                //     &format!("cr: {} {}, t: {} {} {}", cell.row, cell.column, theta, theta_ccw, theta_cw)));
                if cell.inward.is_none() || (cell.inward.is_some() && !cell.is_linked(cell.inward.as_ref().unwrap().upgrade().unwrap().clone())) {
                    context.begin_path();
                    context.arc(center as f64, center as f64, inner_radius, theta_ccw, theta_cw).unwrap();
                    context.stroke();
                }
            }
        }
        
        context.begin_path();
        context.arc(center as f64, center as f64, (self.rows() * cell_size) as f64, 0.0, PI * 2.0).unwrap();
        context.stroke();
    }

}

pub fn grid_to_web_polar(grid: &dyn Grid, formatter: &dyn CellFormatter, colorize: bool) {
    let document = web_sys::window().unwrap().document().unwrap();
    let grid_container = document.create_element("div").unwrap();
    grid.to_web(&document, &grid_container, formatter, colorize);
}

fn setup_canvas() -> Result<CanvasRenderingContext2d, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = Node::from(document.body().unwrap());
    let canvas_container = document.create_element("div").unwrap();
    body.append_child(&canvas_container)?;
    let canvas = document.create_element("canvas").unwrap();
    canvas_container.append_child(&canvas).unwrap();    
    
    canvas.set_attribute("height", "600px").unwrap();
    canvas.set_attribute("width", "600px").unwrap();
    canvas.set_attribute("id", "polar_grid").unwrap();

    {
        let canvas_html = canvas.dyn_ref::<HtmlElement>().unwrap();
        canvas_html.style().set_property("background-color", "rgb(239, 239, 239)").unwrap();
        canvas_html.style().set_property("outline", "1px solid black").unwrap();
    }

    let canvas = canvas.dyn_into::<HtmlCanvasElement>().ok().unwrap();

    let context = canvas.get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    Ok(context)
}

fn cleanup_old_canvas() {
    let document = web_sys::window().unwrap().document().unwrap();
    let old_canvas = document.get_element_by_id("polar_grid");
    if let Some(old_canvas) = old_canvas {
        old_canvas.remove();
    }
}

