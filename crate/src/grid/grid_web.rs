
use grid::{Grid, cell::CellLinkStrong, CellFormatter};
use web_sys::{HtmlElement, Node};
use std::rc::{Rc};
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::JsCast;

// Needed to be able to remove the old style sheet when creating new mazes
static mut STYLESHEET: Option<web_sys::Element> = None;

pub fn grid_to_web(grid: &Grid, formatter: &CellFormatter, colorize: bool) {
    cleanup_old_maze();

    let document = web_sys::window().unwrap().document().unwrap();
    let grid_container = document.create_element("div").unwrap();
    add_class(&grid_container, "grid-container");

    let rule = format!("
        .grid-container {{
            display: grid;
            grid-template-columns: repeat({}, 1fr);
            height: 1000px;
            width: 1000px;
            background-color: #efefef;
        }}
       ", grid.columns());

    unsafe {
        if STYLESHEET.is_some() {
            let s = STYLESHEET.clone().unwrap();
            remove_stylesheet(&s);
        }
        STYLESHEET = Some(create_style_sheet());
        let stylesheet = STYLESHEET.clone().unwrap();
        add_css_rule(&stylesheet, &rule);

        document.head().unwrap().append_child(&Node::from(stylesheet))
            .expect("sheet should have been added");
    }

    for (i, row) in grid.cells().iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if let Some(cell) = cell {
                let html_cell = document.create_element("div").unwrap();
                add_class(&html_cell, "cell");

                // Top of maze
                if i == 0 || (i > 0 && !grid.cells()[i-1][j].is_some()) {
                    add_class(&html_cell, "bt");
                }

                // bottom of maze
                if i == grid.rows() - 1 {
                    add_class(&html_cell, "bb");
                }
                // left side 
                if j == 0  || ( j > 0 && !grid.cells()[i][j-1].is_some()) {
                    add_class(&html_cell, "bl");
                }

                // right side
                if j == grid.columns() -1 {
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

                let c = html_cell.dyn_ref::<HtmlElement>().unwrap().clone();
                if colorize {
                    add_bg_color(&c, cell, formatter);
                }
                grid_container.append_child(&Node::from(html_cell)).unwrap();
            }
            else {
                let html_cell = document.create_element("div").unwrap();
                add_class(&html_cell, "cell");
              
                html_cell.dyn_ref::<HtmlElement>().unwrap().style().set_property("background-color", "white").unwrap();
                grid_container.append_child(&Node::from(html_cell)).unwrap();
            }
        }
    }

    let body = Node::from(document.body().unwrap());
    body.append_child(&Node::from(grid_container)).unwrap();
}

fn create_style_sheet() -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    document.create_element("style").unwrap()
}

fn add_css_rule(sheet: &web_sys::Element, rule: &str) {
    let styles = sheet.inner_html() + rule;
    sheet.set_inner_html(&styles);
}

fn add_class(element: &web_sys::Element, css_class: &str) {
    let arr = js_sys::Array::new();
    arr.push(&JsValue::from_str(css_class));
    element.class_list().add(&arr).expect("should do stuff dammit");
}

fn add_bg_color(element: &web_sys::HtmlElement, cell: &CellLinkStrong, formatter: &CellFormatter) {
    let color = formatter.background_color(cell);
    element.style().set_property("background-color", &color).unwrap();
}

fn remove_stylesheet(element: &web_sys::Element) {
    element.remove();
}

fn cleanup_old_maze() {
    let document = web_sys::window().unwrap().document().unwrap();
    let old_grid = document.query_selector(".grid-container").unwrap();
    if let Some(g) = old_grid {
        g.remove();
    }
}