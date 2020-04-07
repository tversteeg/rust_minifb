mod utils;

use minifb::{Key, ScaleMode, Window, WindowOptions};
use std::panic;
use wasm_bindgen::prelude::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

#[wasm_bindgen]
pub fn setup_window() {
    // Set the panic hook so we can have proper error messages in the console log
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut window = Window::new(
        "Noise Test - Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
}
