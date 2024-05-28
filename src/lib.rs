use wasm_bindgen::prelude::*;

// this allocator will reduce the .wasm file size
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
struct World {
    pub width: i32,
}

#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        World { width: 8 }
    }

    pub fn width(&self) -> i32 {
        self.width
    }
}
