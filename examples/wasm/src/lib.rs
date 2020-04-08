mod utils;

use minifb::{Key, ScaleMode, Window, WindowOptions};
use std::panic;
use wasm_bindgen::prelude::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

#[wasm_bindgen(start)]
pub fn setup() {
    // Set the panic hook so we can have proper error messages in the console log
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn new_window() {
    let mut window = Window::new("Example", 600, 400, WindowOptions::default()).unwrap();
}
