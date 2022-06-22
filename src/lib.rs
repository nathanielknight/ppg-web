use wasm_bindgen::prelude::*;

mod ppg;

#[wasm_bindgen]
pub fn generate() -> String {
    ppg::generate()
}
