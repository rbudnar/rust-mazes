use crate::grid::polar_cell::PolarCellLinkStrong;
use crate::grid::CellFormatter;
use crate::rng::RngWrapper;
use crate::grid::polar_cell::PolarCell;
use crate::grid::Grid;
use std::rc::Rc;
use std::cell::RefCell;
use std::f64::consts::PI;
use web_sys::*;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::JsCast;
use math::round;


pub struct PolarGrid {
    pub cells: Vec<Vec<Option<PolarCellLinkStrong>>>,
    pub rows: usize, 
    pub columns: usize
}

impl PolarGrid {
    pub fn new(rows: usize, columns: usize) -> PolarGrid {
        let mut grid = PolarGrid {
            cells: Vec::new(),
            rows, columns
        };
        grid.prepare_grid();
        // grid.configure_cells();

        grid        
    }
}

impl Grid for PolarGrid {
    fn prepare_grid(&mut self) {
        let mut rows: Vec<Vec<PolarCellLinkStrong>> = vec![];
        rows.push(vec![Rc::new(RefCell::new(PolarCell::new(0,0)))]);

        let row_height = 1.0 / self.rows as f64;

        for row in 1..self.rows {
            let radius = row as f64 / self.rows as f64;
            let circumference = 2.0 * PI * radius;

            let prev_count = rows[row - 1].len();
            let est_cell_width = circumference / prev_count as f64;
            let ratio = round::ceil(est_cell_width / row_height, 0) as usize; // Not sure if this is the right rounding mechanism
            let cells = prev_count * ratio;
            let mut ring: Vec<Option<PolarCellLinkStrong>> = vec![];
            for col in 0..cells {
                ring.push(Some(Rc::new(RefCell::new(PolarCell::new(row,col)))));
            }
            self.cells.push(ring);
        }
    }

    fn random_cell(&self, rng: &dyn RngWrapper) -> Option<PolarCellLinkStrong> {
        let row: usize = rng.gen_range(0, self.rows);
        let col: usize = rng.gen_range(0, self.columns);
        self.get_cell(row, col)
    }

    fn each_cell(&self) -> Vec<Option<PolarCellLinkStrong>> {
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

    fn rows(&self) -> usize {
        self.columns
    }

    fn columns(&self) -> usize {
        self.rows
    }

    fn cells(&self) -> &Vec<Vec<Option<PolarCellLinkStrong>>> {
        &self.cells
    }
    
    fn get_cell(&self, row: usize, column: usize) -> Option<PolarCellLinkStrong> {
        self.get_cell(row, column)
    }

    fn to_string(&self, contents: &dyn CellFormatter) -> String {
        self.to_string(contents)
    }

    fn size(&self) -> usize {
        self.rows * self.columns
    }
}



pub fn grid_to_web_polar(grid: &Grid) {
    let cell_size = 20;

    let img_size = 2 * grid.rows() * cell_size;
    cleanup_old_canvas();
    let context = setup_canvas().unwrap();
    context.set_fill_style(&JsValue::from_str("black"));
    context.set_stroke_style(&JsValue::from_str("black"));

    let center = img_size / 2;

    for (i, cell) in grid.each_cell().iter().enumerate() {
        if let Some(cell) = cell {
            let cell = cell.as_ref().borrow();
            let theta = 2.0 * PI / grid.cells()[cell.row].len() as f64;
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
            // if cell.north.is_none() || (cell.north.is_some() && !cell.is_linked(cell.north.as_ref().unwrap().upgrade().unwrap().clone())) {
            //     context.begin_path();
            //     context.move_to(ax, ay);
            //     context.line_to(cx, cy);
            //     context.stroke();
            // }

            if cell.east.is_none() || (cell.east.is_some() && !cell.is_linked(cell.east.as_ref().unwrap().upgrade().unwrap().clone())) {
                context.begin_path();
                context.move_to(cx, cy);
                context.line_to(dx, dy);
                context.stroke();
            }

            web_sys::console::log_1(&JsValue::from_str(
                &format!("cr: {} {}, t: {} {} {}", cell.row, cell.column, theta, theta_ccw, theta_cw)));
            if cell.north.is_none() || (cell.north.is_some() && !cell.is_linked(cell.north.as_ref().unwrap().upgrade().unwrap().clone())) {
                context.begin_path();
                context.arc(center as f64, center as f64, inner_radius, theta_ccw, theta_cw).unwrap();
                context.stroke();
            }
        }
    }
    
    context.begin_path();
    context.arc(center as f64, center as f64, (grid.rows() * cell_size) as f64, 0.0, PI * 2.0).unwrap();
    context.stroke();
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

