use wasm_bindgen::prelude::*;

// this allocator will reduce the .wasm file size
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(name);
}

#[wasm_bindgen]
extern {
    pub fn alert(name: &str);
}
