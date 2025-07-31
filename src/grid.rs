use crate::symbols::Symbol;

// Array of Symbols seems cleanest for a small grid, since we already implemented the Symbol type
// For larger/variable len grids I would use vector
pub struct Grid(pub [Symbol; 9]);

impl Grid {
    pub fn new() -> Self {
        Grid([Symbol::Empty; 9])
    }

    pub fn is_full(&self) -> bool {
        self.0.iter().all(|&cell| cell != Symbol::Empty)
    }

    pub fn print(&self) {
        for (i, row) in self.0.chunks(3).enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if j > 0 {
                    print!("|");
                }
                print!(" {} ", cell.to_str());
            }
            println!();
            if i < 2 {
                println!("-----------");
            }
        }
    }

    pub fn set_symbol(&mut self, index: usize, symbol: Symbol) -> bool {
        if (index >= 9) || self.0[index] != Symbol::Empty {
            return false; // Invalid index or cell already occupied
        }
        self.0[index] = symbol;
        true
    }

    pub fn is_won(&self) -> bool {
        // 0 1 2
        // 3 4 5
        // 6 7 8
        let winning_combinations = [
            [0, 1, 2], // Row 1
            [3, 4, 5], // Row 2
            [6, 7, 8], // Row 3
            [0, 3, 6], // Column 1
            [1, 4, 7], // Column 2
            [2, 5, 8], // Column 3
            [0, 4, 8], // Diagonal \
            [2, 4, 6], // Diagonal /
        ];

        for combination in winning_combinations.iter() {
            if self.0[combination[0]] != Symbol::Empty
                && self.0[combination[0]] == self.0[combination[1]]
                && self.0[combination[1]] == self.0[combination[2]]
            {
                return true;
            }
        }
        false
    }
}

pub fn grid_to_index(grid: &Grid) -> usize {
    let mut index = 0;
    for (i, &symbol) in grid.0.iter().enumerate() {
        let symbol_num = match symbol {
            Symbol::Empty => 0,
            Symbol::Circle => 1,
            Symbol::Cross => 2,
        };
        // Convert to trinary number
        index += symbol_num * 3_usize.pow(i as u32);
    }
    index
}

pub fn index_to_grid(index: usize) -> Grid {
    let mut grid = Grid::new();
    let mut grid_num = index;

    for i in 0..9 {
        let symbol_num = grid_num % 3; // get last digit
        grid.0[i] = match symbol_num {
            0 => Symbol::Empty,
            1 => Symbol::Circle,
            2 => Symbol::Cross,
            _ => unreachable!(), // This should never happen since we only use 0, 1, 2
        };
        grid_num /= 3; // remove last digit
    }

    grid
}
