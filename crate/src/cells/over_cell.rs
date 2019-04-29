use wasm_bindgen::{prelude::JsValue};
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::any::Any;

use crate::grid::weave_grid::WeaveGrid;
use crate::GRID;
use crate::grid::Grid;
use crate::cells::{ICellStrong, ICell};

pub type OverCellLinkWeak = Weak<RefCell<OverCell>>;
pub type OverCellLinkStrong = Rc<RefCell<OverCell>>;

#[derive(Debug)]
pub struct OverCell {
    pub row: usize,
    pub column: usize,
    pub links: Vec<Option<OverCellLinkWeak>>,
    pub north: Option<OverCellLinkWeak>,
    pub south: Option<OverCellLinkWeak>,
    pub east: Option<OverCellLinkWeak>,
    pub west: Option<OverCellLinkWeak>,
    pub self_rc: OverCellLinkWeak,
    pub is_under_cell: bool,
    pub grid_ref: Rc::<RefCell<WeaveGrid>>
}

impl PartialEq for OverCell {
    fn eq(&self, other: &OverCell) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl ICell for OverCell {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn row(&self) -> usize {
        self.row
    }
    fn column(&self) -> usize {
        self.column
    }
    
    fn neighbors(&self) -> Vec<ICellStrong> {
        let mut vec: Vec<ICellStrong> = vec![];

        if let Some(ref north) = self.north {
            let north = north.upgrade().unwrap();
            vec.push(north as ICellStrong);
        }

        if let Some(ref south) = self.south {
            let south = south.upgrade().unwrap();
            vec.push(south as ICellStrong);
        }

        if let Some(ref east) = self.east {
            let east = east.upgrade().unwrap();
            vec.push(east as ICellStrong);
        }

        if let Some(ref west) = self.west {
            let west = west.upgrade().unwrap();
            vec.push(west as ICellStrong);
        }

        if self.can_tunnel_north() {
            if let Some(ref north) = self.north {
                let north = north.upgrade().unwrap();
                let nb = north.borrow();
                if let Some(ref n_north) = nb.north {
                    let n_north = n_north.upgrade().unwrap();
                    vec.push(n_north as ICellStrong);
                }
            }
        }

        if self.can_tunnel_south() {
            if let Some(ref south) = self.south {
                let south = south.upgrade().unwrap();
                let nb = south.borrow();
                if let Some(ref n_south) = nb.south {
                    let n_south = n_south.upgrade().unwrap();
                    vec.push(n_south as ICellStrong);
                }
            }
        }

        if self.can_tunnel_east() {
            if let Some(ref east) = self.east {
                let east = east.upgrade().unwrap();
                let nb = east.borrow();
                if let Some(ref n_east) = nb.east {
                    let n_east = n_east.upgrade().unwrap();
                    vec.push(n_east as ICellStrong);
                }
            }
        }

        if self.can_tunnel_west() {
            if let Some(ref west) = self.west {
                let west = west.upgrade().unwrap();
                let nb = west.borrow();
                if let Some(ref n_west) = nb.west {
                    let n_west = n_west.upgrade().unwrap();
                    vec.push(n_west as ICellStrong);
                }
            }
        }

        vec
    }

    fn links(&self) -> Vec<Option<ICellStrong>> {
        self.links.iter()
            .map(|c| 
                Some(c.as_ref().unwrap().upgrade().unwrap() as ICellStrong)
            ).collect()
    }

    fn link(&mut self, other: ICellStrong) {        
        if let Some(nl) = other.borrow().as_any().downcast_ref::<OverCell>() {
            if self.is_under_cell {
                let _other: OverCellLinkWeak = Rc::downgrade(&Rc::clone(&nl.self_rc.upgrade().unwrap()));
                self.links.push(Some(_other));
                return;
            }

            let other: OverCellLinkStrong = nl.self_rc.upgrade().unwrap();
            let north = self.get_neighbor(&self.north);
            let south = self.get_neighbor(&self.south);
            let east = self.get_neighbor(&self.east);
            let west = self.get_neighbor(&self.west);

            // TODO: Fix already borrowed mut error
            let (other_north, other_south, other_east, other_west) = {
                let ob = Rc::clone(&other);
                let ob = ob.borrow();

                // let other_north = ob.get_neighbor(&ob.north);
                // let other_south = ob.get_neighbor(&ob.south);
                // let other_east = ob.get_neighbor(&ob.east);
                // let other_west = ob.get_neighbor(&ob.west);

                let other_north =  if let Some(o) = ob.north.clone() {
                    o.upgrade()
                } else { None };
                let other_south = if let Some(o) = ob.south.clone() {
                    o.upgrade()
                } else { None };
                let other_east = if let Some(o) = ob.east.clone() {
                    o.upgrade()
                } else { None };
                let other_west =if let Some(o) = ob.west.clone() {
                    o.upgrade()
                } else { None };
                (other_north, other_south, other_east, other_west)
            };

            web_sys::console::log_1(&JsValue::from_str("1"));
            let (nr, nc) = if let Some(n) = north.as_ref() {
                (Some(n.borrow().row()), Some(n.borrow().column()))
            } else {
                (None, None)
            };

            web_sys::console::log_1(&JsValue::from_str("2"));
            let (sr, sc) = if let Some(n) = south.as_ref() {
                (Some(n.borrow().row()), Some(n.borrow().column()))
            } else {
                (None, None)
            };
            
            web_sys::console::log_1(&JsValue::from_str("3"));
            let (er, ec) = if let Some(n) = east.as_ref() {
                (Some(n.borrow().row()), Some(n.borrow().column()))
            } else {
                (None, None)
            };

            web_sys::console::log_1(&JsValue::from_str("4"));
            let (wr, wc) = if let Some(n) = west.as_ref() {
                (Some(n.borrow().row()), Some(n.borrow().column()))
            } else {
                (None, None)
            };
            web_sys::console::log_1(&JsValue::from_str("5"));

            let (onr, onc) = if let Some(n) = other_north.clone().as_ref() {
                web_sys::console::log_1(&JsValue::from_str("5a"));
                (Some(n.borrow().row()), Some(n.borrow().column()))
            } else {
                (None, None)
            };

            web_sys::console::log_1(&JsValue::from_str("6"));
            let (osr, osc) = if let Some(n) = other_south.clone().as_ref() {
                web_sys::console::log_1(&JsValue::from_str("6a"));
                (Some(n.borrow().row()), Some(n.borrow().column()))
            } else {
                (None, None)
            };
            
            web_sys::console::log_1(&JsValue::from_str("7"));
            let (oer, oec) = if let Some(n) = other_east.clone().as_ref() {
                web_sys::console::log_1(&JsValue::from_str("7a"));
                (Some(n.borrow().row()), Some(n.borrow().column()))
            } else {
                (None, None)
            };

            web_sys::console::log_1(&JsValue::from_str("8"));
            let (owr, owc) = if let Some(n) = other_west.clone().as_ref() {
                web_sys::console::log_1(&JsValue::from_str("8a"));
                (Some(n.borrow().row()), Some(n.borrow().column()))
            } else {
                (None, None)
            };
            web_sys::console::log_1(&JsValue::from_str("9"));

            
            let neighbor = if nr.is_some() && osr.is_some() && nr.unwrap() == osr.unwrap() && nc.unwrap() == osc.unwrap() {
                web_sys::console::log_1(&JsValue::from_str("10"));
                north.as_ref()
            } else if sr.is_some() && onr.is_some() && sr.unwrap() == onr.unwrap() && sc.unwrap() == onc.unwrap()  {
                web_sys::console::log_1(&JsValue::from_str("11"));
                south.as_ref()
            } else if er.is_some() && owr.is_some() && er.unwrap() == owr.unwrap() && ec.unwrap() == owc.unwrap() {
            // cells_are_same(&east, &other_west) {
                web_sys::console::log_1(&JsValue::from_str("12"));
                east.as_ref()
            } else if wr.is_some() && owr.is_some() && wr.unwrap() == oer.unwrap() && wc.unwrap() == oec.unwrap() {
            //cells_are_same(&west, &other_east) {
                web_sys::console::log_1(&JsValue::from_str("13"));
                west.as_ref()
            } else {
                web_sys::console::log_1(&JsValue::from_str("14"));
                None
            };


            // let other_north = other.borrow().get_neighbor(&other.borrow().north);
            // let other_south = other.borrow().get_neighbor(&other.borrow().south);
            // let other_east = other.borrow().get_neighbor(&other.borrow().east);
            // let other_west = other.borrow().get_neighbor(&other.borrow().west);

            // web_sys::console::log_1(&JsValue::from_str("about to nr"));
            // let nes = if let (Some(first), Some(second)) = (north.as_ref(), other_south.as_ref()) {
            //     first.borrow().row() == second.borrow().row() && first.borrow().column() == second.borrow().column()
            // } else {
            //     false
            // };

            // web_sys::console::log_1(&JsValue::from_str("1"));
            // let sen = if let (Some(first), Some(second)) = (south.as_ref(), other_north.as_ref()) {
            //     first.borrow().row() == second.borrow().row() && first.borrow().column() == second.borrow().column()
            // } else {
            //     false
            // };
            // web_sys::console::log_1(&JsValue::from_str("2"));
            // let eew = if let (Some(first), Some(second)) = (east.as_ref(), other_west.as_ref()) {
            //     first.borrow().row() == second.borrow().row() && first.borrow().column() == second.borrow().column()
            // } else {
            //     false
            // };

            // web_sys::console::log_1(&JsValue::from_str("3"));

            // let wee = if let (Some(first), Some(second)) = (west.as_ref(), other_east.as_ref()) {
            //     first.borrow().row() == second.borrow().row() && first.borrow().column() == second.borrow().column()
            // } else {
            //     false
            // };

            // web_sys::console::log_1(&JsValue::from_str("nr"));
            
            // let neighbor = if nes {
            //     web_sys::console::log_1(&JsValue::from_str("1"));
            //     north
            // } else if sen {
            //     web_sys::console::log_1(&JsValue::from_str("2"));
            //     south
            // } else if eew {
            //     web_sys::console::log_1(&JsValue::from_str("3"));
            //     east
            // } else if wee {
            //     web_sys::console::log_1(&JsValue::from_str("4"));
            //     west 
            // } else {
            //     web_sys::console::log_1(&JsValue::from_str("5"));
            //     None
            // };
            // let neighbor = if cells_are_same(&north, &other_south) {
            //     web_sys::console::log_1(&JsValue::from_str("1"));
            //     north.as_ref()
            // } else if cells_are_same(&south, &other_north) {
            //     web_sys::console::log_1(&JsValue::from_str("2"));
            //     south.as_ref()
            // } else if cells_are_same(&east, &other_west) {
            //     web_sys::console::log_1(&JsValue::from_str("3"));
            //     east.as_ref()
            // } else if cells_are_same(&west, &other_east) {
            //     web_sys::console::log_1(&JsValue::from_str("4"));
            //     west.as_ref()
            // } else {
            //     web_sys::console::log_1(&JsValue::from_str("5"));
            //     None
            // };

            web_sys::console::log_1(&JsValue::from_str("assigning"));
            if let Some(neighbor) = neighbor {
                self.grid_ref.borrow_mut().tunnel_under(&neighbor);
            } else {
                let _other: OverCellLinkWeak = Rc::downgrade(&Rc::clone(&nl.self_rc.upgrade().unwrap()));
                self.links.push(Some(_other));
            }
        }
    }

    
}

impl OverCell {
    pub fn new(row: usize, column: usize, over_cell: Option<OverCellLinkStrong>, grid_ref: Rc<RefCell<WeaveGrid>>) -> OverCellLinkStrong {
        let (row, column, is_under_cell) = if let Some(over_cell) = &over_cell {
            (over_cell.borrow().row(), over_cell.borrow().column(), true)
        } else {
            (row, column, false)
        };

        let c = OverCell {
            row, 
            column, 
            north: None, 
            south: None, 
            east: None, 
            west: None, 
            links: Vec::new(), 
            self_rc: Weak::new(),
            is_under_cell,
            grid_ref
        };

        let rc = Rc::new(RefCell::new(c));
        rc.borrow_mut().self_rc = Rc::downgrade(&rc);

        if let Some(over_cell) = over_cell {
            if over_cell.borrow().horizontal_passage() {
                if let Some(oc_north) = over_cell.borrow().get_neighbor(&over_cell.borrow().north) {
                    rc.borrow_mut().north = Some(Rc::downgrade(&oc_north));
                    oc_north.borrow_mut().south = Some(Rc::downgrade(&rc));
                    
                    // link(north)
                    rc.borrow_mut().link(oc_north.clone());
                    oc_north.borrow_mut().link(rc.clone());
                }
                if let Some(oc_south) = over_cell.borrow().get_neighbor(&over_cell.borrow().south) {
                    rc.borrow_mut().south = Some(Rc::downgrade(&oc_south));
                    oc_south.borrow_mut().north = Some(Rc::downgrade(&rc));

                    // link(south)
                    rc.borrow_mut().link(oc_south.clone());
                    oc_south.borrow_mut().link(rc.clone());
                }
            } else {
                if let Some(oc_east) = over_cell.borrow().get_neighbor(&over_cell.borrow().east) {
                    rc.borrow_mut().east = Some(Rc::downgrade(&oc_east));
                    oc_east.borrow_mut().west = Some(Rc::downgrade(&rc));
                    
                    // link(east)
                    rc.borrow_mut().link(oc_east.clone());
                    oc_east.borrow_mut().link(rc.clone());
                }
                if let Some(oc_west) = over_cell.borrow().get_neighbor(&over_cell.borrow().west) {
                    rc.borrow_mut().west = Some(Rc::downgrade(&oc_west));
                    oc_west.borrow_mut().east = Some(Rc::downgrade(&rc));

                    // link(west)
                    rc.borrow_mut().link(oc_west.clone());
                    oc_west.borrow_mut().link(rc.clone());
                }
            }   
        }

        rc
    }

