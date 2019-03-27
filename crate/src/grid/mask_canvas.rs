use web_sys::{EventTarget, ImageData};
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{HtmlElement, Node, HtmlCanvasElement, CanvasRenderingContext2d};

use super::{Grid, mask::Mask, masked_grid::MaskedGrid, GridType, canvas::{remove_old_canvas, cleanup_canvas}};
use crate::algorithms::{MazeAlgorithm, recursive_backtracker::RecursiveBacktracker};
use crate::prepare_distance_grid;
use crate::rng::wasm_rng::WasmRng;
use crate::{get_colorize, get_grid_type, set_grid_type};

static SAMPLE_RESOLUTION: usize = 5;
static MASK_CANVAS: &str = "mask_canvas";

thread_local! {
    static INVERT_MASK: RefCell<bool> = RefCell::new(false);
    static START_X: RefCell<f64> = RefCell::new(0.0);
    static START_Y: RefCell<f64> = RefCell::new(0.0);
    static IMG_DATA: RefCell<Option<ImageData>> = RefCell::new(None);
}

fn get_invert_mask() -> bool {
    INVERT_MASK.with(|m| *m.borrow())
}

fn get_start_x() -> f64 {
    START_X.with(|x| *x.borrow())
}

fn get_start_y() -> f64 {
    START_Y.with(|y| *y.borrow())
}

fn get_img_data() -> Option<ImageData> {
    IMG_DATA.with(|img| img.borrow().clone())
}

fn set_invert_mask(value: bool) {
    INVERT_MASK.with(|m| *m.borrow_mut() = value);
}

fn set_start_x(value: f64) {
    START_X.with(|x| *x.borrow_mut() = value);
}

fn set_start_y(value: f64) {
    START_Y.with(|y| *y.borrow_mut() = value);
}

fn set_img_data(img_data: Option<ImageData>) {
    IMG_DATA.with(|img| *img.borrow_mut() = img_data);
}

pub fn append_mask_canvas() {
    remove_old_canvas(MASK_CANVAS);
    let document = web_sys::window().unwrap().document().unwrap();
    let body = Node::from(document.body().unwrap());
    let canvas_container = document.create_element("div").unwrap();

    let canvas = document.create_element("canvas").unwrap();
    canvas_container.append_child(&canvas).unwrap();    
    
    canvas.set_attribute("height", "400px").unwrap();
    canvas.set_attribute("width", "400px").unwrap();
    canvas.set_attribute("id", "mask_canvas").unwrap();
    {
        let canvas_html = canvas.dyn_ref::<HtmlElement>().unwrap();
        canvas_html.style().set_property("background-color", "rgb(239, 239, 239)").unwrap();
        canvas_html.style().set_property("outline", "1px solid black").unwrap();
    }

    let canvas = canvas.dyn_into::<HtmlCanvasElement>().ok().unwrap();

    setup_drawing(canvas).unwrap();
    let controls_container = document.create_element("div").unwrap();
    {
        let controls_container = controls_container.dyn_ref::<HtmlElement>().unwrap();
        controls_container.style().set_property("display", "flex").unwrap();
        controls_container.style().set_property("justify-content", "space-between").unwrap();

        let canvas_container = canvas_container.dyn_ref::<HtmlElement>().unwrap();
        canvas_container.style().set_property("width", "400px").unwrap();
    }

    setup_invert_box(&document, &controls_container).unwrap();
    setup_clear_button(&document, &controls_container).unwrap();
    setup_apply_button(&document, &controls_container).unwrap();
    canvas_container.append_child(&controls_container).unwrap();
    body.append_child(&Node::from(canvas_container)).unwrap();    
}

fn setup_clear_button(document: &web_sys::Document, container: &web_sys::Element) -> Result<(), JsValue> {
    let clear_btn = document.create_element("button")?;
    let clear_btn = clear_btn.dyn_ref::<HtmlElement>().unwrap();
    clear_btn.set_inner_text("Clear Mask");

    let cb = Closure::wrap(Box::new(clear_mask) as Box<dyn Fn()>);

    let b = clear_btn.dyn_ref::<EventTarget>().unwrap();
    b.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
    container.append_child(&clear_btn)?;

    // This unfortunately leaks memory, but I'm not sure that there is a way around it at this time.
    // https://github.com/rustwasm/wasm-bindgen/blob/master/examples/closures/src/lib.rs#L75
    cb.forget();
    Ok(())
}

