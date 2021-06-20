mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width =  64;
        let height = 64;
        let size = (width + 2) * (height + 2);

        let cells = (0..size).map(|_i| Cell::Dead).collect();

        let mut instance = Universe {
            width,
            height,
            cells,
        };

        let mut i = 0;
        for row in 0..height {
            for col in 0..width {
                if i % 2 == 0 || i % 7 == 0 {
                    instance.toggle_cell(row, col);
                }
                i += 1;
            }
        }

        instance
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        let size = (width + 2) * (self.height + 2);
        self.width = width;
        self.cells = (0..size).map(|_i| Cell::Dead).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        let size = (self.width + 2) * (height + 2);
        self.height = height;
        self.cells = (0..size).map(|_i| Cell::Dead).collect();
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");

        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
        self.update_border();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();

        if self.on_border(row, column) {
            self.update_border();
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        ((row + 1) * (self.width + 2) + column + 1) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        let mut idx = self.get_index(row, column) - (self.width + 3) as usize;

        count += self.cells[idx] as u8; // NW
        idx += 1;
        count += self.cells[idx] as u8; // N
        idx += 1;
        count += self.cells[idx] as u8; // NE
        idx += self.width as usize;
        count += self.cells[idx] as u8; // W
        idx += 2;
        count += self.cells[idx] as u8; // E
        idx += self.width as usize;
        count += self.cells[idx] as u8; // SW
        idx += 1;
        count += self.cells[idx] as u8; // S
        idx += 1;
        count += self.cells[idx] as u8; // SE

        count
    }

    fn on_border(&self, row: u32, col: u32) -> bool {
        row == 0 || col == 0 || row == self.height - 1 || col == self.width - 1
    }

    fn update_border(&mut self) {
        let mut i = self.get_index(0, 0);
        let mut j = self.get_index(0, self.width - 1);
        let row_size = (self.width + 2) as usize;
        for _ in 0..self.height {
            self.cells[i - 1] = self.cells[j];
            self.cells[j + 1] = self.cells[i];
            i += row_size;
            j += row_size;
        };

        i = self.get_index(0, 0) - 1;
        j = self.get_index(self.height - 1, 0) - 1;
        for _ in 0..row_size {
            self.cells[i - row_size] = self.cells[j];
            self.cells[j + row_size] = self.cells[i];
            i += 1;
            j += 1;
        };
    }
}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }

        self.update_border();
    }
}