    pub fn is_linked(&self, other: OverCellLinkStrong) -> bool {
        self.index_of_other(Rc::clone(&other)).is_some()        
    }

    pub fn index_of_other(&self, other: OverCellLinkStrong) -> Option<usize> {
        let other_row: usize = other.borrow().row;
        let other_col: usize = other.borrow().column;
        self.links.iter().position(|ref s| {
            if let Some(st) = s {
                let strong : OverCellLinkStrong = st.upgrade().unwrap();
                let c = strong.borrow();
                c.row == other_row && c.column == other_col
            }
            else {
                false
            }
        })
    }

    pub fn is_not_linked(&self, other: &Option<OverCellLinkWeak>) -> bool {
        if let Some(other) = other.clone() {
            let other = other.upgrade();
            if !self.is_linked(other.unwrap()) {
                return true;
            }
        } else {
            return true;
        }    
        false
    }

    pub fn neighbors_std(&self) -> Vec<OverCellLinkStrong> {
        let mut vec: Vec<OverCellLinkStrong> = vec![];

        if let Some(ref north) = self.north {
            let north = north.upgrade().unwrap();
            vec.push(north);
        }

        if let Some(ref south) = self.south {
            let south = south.upgrade().unwrap();
            vec.push(south);
        }

        if let Some(ref east) = self.east {
            let east = east.upgrade().unwrap();
            vec.push(east);
        }

        if let Some(ref west) = self.west {
            let west = west.upgrade().unwrap();
            vec.push(west);
        }

        vec
    }

