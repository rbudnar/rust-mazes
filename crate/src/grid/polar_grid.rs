use crate::grid::Grid;
use std::f64::consts::PI;
use web_sys::*;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::JsCast;

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

// pub struct PolarGrid {
//     pub grid: GridBase
// }

// impl PolarGrid {
//     pub fn new(grid: GridBase) -> PolarGrid {
//         PolarGrid {
//             grid
//         }
//     }

//     pub fn grid_to_web_polar(&self)  {
//         let cell_size = 10;

//         let img_size = 2 * self.grid.rows * cell_size;
//         let context = PolarGrid::setup_canvas().unwrap();
//         context.set_fill_style(&JsValue::from_str("black"));
//         context.set_stroke_style(&JsValue::from_str("black"));

//         let center = img_size / 2;

//         for cell in self.grid.each_cell().iter() {
//             if let Some(cell) = cell {
//                 let cell = cell.as_ref().borrow();
//                 let theta = 2.0 * PI / self.grid.cells[cell.row].len() as f64;
//                 let inner_radius = (cell.row * cell_size) as f64;
//                 let outer_radius = ((cell.row + 1) * cell_size) as f64;
//                 let theta_ccw = cell.column as f64 * theta;
//                 let theta_cw = (cell.column + 1) as f64 * theta;

//                 let ax = (center + (inner_radius * theta_ccw.cos()) as usize) as f64;
//                 let ay = (center + (inner_radius * theta_ccw.sin()) as usize) as f64;
//                 let bx = (center + (outer_radius * theta_ccw.cos()) as usize) as f64;
//                 let by = (center + (outer_radius * theta_ccw.sin()) as usize) as f64;
//                 let cx = (center + (inner_radius * theta_cw.cos()) as usize) as f64;
//                 let cy = (center + (inner_radius * theta_cw.sin()) as usize) as f64;
//                 let dx = (center + (outer_radius * theta_cw.cos()) as usize) as f64;
//                 let dy = (center + (outer_radius * theta_cw.sin()) as usize) as f64;

//                 if cell.north.is_some() {
//                     context.begin_path();
//                     context.move_to(ax, ay);
//                     context.line_to(cx, cy);
//                     context.stroke();
//                 }

//                 if cell.east.is_some() {
//                     context.begin_path();
//                     context.move_to(cx, cy);
//                     context.line_to(dx, dy);
//                     context.stroke();
//                 }
//             }
//         }

//         context.arc(center as f64, center as f64, (self.grid.rows * cell_size) as f64, 0.0, PI * 2.0).unwrap();
//     }

//     fn setup_canvas() -> Result<CanvasRenderingContext2d, JsValue> {
//         let document = web_sys::window().unwrap().document().unwrap();
//         // let body = Node::from(document.body().unwrap());
//         let canvas_container = document.create_element("div").unwrap();

//         let canvas = document.create_element("canvas").unwrap();
//         canvas_container.append_child(&canvas).unwrap();    
        
//         canvas.set_attribute("height", "600px").unwrap();
//         canvas.set_attribute("width", "600px").unwrap();
//         canvas.set_attribute("id", "polar_grid").unwrap();

//         {
//             let canvas_html = canvas.dyn_ref::<HtmlElement>().unwrap();
//             canvas_html.style().set_property("background-color", "rgb(239, 239, 239)").unwrap();
//             canvas_html.style().set_property("outline", "1px solid black").unwrap();
//         }

//         let canvas = canvas.dyn_into::<HtmlCanvasElement>().ok().unwrap();

//         let context = canvas.get_context("2d")?
//             .unwrap()
//             .dyn_into::<CanvasRenderingContext2d>()?;

//         Ok(context)
//     }
// }