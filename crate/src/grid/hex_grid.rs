use std::rc::{Rc};
use wasm_bindgen::prelude::JsValue;
use super::{Grid, CellFormatter, canvas::{remove_old_canvas, setup_grid_canvas, draw_line, DrawMode, draw_shape, set_canvas_size}};
use crate::rng::RngWrapper;
use crate::cells::{ICellStrong, hex_cell::{HexCellStrong, HexCellWeak, HexCell}};

pub static HEX_GRID: &str = "hex_grid";

pub struct HexGrid {
    pub cells: Vec<Vec<Option<HexCellStrong>>>,
    pub rows: usize, 
    pub columns: usize,
    _cells: Option<Vec<Vec<Option<ICellStrong>>>>
}

impl Grid for HexGrid {
    fn new_cell(&self, row: usize, column: usize) -> ICellStrong {
        HexCell::new(row, column) as ICellStrong
    }

    fn prepare_grid(&mut self) {
        for i in 0..self.rows {
            let mut row: Vec<Option<HexCellStrong>> = Vec::new();
            
            for j in 0..self.columns {
                row.push(Some(HexCell::new(i as usize, j as usize)));
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

    fn to_string(&self, _contents: &dyn CellFormatter) -> String {
        "".to_string()
    }

    fn size(&self) -> usize {
        self.rows * self.columns
    }

    fn to_web(&self, formatter: &dyn CellFormatter, colorize: bool) {
        let size = 20;

        let size = f64::from(size);
        
        let a_size = size / 2_f64;
        let b_size = size * 3_f64.sqrt() / 2_f64;
        // let width = size * 2_f64;
        let height = b_size * 2_f64;

        let img_width = (3_f64 * a_size * (self.columns as f64) + a_size + 0.5_f64).trunc() as usize;
        let img_height = (height * (self.rows as f64) + b_size + 0.5_f64).trunc() as usize;
        
        remove_old_canvas(HEX_GRID);
        let context = setup_grid_canvas(HEX_GRID).unwrap();
        set_canvas_size(HEX_GRID, img_width, img_height);
        context.set_fill_style(&JsValue::from_str("black"));
        context.set_stroke_style(&JsValue::from_str("black"));

        for mode in [DrawMode::Background, DrawMode::Line].iter() {
            for cell in self.each_hex_cell().iter() {
                if let Some(cell) = cell {
                    let cx = size + 3_f64 * (cell.borrow().column as f64) * a_size;
                    let mut cy = b_size + (cell.borrow().row as f64) * height;
                    if cell.borrow().column % 2 != 0 {
                        cy += b_size;
                    }

                    // f/n = far/near
                    // n/s/e/w = north/south/east/west
                    // m = middle
                    let x_fw = (cx - size).trunc();
                    let x_nw = (cx - a_size).trunc();
                    let x_ne = (cx + a_size).trunc();
                    let x_fe = (cx + size).trunc();

                    let y_n = (cy - b_size).trunc();
                    let y_m = cy.trunc();
                    let y_s = (cy + b_size).trunc();

                    match mode {
                        DrawMode::Background => {
                            if colorize {
                                let points = vec![(x_fw, y_m), (x_nw, y_n), (x_ne, y_n), (x_fe, y_m), (x_ne, y_s), (x_nw, y_s)];
                                let ics: ICellStrong =  Rc::clone(cell) as ICellStrong;
                                let color = formatter.background_color(&ics);
                                draw_shape(&context, points, &color);
                            }
                        },
                        DrawMode::Line => {
                            if cell.borrow().southwest.is_none() {
                                draw_line(&context, x_fw, y_m, x_nw, y_s);
                            }

                            if cell.borrow().northwest.is_none() {
                                draw_line(&context, x_fw, y_m, x_nw, y_n);
                            }

                            if cell.borrow().north.is_none() {
                                draw_line(&context, x_nw, y_n, x_ne, y_n);
                            }

                            if is_not_linked(cell, &cell.borrow().northeast) {
                                draw_line(&context, x_ne, y_n, x_fe, y_m);
                            }
                            
                            if is_not_linked(cell, &cell.borrow().southeast) {
                                draw_line(&context, x_fe, y_m, x_ne, y_s);
                            }

                            if is_not_linked(cell, &cell.borrow().south) {
                                draw_line(&context, x_ne, y_s, x_nw, y_s);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn is_not_linked(cell: &HexCellStrong, other: &Option<HexCellWeak>) -> bool {
    if let Some(other) = other.clone() {
        let other = other.upgrade();
        if !cell.borrow().is_linked(other.unwrap()) {
            return true;
        }
    } else {
        return true;
    }    
    false
}

impl HexGrid {
    pub fn new(rows: usize, columns: usize) -> HexGrid {
        let mut grid = HexGrid {
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
        for cell in self.each_hex_cell().iter() {
            if let Some(cell) = cell {
                let row = cell.borrow().row;
                let col = cell.borrow().column;
                
                let (north_diagonal, south_diagonal) = if col % 2 == 0 {
                    ((row as i32) - 1, row)
                } else {
                    ((row as i32), row + 1)
                };

                if north_diagonal >= 0 {
                    let north_diagonal = north_diagonal as usize;
                    if col > 0 {
                        if let Some(nw) = self.cells[north_diagonal][col - 1].clone() {
                            cell.borrow_mut().northwest = Some(Rc::downgrade(&nw));
                        }
                    }

                    if col < self.columns - 1 {
                        if let Some(ne) = self.cells[north_diagonal][col + 1].clone() {
                            cell.borrow_mut().northeast = Some(Rc::downgrade(&ne));
                        }
                    }
                }

                if row > 0 {
                    if let Some(north) = self.cells[row - 1][col].clone() {
                        cell.borrow_mut().north = Some(Rc::downgrade(&north));
                    }
                }
                
                if row < self.rows  -1 {
                    if let Some(south) = self.cells[row + 1][col].clone() {
                        cell.borrow_mut().south = Some(Rc::downgrade(&south));
                    }
                }

                if col > 0 && south_diagonal < self.rows {
                    if let Some(sw) = self.cells[south_diagonal][col - 1].clone() {
                        cell.borrow_mut().southwest = Some(Rc::downgrade(&sw));
                    }
                }

                if col < self.columns - 1 && south_diagonal < self.rows {
                    if let Some(se) = self.cells[south_diagonal][col + 1].clone() {
                        cell.borrow_mut().southeast = Some(Rc::downgrade(&se));
                    }            
                }
            }
        }
    }

    fn each_hex_cell(&self) -> Vec<Option<HexCellStrong>> {
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