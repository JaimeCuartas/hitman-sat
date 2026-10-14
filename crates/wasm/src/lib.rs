use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sat() -> bool {
    hitman::small_sat()
}
