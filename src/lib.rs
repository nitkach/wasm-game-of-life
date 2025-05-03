use std::fmt::Write;
use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.cells.chunks(self.width as usize) {
            for cell in line {
                let char = match cell {
                    Cell::Dead => '◻',
                    Cell::Alive => '◼',
                };
                f.write_char(char)?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Universe {
    pub fn new(width: u32, height: u32) -> Option<Self> {
        let repeat_times = width.checked_mul(height)?;

        let cells = std::iter::repeat_n(Cell::Dead, repeat_times as usize).collect();

        Some(Self {
            width,
            height,
            cells,
        })
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let top_row = (row + self.height - 1) % self.height;
        let middle_row = (row + self.height) % self.height;
        let bottom_row = (row + self.height + 1) % self.height;

        let left_column = (column + self.width - 1) % self.width;
        let middle_column = (column + self.width) % self.width;
        let right_column = (column + self.width + 1) % self.width;

        let mut live_cells_count = 0;
        // upper row
        live_cells_count += self.cells[self.get_index(top_row, left_column)] as u8;
        live_cells_count += self.cells[self.get_index(top_row, middle_column)] as u8;
        live_cells_count += self.cells[self.get_index(top_row, right_column)] as u8;
        // middle row
        live_cells_count += self.cells[self.get_index(middle_row, left_column)] as u8;
        live_cells_count += self.cells[self.get_index(middle_row, right_column)] as u8;
        // bottom row
        live_cells_count += self.cells[self.get_index(bottom_row, left_column)] as u8;
        live_cells_count += self.cells[self.get_index(bottom_row, middle_column)] as u8;
        live_cells_count += self.cells[self.get_index(bottom_row, right_column)] as u8;

        live_cells_count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new_with_live_cells() -> Self {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|number| {
                if number % 2 == 0 || number % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn new_single_spaceship() -> Self {
        let width = 64;
        let height = 64;

        let mut cells =
            std::iter::repeat_n(Cell::Dead, (width * height) as usize).collect::<Vec<_>>();

        let transform = |row, column| (row * width + column) as usize;

        /*
        creates single spaceship:
        ADDAD
        DDDDA
        ADDDA
        DAAAA
         */
        let alive_coordinates = [
            (0, 0),
            (0, 3),
            (1, 4),
            (2, 0),
            (2, 4),
            (3, 1),
            (3, 2),
            (3, 3),
            (3, 4),
        ];

        alive_coordinates
            .iter()
            .for_each(|(row, column)| cells[transform(row, column)] = Cell::Alive);

        Self {
            width,
            height,
            cells,
        }
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

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.get_index(row, column);
                let cell = self.cells[index];
                let live_neighbor_count = self.live_neighbor_count(row, column);

                let next_cell = match (cell, live_neighbor_count) {
                    // Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
                    (Cell::Alive, live_cells) if live_cells < 2 => Cell::Dead,
                    // Any live cell with two or three live neighbours lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Any live cell with more than three live neighbours dies, as if by overpopulation.
                    (Cell::Alive, live_cells) if live_cells > 3 => Cell::Dead,
                    // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[index] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universe_live_neighbors_general() {
        /*
        AAAD
        AXAD
        AAAD
        DDDD
         */
        let universe = Universe {
            width: 4,
            height: 4,
            cells: vec![
                // row 0
                Cell::Alive,
                Cell::Alive,
                Cell::Alive,
                Cell::Dead,
                // row 1
                Cell::Alive,
                Cell::Dead,
                Cell::Alive,
                Cell::Dead,
                // row 2
                Cell::Alive,
                Cell::Alive,
                Cell::Alive,
                Cell::Dead,
                // row 3
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
            ],
        };
        assert_eq!(8, universe.live_neighbor_count(1, 1));

        /*
        DDDD
        DAAA
        DAXA
        DAAA
         */
        let universe = Universe {
            width: 4,
            height: 4,
            cells: vec![
                // row 0
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
                // row 1
                Cell::Dead,
                Cell::Alive,
                Cell::Alive,
                Cell::Alive,
                // row 2
                Cell::Dead,
                Cell::Alive,
                Cell::Dead,
                Cell::Alive,
                // row 3
                Cell::Dead,
                Cell::Alive,
                Cell::Alive,
                Cell::Alive,
            ],
        };
        assert_eq!(8, universe.live_neighbor_count(2, 2));
    }

    #[test]
    fn test_universe_live_neighbors_periodic() {
        /*
        AADA
        XADA
        AADA
        DDDD
         */
        let universe = Universe {
            width: 4,
            height: 4,
            cells: vec![
                // row 0
                Cell::Alive,
                Cell::Alive,
                Cell::Dead,
                Cell::Alive,
                // row 1
                Cell::Dead,
                Cell::Alive,
                Cell::Dead,
                Cell::Alive,
                // row 2
                Cell::Alive,
                Cell::Alive,
                Cell::Dead,
                Cell::Alive,
                // row 3
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
            ],
        };
        assert_eq!(8, universe.live_neighbor_count(1, 0));

        /*
        ADAA
        DDDD
        ADAA
        ADAX
         */
        let universe = Universe {
            width: 4,
            height: 4,
            cells: vec![
                // row 0
                Cell::Alive,
                Cell::Dead,
                Cell::Alive,
                Cell::Alive,
                // row 1
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
                // row 2
                Cell::Alive,
                Cell::Dead,
                Cell::Alive,
                Cell::Alive,
                // row 3
                Cell::Alive,
                Cell::Dead,
                Cell::Alive,
                Cell::Dead,
            ],
        };
        assert_eq!(8, universe.live_neighbor_count(3, 3));
    }
}
