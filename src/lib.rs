use wasm_bindgen::prelude::*;

// this allocator will reduce the .wasm file size
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
}

impl Snake {
    pub fn new(initial_index: usize) -> Snake {
        Snake {
            body: vec![SnakeCell(initial_index)],
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: i32,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        World {
            width: 8,
            snake: Snake::new(10),
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn snake_head(&self) -> usize {
        self.snake.body[0].0
    }
}