    // fn link(&mut self, other: OverCellLinkStrong) {       
    //     let nl = other.borrow();
    //     let _other: OverCellLinkWeak = Rc::downgrade(&Rc::clone(&nl.self_rc.upgrade().unwrap()));
    //     self.links.push(Some(_other));        
    // }

    fn can_tunnel_north(&self) -> bool {
        if let Some(north) = self.get_neighbor(&self.north) {
            north.borrow().north.is_some() && north.borrow().horizontal_passage()
        } else {
            false
        }
    }

    fn can_tunnel_south(&self) -> bool {
        if let Some(south) = self.get_neighbor(&self.south) {
            south.borrow().south.is_some() && south.borrow().horizontal_passage()
        } else {
            false
        }
    }

    fn can_tunnel_east(&self) -> bool {
        if let Some(east) = self.get_neighbor(&self.east) {
            east.borrow().east.is_some() && east.borrow().vertical_passage()
        } else {
            false
        }
    }

    fn can_tunnel_west(&self) -> bool {
        if let Some(west) = self.get_neighbor(&self.west) {
            west.borrow().west.is_some() && west.borrow().vertical_passage()
        } else {
            false
        }
    }

    pub fn horizontal_passage(&self) -> bool {
        if self.is_under_cell {
            return self.east.is_some() || self.west.is_some();
        }

        let east = self.get_neighbor(&self.east);
        let west = self.get_neighbor(&self.west);
        let north = self.get_neighbor(&self.north);
        let south = self.get_neighbor(&self.south);
        if let (Some(east), Some(west), Some(north), Some(south)) = (east, west, north, south) {
            self.is_linked(east) && self.is_linked(west) && !self.is_linked(north) && !self.is_linked(south)
        } else {
            false
        }
    }

