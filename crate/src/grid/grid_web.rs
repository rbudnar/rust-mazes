
use crate::grid::cell::ICellStrong;
use crate::grid::{Grid, CellFormatter};
use web_sys::{Node};
use wasm_bindgen::prelude::JsValue;


// Needed to be able to remove the old style sheet when creating new mazes
static mut STYLESHEET: Option<web_sys::Element> = None;

pub fn grid_to_web(grid: &dyn Grid, formatter: &dyn CellFormatter, colorize: bool) {
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

    grid.to_web(&document, &grid_container, formatter, colorize);
    
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

pub fn add_class(element: &web_sys::Element, css_class: &str) {
    let arr = js_sys::Array::new();
    arr.push(&JsValue::from_str(css_class));
    element.class_list().add(&arr).expect("should do stuff dammit");
}

pub fn add_bg_color(element: &web_sys::HtmlElement, cell: &ICellStrong, formatter: &dyn CellFormatter) {
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