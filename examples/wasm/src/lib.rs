extern crate minifb;

mod utils;

use minifb::{Key, ScaleMode, Window, WindowOptions};
use wasm_bindgen::prelude::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

#[wasm_bindgen]
pub fn main() {
    let mut noise;
    let mut carry;
    let mut seed = 0xbeefu32;

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
