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

    // pub fn to_string(&self, contents: &dyn CellFormatter) -> String {
    //     let mut output = String::new();
    //     output += "\r";

    //     for (i, row) in self.cells.iter().enumerate() {
    //         let mut top = String::from("|");
    //         let mut bottom = String::from("+");
    //         for (j, cell) in row.iter().enumerate() {
    //             if let Some(cell) = cell {
    //                 if i == 0 {
    //                     output += "+---";
    //                     if j == self.columns -1 {
    //                         output += "+";
    //                     }
    //                 }                    

    //                 // let body = format!(" {} ", contents.contents_of(&cell));
    //                 let body = format!("   ");
    //                 let e = cell.borrow();
    //                 let east = e.east_i.as_ref();
    //                 let east_border = if east.is_some() && cell.borrow().is_linked_i(*east.unwrap()) {
    //                     " "
    //                 }
    //                 else {
    //                     "|"
    //                 };

    //                 top += &body;
    //                 top += east_border;

    //                 let south = e.south.as_ref();
    //                 let south_border = if south.is_some() && cell.borrow().is_linked(Rc::clone(&south.unwrap().upgrade().unwrap())) {
    //                     "   "
    //                 }
    //                 else {
    //                     "---"
    //                 };
    //                 let corner = String::from("+");
    //                 bottom += south_border;
    //                 bottom += &corner;
    //             } else {
    //                 if i == 0 {
    //                     // very top of grid
    //                     if j > 0 && self.cells[i][j-1].is_some() {
    //                         output += "+   ";
    //                     } else {
    //                         output += "    ";
    //                     }                 
    //                 }

    //                 if j == 0 {
    //                     top = String::from(" ");
    //                 }                    
                    
    //                 // Handle sides 
    //                 if j == self.columns - 1 || (j > 0 && self.cells[i][j + 1].is_none()) {
    //                     top += "    ";
    //                 }
    //                 else {
    //                     top += "   |"; 
    //                 }

    //                 if j == 0 && (i == self. rows- 1 || (i < self.rows - 1 && self.cells[i+1][j].is_none())) {
    //                     bottom = String::from(" ");
    //                 }

    //                 // Here we need to check the next row/column and see if we need to render the bottom
    //                 // 1) Check if last row OR if before the last row and the cell below it is None
    //                 if i == self.rows -1 || i < self.rows - 1 && self.cells[i+1][j].is_none() {
    //                     // 2) If we aren't in the last column
    //                     if j < self.columns - 1 {
    //                         // 3) Check if the next cell to the right or the cell to the south east is present and render the corner
    //                         if self.cells[i][j+1].is_some() || self.cells[i+1][j+1].is_some() {
    //                             bottom += "   +";                                    
    //                         } else {
    //                             bottom += "    ";    
    //                         }

    //                     }
    //                     else {
    //                         bottom += "    ";
    //                     }
    //                 } else {
    //                     bottom += "---+";
    //                 }
    //             }
    //         }

    //         if i == 0 {
    //             output += "\r\n";
    //         }

    //         output += &format!("{}\r\n", &top);
    //         output += &format!("{}\r\n", &bottom);

    //     }
        
    //     output
    // }

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

    pub fn configure_cells_i(&mut self) {
        for row in &mut self.cells.iter() {
            for cell in &mut row.iter() {
                if let Some(cell) = cell {
                    // can't subtract from a usize of 0 apparently
                    let cell_row = cell.borrow().row;
                    if cell_row > 0 {
                        let north = self.get_cell(cell_row - 1, cell.borrow().column);
                        if north.is_some() {
                            cell.borrow_mut().north_i = Some(north.unwrap().borrow().index);
                        }
                    }

                    let south = self.get_cell(cell.borrow().row + 1, cell.borrow().column);
                    if south.is_some() {
                        cell.borrow_mut().south_i = Some(south.unwrap().borrow().index);
                    }

                    let east = self.get_cell(cell.borrow().row, cell.borrow().column + 1);
                    if east.is_some() {
                        cell.borrow_mut().east_i = Some(east.unwrap().borrow().index);
                    }

                    let cell_column = cell.borrow().column;
                    if cell_column > 0 {
                        let west = self.get_cell(cell.borrow().row, cell_column - 1);
                        if west.is_some() {
                            cell.borrow_mut().west_i = Some(west.unwrap().borrow().index);
                        }
                    }
                }
            }
        }
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
}