pub fn clear_mask() {
    let document = web_sys::window().unwrap().document().unwrap();    
    let canvas = document.get_element_by_id("mask_canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
    let context = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>().unwrap();

    // set canvas back to white, reset fill style for next drawing
    context.set_fill_style(&JsValue::from_str("white"));    
    context.fill_rect(0.0, 0.0, 400.0, 400.0);
    context.set_fill_style(&JsValue::from_str("black"));

    // Reset the invert mask button. If the entire canvas is "masked", then the algorithms will currently loop infinitely looking for cells. 
    // TODO: Need to put in a fail safe for the above. The user can still apply a mask to the whole canvas if they really want to.
    set_invert_mask(false);
    let invert_checkbox = document.get_element_by_id("invert_chk").unwrap();
    let invert_checkbox = invert_checkbox.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
    invert_checkbox.set_checked(false);
}

fn setup_apply_button(document: &web_sys::Document, container: &web_sys::Element) -> Result<(), JsValue> {
    let apply_btn = document.create_element("button")?;
    let apply_btn = apply_btn.dyn_ref::<HtmlElement>().unwrap();
    apply_btn.set_inner_text("Apply Mask");

    let cb = Closure::wrap(Box::new(|| {
        cleanup_canvas(&get_grid_type());
        set_grid_type(GridType::StandardGrid);
        canvas_to_mask(get_colorize());
    }) as Box<dyn Fn()>);
    let b = apply_btn.dyn_ref::<EventTarget>().unwrap();
    b.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
    container.append_child(&apply_btn)?;

    // This unfortunately leaks memory, but I'm not sure that there is a way around it at this time.
    // https://github.com/rustwasm/wasm-bindgen/blob/master/examples/closures/src/lib.rs#L75
    cb.forget();
    Ok(())
}

fn setup_invert_box(document: &web_sys::Document, container: &web_sys::Element) -> Result<(), JsValue> {
    let check_box = document.create_element("input")?;
    let label = document.create_element("label")?;
    let flex_container = document.create_element("div")?;

    check_box.set_attribute("type", "checkbox")?;
    check_box.set_attribute("id", "invert_chk")?;
    label.set_attribute("for", "invert_chk")?;
    label.set_inner_html("Invert Mask");
    
    flex_container.append_child(&check_box)?;
    flex_container.append_child(&label)?;
    container.append_child(&flex_container)?;
    let cb = Closure::wrap(Box::new(|event: web_sys::Event| {
        if let Some(target) = event.target() {
            if let Some(input_el) = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&target)
            {
                set_invert_mask(input_el.checked());                    
            }
        }
    }) as Box<dyn Fn(_)>);
    let b = check_box.dyn_ref::<EventTarget>().unwrap();
    b.add_event_listener_with_callback("change", cb.as_ref().unchecked_ref()).unwrap();
    cb.forget();

    Ok(())
}

fn setup_drawing(canvas: HtmlCanvasElement) -> Result<(), JsValue> {
    let context = canvas.get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;
    
    let canvas = Rc::new(canvas);
    let context = Rc::new(context);
    context.set_fill_style(&JsValue::from_str("white"));    
    context.fill_rect(0.0, 0.0, 400.0, 400.0);

    context.set_fill_style(&JsValue::from_str("black"));

    let context = context.clone();
    let context2 = context.clone();
    let context3 = context.clone();

    let cv = canvas.clone();
    let cv2 = canvas.clone();
    let cv3 = canvas.clone();
    let cv4 = canvas.clone();

    let mouse_move = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let end_x = f64::from(event.offset_x());
        let end_y = f64::from(event.offset_y());

        context.clear_rect(0.0,0.0,400.0,400.0);
        context.put_image_data(&get_img_data().unwrap(), 0.0, 0.0).unwrap();
        let start_x = get_start_x();
        let start_y = get_start_y();
        context.fill_rect(start_x, start_y, end_x - start_x, end_y - start_y);
            // web_sys::console::log_1(&JsValue::from_str(&format!("move {} {}", START_X, START_Y)));

    }) as Box<dyn FnMut(_)>);

    let mm = Rc::new(mouse_move);
    let mm1 = mm.clone();
    
    let mouse_down = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        set_img_data(context2.get_image_data(0.0, 0.0, 400.0, 400.0).ok());
        set_start_x(f64::from(event.offset_x()));
        set_start_y(f64::from(event.offset_y()));
        cv.add_event_listener_with_callback("mousemove", (*mm).as_ref().unchecked_ref()).unwrap();

    }) as Box<dyn FnMut(_)>);
    
    cv2.add_event_listener_with_callback("mousedown", mouse_down.as_ref().unchecked_ref())?;
    mouse_down.forget();

    let mouse_up = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let end_x = f64::from(event.offset_x());
        let end_y = f64::from(event.offset_y());
        let start_x = get_start_x();
        let start_y = get_start_y();
        context3.fill_rect(start_x, start_y, end_x - start_x, end_y - start_y);
        cv4.remove_event_listener_with_callback("mousemove", (*mm1).as_ref().unchecked_ref()).unwrap();
    }) as Box<dyn FnMut(_)>);
    
    cv3.add_event_listener_with_callback("mouseup", mouse_up.as_ref().unchecked_ref())?;
    mouse_up.forget();    

    Ok(())
}

pub fn canvas_to_mask(colorize: bool) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(MASK_CANVAS).unwrap();
    
    let canvas = canvas.dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let width = canvas.offset_width() as usize;
    let height = canvas.offset_height() as usize;
    
    // web_sys::console::log_2(&JsValue::from(width), &JsValue::from(height));

    let context = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let img_data = context.get_image_data(0.0, 0.0, width as f64, height as f64).unwrap();
    // web_sys::console::log_1(&img_data);

    let mut mask = Mask::new(height / SAMPLE_RESOLUTION, width / SAMPLE_RESOLUTION);
    let data = &img_data.data();

    let color = if get_invert_mask() {
        0 
    } else {
        255
    };
    
    for i in 0..mask.bits.len() {
        let columns = mask.bits[i].len();
        for j in 0..columns {
            let i_offset = i * 4 * SAMPLE_RESOLUTION * SAMPLE_RESOLUTION * columns;
            let j_offset = j * 4 * SAMPLE_RESOLUTION;
            let first_index = i_offset + j_offset;

            if data[first_index] == color && data[first_index+1] == color && data[first_index+2] == color {
                // web_sys::console::log_1(&JsValue::from_str(&format!("{}, {}, {}", i, j, first_index)));
                mask.bits[i][j] = false;
            }
        }
        // web_sys::console::log_1(&JsValue::from_str(&format!("{}, {}, {}", i, cell_count, cell_count*4*SAMPLE_RESOLUTION)));
    }

    let masked_grid = MaskedGrid::new(mask);
    RecursiveBacktracker.on(&masked_grid, &WasmRng);
    let distance_grid = prepare_distance_grid(&masked_grid);
    masked_grid.to_web(&distance_grid, colorize, 0.1);
}