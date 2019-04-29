use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{HtmlElement, Node, HtmlCanvasElement, CanvasRenderingContext2d};
use super::{GridType, triangle_grid::TRIANGLE_GRID, hex_grid::HEX_GRID, polar_grid::POLAR_GRID, standard_grid::STANDARD_GRID, weave_grid::WEAVE_GRID};

pub fn cleanup_canvas(grid_type: &GridType) {
    match grid_type {
        GridType::StandardGrid => remove_old_canvas(STANDARD_GRID),
        GridType::PolarGrid => remove_old_canvas(POLAR_GRID),
        GridType::HexGrid => remove_old_canvas(HEX_GRID),
        GridType::TriangleGrid => remove_old_canvas(TRIANGLE_GRID),
        GridType::WeaveGrid => remove_old_canvas(WEAVE_GRID)
    }            
}

pub fn setup_grid_canvas(element_id: &str) -> Result<CanvasRenderingContext2d, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = Node::from(document.body().unwrap());
    let canvas_container = document.create_element("div").unwrap();
    body.append_child(&canvas_container)?;
    let canvas = document.create_element("canvas").unwrap();
    canvas_container.append_child(&canvas).unwrap();        
    // height/width is now set by the grid depending on how big the maze is.
    canvas.set_attribute("id", element_id).unwrap();

    {
        let canvas_html = canvas.dyn_ref::<HtmlElement>().unwrap();
        canvas_html.style().set_property("margin", "10px").unwrap();
        // canvas_html.style().set_property("background-color", "rgb(239, 239, 239)").unwrap();
        // canvas_html.style().set_property("outline", "1px solid black").unwrap();
    }

    let canvas = canvas.dyn_into::<HtmlCanvasElement>().ok().unwrap();

    let context = canvas.get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    Ok(context)
}

pub fn remove_old_canvas(element_id: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let old_canvas = document.get_element_by_id(element_id);
    if let Some(old_canvas) = old_canvas {
        old_canvas.remove();
    }
}

pub fn set_canvas_size(element_id: &str, width: usize, height: usize) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(element_id);
    if let Some(canvas) = canvas {
        // add a little padding to avoid clipping
        canvas.set_attribute("height", &(height + 5).to_string()).unwrap();
        canvas.set_attribute("width", &(width + 5).to_string()).unwrap();
    }
}

pub fn draw_line(ctx: &CanvasRenderingContext2d, x1: f64, y1: f64, x2: f64, y2: f64) {
    ctx.begin_path();
    ctx.move_to(x1, y1);
    ctx.line_to(x2, y2);
    ctx.stroke();
}

pub fn draw_shape(ctx: &CanvasRenderingContext2d, xys: Vec<(f64,f64)>, color: &str) {
    if xys.is_empty() {
        return;
    }

    ctx.begin_path();
    ctx.set_fill_style(&JsValue::from_str(color));
    ctx.set_stroke_style(&JsValue::from_str(color));

    for (i, (x, y)) in xys.iter().enumerate() {
        if i == 0 {
            ctx.move_to(*x, *y);
        } else {
            ctx.line_to(*x, *y);
        }
    }

    ctx.fill();
    ctx.set_fill_style(&JsValue::from_str("black"));
    ctx.set_stroke_style(&JsValue::from_str("black"));
}

pub enum DrawMode {
    Line,
    Background
}