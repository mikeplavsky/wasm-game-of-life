extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use std::fmt;
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

impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
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

    pub fn live_neighbor_count(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;
        for r in vec![self.width - 1, 0, 1] {
            for c in vec![self.height - 1, 0, 1] {
                if c == 0 && r == 0 {
                    continue;
                }

                let n_r = (row + r) % self.width;
                let n_c = (column + c) % self.height;

                let idx = self.get_index(n_r, n_c);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn rules(cell: Cell, count: u8) -> Cell {
        match (cell, count) {
            (Cell::Alive, x) if x < 2 => Cell::Dead,
            (Cell::Alive, x) if x == 2 || x == 3 => Cell::Alive,
            (Cell::Alive, x) if x > 3 => Cell::Dead,
            (Cell::Dead, 3) => Cell::Alive,
            (same, _) => same,
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for r in 0..self.width {
            for c in 0..self.height {
                let idx = self.get_index(r, c);
                let cell = self.cells[idx];

                let count = self.live_neighbor_count(r, c);
                let next_cell = Universe::rules(cell, count);

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        for s in self.cells.chunks(self.width) {
            for c in s {
                let sym = if *c == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", sym)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
