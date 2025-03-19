use wasm_bindgen::prelude::wasm_bindgen;

#[repr(u8)]
#[derive(Clone, PartialEq)]
#[wasm_bindgen]
pub enum Visibility {
    DEFAULT,
    SOFT,
    HARD,
}