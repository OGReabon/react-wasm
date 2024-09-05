use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element};

#[wasm_bindgen]
pub fn run() {
    let window = window().expect("Should be a window in the context");

    let document = window.document().expect("window should have a document");

    let element = document
        .get_element_by_id("app")
        .expect("Should have an element with id 'output'");

    element.set_inner_html("Hello from Rust!");
}
