extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use std::fmt::Display;
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

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    pub fn live_neighbor_count(&self, row: usize, column: usize) -> usize {
        let mut count = 0;
        for r in vec![self.width - 1, 0, 1] {
            for c in vec![self.height - 1, 0, 1] {
                if c == 0 && r == 0 {
                    continue;
                }

                let n_r = (self.width + row) % self.width;
                let n_c = (self.height + column) % self.height;

                let idx = self.get_index(n_r, n_c);
                count += self.cells[idx] as usize;
            }
        }
        count
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for s in self.cells.chunks(self.width) {
            for c in s {
                let sym = if *c == Cell::Alive { '◻' } else { '◼' };
                write!(f, "{}", sym)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
