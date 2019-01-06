extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new() -> Universe {
        Universe {
            width: 4,
            height: 4,
            cells: vec![],
        }
    }

    pub fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    pub fn live_neighbor_count(&self, row: usize, column: usize) -> usize {
        let mut count = 0;
        for d_r in vec![self.width - 1, 0, 1] {
            for d_c in vec![self.height - 1, 0, 1] {

                if d_c == 0 && d_r == 0 { continue; }

            }
        }
        count
    }
}
