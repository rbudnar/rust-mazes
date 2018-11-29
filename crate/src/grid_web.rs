use grid::*;
use web_sys::*;
use std::rc::{Rc};
use wasm_bindgen::prelude::*;

pub fn grid_to_web(grid: &Grid) {
    let document = web_sys::window().unwrap().document().unwrap();
    let grid_container = document.create_element("div").unwrap();
    add_class(&grid_container, "grid-container");

    let stylesheet = create_style_sheet();
    let rule = format!("
        .grid-container {{\n
        display: grid;\n
        grid-template-columns: repeat({}, 1fr);\n
        height: 80vw; \n
        width: 80vw;\n
        background-color: #efefef;\n
        border: 1px solid black;\n
        }}
       ", grid.columns);

    add_css_rule(&stylesheet, &rule);

    document.head().unwrap().append_child(&Node::from(stylesheet))
        .expect("sheet should have been added");

    for (i, row) in grid.cells.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let html_cell = document.create_element("div").unwrap();
            add_class(&html_cell, "cell");
            // Top of maze
            if i == 0 {
                add_class(&html_cell, "bt");
            }

            // bottom of maze
            if i == grid.rows - 1 {
                add_class(&html_cell, "bb");
            }
            // left side 
            if j == 0 {
                add_class(&html_cell, "bl");
            }

            // right side
            if j == grid.columns -1 {
                add_class(&html_cell, "br");
            }

            let e = cell.borrow();
            let east = e.east.as_ref();
            if !(east.is_some() && cell.borrow().is_linked(Rc::clone(&east.unwrap().upgrade().unwrap()))) {
                add_class(&html_cell, "br");            
            }

            let south = e.south.as_ref();
            if !(south.is_some() && cell.borrow().is_linked(Rc::clone(&south.unwrap().upgrade().unwrap()))) {
                add_class(&html_cell, "bb");            
            }

            grid_container.append_child(&Node::from(html_cell)).unwrap();
        }
    }

    let body = Node::from(document.body().unwrap());
    body.append_child(&Node::from(grid_container)).unwrap();

}

pub fn create_style_sheet() -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let style = document.create_element("style").unwrap();
    style
}

pub fn add_css_rule(sheet: &web_sys::Element, rule: &str) {
    let styles = sheet.inner_html() + rule;
    sheet.set_inner_html(&styles);
}

pub fn add_class(element: &web_sys::Element, css_class: &str) {
    let arr = js_sys::Array::new();
    arr.push(&JsValue::from_str(css_class));
    element.class_list().add(&arr).expect("should do stuff dammit");
}