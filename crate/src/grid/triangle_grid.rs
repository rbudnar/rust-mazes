use crate::grid::canvas::set_canvas_size;
use std::rc::{Rc};
use wasm_bindgen::prelude::JsValue;
use crate::grid::{Grid, CellFormatter, canvas::{remove_old_canvas, setup_canvas, draw_shape, draw_cv_line, DrawMode}};
use crate::rng::RngWrapper;
use crate::cells::{ICellStrong, triangle_cell::{TriangleCellStrong, TriangleCellWeak, TriangleCell}};

pub static TRIANGLE_GRID: &str = "triangle_grid";

pub struct TriangleGrid {
    pub cells: Vec<Vec<Option<TriangleCellStrong>>>,
    pub rows: usize, 
    pub columns: usize,
    _cells: Option<Vec<Vec<Option<ICellStrong>>>>
}

impl Grid for TriangleGrid {
    fn new_cell(&self, row: usize, column: usize) -> ICellStrong {
        TriangleCell::new(row, column) as ICellStrong
    }

    fn prepare_grid(&mut self) {
        for i in 0..self.rows {
            let mut row: Vec<Option<TriangleCellStrong>> = Vec::new();
            
            for j in 0..self.columns {
                row.push(Some(TriangleCell::new(i as usize, j as usize)));
            }
            self.cells.push(row);
        }   
    }

    fn random_cell(&self, rng: &dyn RngWrapper) -> Option<ICellStrong> {
        let row: usize = rng.gen_range(0, self.rows);
        let col: usize = rng.gen_range(0, self.columns);
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

    fn to_web(&self, formatter: &dyn CellFormatter, colorize: bool) {
        let size = 40_f64;
        let half_width = size / 2_f64;
        let height = size * 3_f64.sqrt() / 2_f64;
        let half_height = height / 2_f64;
        let img_width = (size * ((self.columns as f64) + 1_f64) / 2_f64).trunc() as usize;
        let img_height = (height * (self.rows as f64)).trunc() as usize;
        
        remove_old_canvas(TRIANGLE_GRID);
        let context = setup_canvas(TRIANGLE_GRID).unwrap();
        context.set_fill_style(&JsValue::from_str("black"));
        context.set_stroke_style(&JsValue::from_str("black"));
        set_canvas_size(TRIANGLE_GRID, img_width, img_height);

        for mode in [DrawMode::Background, DrawMode::Line].iter() {
            for cell in self.each_triangle_cell().iter() {
                if let Some(cell) = cell {
                    let cx = half_width + (cell.borrow().column as f64) * half_width;
                    let cy = half_height + (cell.borrow().row as f64) * height;
                    
                    let west_x = (cx - half_width).trunc();
                    let mid_x = cx.trunc();
                    let east_x = (cx + half_width).trunc();

                    let (apex_y, base_y) = if cell.borrow().upright() {
                        ((cy - half_height).trunc(), (cy + half_height).trunc())
                    } else {
                        ((cy + half_height).trunc(), (cy - half_height).trunc())
                    };

                    match mode {
                        DrawMode::Background => {
                            if colorize {
                                let points = vec![(west_x, base_y), (mid_x, apex_y), (east_x, base_y)];
                                let ics: ICellStrong =  Rc::clone(cell) as ICellStrong;
                                let color = formatter.background_color(&ics);
                                draw_shape(&context, points, &color);
                            }
                        },
                        DrawMode::Line => {
                            if cell.borrow().west.is_none() {
                                draw_cv_line(&context, west_x, base_y, mid_x, apex_y);
                            }

                            if cell.borrow().is_not_linked(&cell.borrow().east){
                                draw_cv_line(&context, east_x, base_y, mid_x, apex_y);
                            }

                            let no_south = cell.borrow().upright() && cell.borrow().south.is_none();
                            let not_linked = !cell.borrow().upright() && cell.borrow().is_not_linked(&cell.borrow().north);

                            if no_south || not_linked {
                                draw_cv_line(&context, east_x, base_y, west_x, base_y);
                            }                    
                        }
                    }                    
                }
            }
        }
    }
}

fn is_not_linked(cell: &TriangleCellStrong, other: &Option<TriangleCellWeak>) -> bool {
    if let Some(other) = other.clone() {
        let other = other.upgrade();
        if !cell.borrow().is_linked(other.unwrap()) {
            return true;
        }
    } else {
        return true;
    }    
    return false;
}

impl TriangleGrid {
    pub fn new(rows: usize, columns: usize) -> TriangleGrid {
        let mut grid = TriangleGrid {
            cells: Vec::new(),
            rows, columns,
            _cells: None
        };

        grid.prepare_grid();
        grid.configure_cells();
        grid.create_cells();

        grid        
    }

    fn configure_cells(&mut self) {
        for cell in self.each_triangle_cell().iter() {
            if let Some(cell) = cell {
                let row = cell.borrow().row;
                let col = cell.borrow().column;
                
                if col > 0 {
                    if let Some(west) = self.cells[row][col - 1].clone() {
                        cell.borrow_mut().west = Some(Rc::downgrade(&west));
                    }
                }

                if col < self.columns - 1 {
                    if let Some(east) = self.cells[row][col + 1].clone() {
                        cell.borrow_mut().east = Some(Rc::downgrade(&east));
                    }
                }

                if cell.borrow().upright() {
                    if row < self.rows - 1 {
                        if let Some(south) = self.cells[row + 1][col].clone() {
                            cell.borrow_mut().south = Some(Rc::downgrade(&south));
                        }
                    }
                } else {
                    if row > 0 {
                        if let Some(north) = self.cells[row - 1][col].clone() {
                            cell.borrow_mut().north = Some(Rc::downgrade(&north));
                        }
                    }
                }
            }
        }
    }

    fn each_triangle_cell(&self) -> Vec<Option<TriangleCellStrong>> {
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
}