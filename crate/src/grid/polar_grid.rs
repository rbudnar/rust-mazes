use std::rc::Rc;
use std::f64::consts::PI;
use wasm_bindgen::prelude::JsValue;
use math::round;
use crate::grid::{Grid, CellFormatter, canvas::{setup_grid_canvas, remove_old_canvas, draw_line, DrawMode, set_canvas_size}};
use crate::cells::{ICellStrong, polar_cell::{PolarCellLinkStrong, PolarCell}};
use crate::rng::RngWrapper;

pub static POLAR_GRID: &str = "polar_grid";

pub struct PolarGrid {
    pub cells: Vec<Vec<Option<PolarCellLinkStrong>>>,
    pub rows: usize, 
    pub columns: usize,
    pub _cells: Option<Vec<Vec<Option<ICellStrong>>>>
}

impl PolarGrid {
    pub fn new(rows: usize, columns: usize) -> PolarGrid {
        let mut grid = PolarGrid {
            cells: Vec::new(),
            rows, columns,
            _cells: None
        };
        grid.prepare_grid();
        grid.configure_cells();
        grid.create_cells();

        grid        
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
        PolarCell::new(row, column)
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

    fn random_cell(&self, rng: &dyn RngWrapper<Shuffle=ICellStrong>) -> Option<ICellStrong> {
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
        self._cells.as_ref().unwrap()
    }
    
    fn get_cell(&self, row: usize, column: usize) -> Option<ICellStrong> {        
        if row >= self.rows {
            return None;
        }
        if let Some(cell) = self.cells[row][column % self.cells[row].len()].clone() {
            return Some(Rc::clone(&cell) as ICellStrong);
        }
        None
    }

    fn to_string(&self, _contents: &dyn CellFormatter) -> String {
        // Not sure how to make this a text string...
        "".to_string()
    }

    fn size(&self) -> usize {
        self.cells.iter().fold(0, |acc, r| acc + r.len())
    }

    fn braid(&self, p: f64, rng: &dyn RngWrapper<Shuffle=ICellStrong>) {
        // TODO: not yet implemented
    }

    fn to_web(&self, formatter: &dyn CellFormatter, colorize: bool, inset: f64) {
        let size = 20;
        let img_size = 2 * self.rows * size;

        remove_old_canvas(POLAR_GRID);
        let context = setup_grid_canvas(POLAR_GRID).unwrap();
        set_canvas_size(POLAR_GRID, img_size, img_size);
        
        context.set_fill_style(&JsValue::from_str("black"));
        context.set_stroke_style(&JsValue::from_str("black"));

        let center = img_size / 2;

        for mode in [DrawMode::Background, DrawMode::Line].iter() {
            for cell in self.each_polar_cell().iter() {
                if let Some(cell) = cell {
                    let c = cell.as_ref().borrow();
                    

                    let theta = 2.0 * PI / self.cells[c.row].len() as f64;
                    let inner_radius = (c.row * size) as f64;
                    let outer_radius = ((c.row + 1) * size) as f64;
                    let theta_ccw = c.column as f64 * theta;
                    let theta_cw = (c.column + 1) as f64 * theta;

                    // let ax = (center as f64 + (inner_radius * theta_ccw.cos()) ) as f64;
                    // let ay = (center as f64 + (inner_radius * theta_ccw.sin()) ) as f64;
                    // let bx = (center as f64 + (outer_radius * theta_ccw.cos()) ) as f64;
                    // let by = (center as f64 + (outer_radius * theta_ccw.sin()) ) as f64;
                    let cx = (center as f64 + (inner_radius * theta_cw.cos()) ) as f64;
                    let cy = (center as f64 + (inner_radius * theta_cw.sin()) ) as f64;
                    let dx = (center as f64 + (outer_radius * theta_cw.cos()) ) as f64;
                    let dy = (center as f64 + (outer_radius * theta_cw.sin()) ) as f64;

                    // web_sys::console::log_1(&JsValue::from_str(
                    //     &format!("cr: {} {}, t: {}, a: {} {}, b: {} {}, c: {} {}, d: {} {}", cell.row, cell.column, theta, ax, ay, bx, by, cx, cy, dx, dy)));

                    match mode {
                        DrawMode::Background => {
                            if colorize {
                                // let points = vec![(ax, ay), (bx, by), (cx, cy), (dx, dy)];
                                let ics: ICellStrong =  Rc::clone(cell) as ICellStrong;
                                let color = formatter.background_color(&ics).to_string();
 
                                context.set_fill_style(&JsValue::from_str(&color));
                                context.set_stroke_style(&JsValue::from_str(&color));
                                
                                context.begin_path();
                                // arc(x, y, radius, startAngle, endAngle, anticlockwise)
                                context.arc(center as f64, center as f64, inner_radius, theta_ccw, theta_cw).unwrap();
                                context.arc_with_anticlockwise(center as f64, center as f64, outer_radius, theta_cw, theta_ccw, true).unwrap();

                                context.fill();
                                context.stroke();
                                context.set_fill_style(&JsValue::from_str("black"));
                                context.set_stroke_style(&JsValue::from_str("black"));
                            }
                        },
                        DrawMode::Line => {
                            if c.row == 0 {
                                // Hide the line in the middle. Not really needed but makes it a little prettier.
                                continue; 
                            }
                            if c.cw.is_none() || (c.cw.is_some() && !c.is_linked(c.cw.as_ref().unwrap().upgrade().unwrap().clone())) {
                                draw_line(&context, cx, cy, dx, dy);
                            }
                            
                            if c.inward.is_none() || (c.inward.is_some() && !c.is_linked(c.inward.as_ref().unwrap().upgrade().unwrap().clone())) {
                                context.begin_path();
                                context.arc(center as f64, center as f64, inner_radius, theta_ccw, theta_cw).unwrap();
                                context.stroke();
                            }                                     
                        }
                    }                
                    
                }
            }
        }
        
        context.begin_path();
        context.arc(center as f64, center as f64, (self.rows() * size) as f64, 0.0, PI * 2.0).unwrap();
        context.stroke();
    }
}