    pub fn vertical_passage(&self) -> bool {
        if self.is_under_cell {
            return self.north.is_some() || self.south.is_some();
        }

        let east = self.get_neighbor(&self.east);
        let west = self.get_neighbor(&self.west);
        let north = self.get_neighbor(&self.north);
        let south = self.get_neighbor(&self.south);
        if let (Some(east), Some(west), Some(north), Some(south)) = (east, west, north, south) {
            !self.is_linked(east) && !self.is_linked(west) && self.is_linked(north) && self.is_linked(south)
        } else {
            false
        }
    }
    
    fn get_neighbor(&self, neighbor: &Option<OverCellLinkWeak>) -> Option<OverCellLinkStrong> {
        if neighbor.is_none(){
            None
        }
        else {
            let n = neighbor.clone().unwrap();
            // let n_src = n.upgrade().as_ref().unwrap().as_ref().borrow().self_rc.upgrade().unwrap();
            // Some(Rc::clone(&n_src))
            Some(Rc::clone(&n.upgrade().unwrap()))
        }
    }
}

fn cells_are_same(first: &Option<OverCellLinkStrong>, second: &Option<OverCellLinkStrong>) -> bool {
    web_sys::console::log_1(&JsValue::from_str("a"));
    let (fr, fc) = if let Some(n) = first.as_ref() {
        (Some(n.borrow().row()), Some(n.borrow().column()))
    } else {
        (None, None)
    };

    web_sys::console::log_1(&JsValue::from_str("b"));
    let (sr, sc) = if let Some(n) = second.as_ref() {
        (Some(n.borrow().row()), Some(n.borrow().column()))
    } else {
        (None, None)
    };

    fr == sr && fc == sc
    // if let (Some(first), Some(second)) = (first.as_ref(), second.as_ref()) {
    //     first.borrow().row() == second.borrow().row() && first.borrow().column() == second.borrow().column()
    // } else {
    //     false
    // }
}