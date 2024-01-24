use std::collections::HashSet;
use std::fmt;
use std::ops::Range;

type CellPossibilities = HashSet<u8>;

struct Cell {
    possibilities: CellPossibilities,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            possibilities: CellPossibilities::from([1, 2, 3, 4, 5, 6, 7, 8, 9]),
        }
    }

    pub fn restrict(&mut self, num: u8) {
        self.possibilities = self
            .possibilities
            .difference(&CellPossibilities::from([num]))
            .cloned()
            .collect()
    }

    pub fn collapse(&mut self, num: u8) -> CellPossibilities {
        let mut ret = self.possibilities.clone();
        ret.remove(&num);
        self.possibilities = CellPossibilities::from([num]);
        ret
    }

    pub fn possibilities_left(&self) -> usize {
        self.possibilities.len()
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.possibilities.len() {
            1 => {
                write!(f, "{}", self.possibilities.iter().next().unwrap().clone())
            }
            0 => {
                write!(f, "0")
            }
            _ => {
                write!(f, "-")
            }
        }
    }
}

struct SudokuSpiel {
    cells: [[Cell; 9]; 9],
}

impl SudokuSpiel {
    pub fn new() -> Self {
        SudokuSpiel {
            cells: std::array::from_fn(|_| std::array::from_fn(|_| Cell::new())),
        }
    }

    pub fn collapse_block(&mut self, x: usize, y: usize, num: u8) -> CellPossibilities {
        let col = self.cells[x][y].collapse(num);
        let (block_x_range, block_y_range) = self.get_3x3_block(&x, &y);
        for it_x in block_x_range {
            for it_y in block_y_range.clone() {
                if it_x != x && it_y != y {
                    self.cells[it_x][it_y].restrict(num);
                }
            }
        }

        for i in 0..9 {
            if i != y {
                self.cells[x][i].restrict(num);
            }
            if i != x {
                self.cells[i][y].restrict(num);
            }
        }

        col
    }

    pub fn get_least_amount_of_possibilities(&self) -> (usize, usize, usize) {
        let mut least: (usize, usize, usize) = (0, 0, usize::MAX);
        for x in 0..9 {
            for y in 0..9 {
                let left = self.cells[x][y].possibilities_left();
                if left < least.2 && left > 1 {
                    least = (x, y, left)
                }
            }
        }

        least
    }

    pub fn get_3x3_block(&self, x: &usize, y: &usize) -> (Range<usize>, Range<usize>) {
        let block_start_x = x - x % 3;
        let block_start_y = y - y % 3;
        (
            block_start_x..block_start_x + 3,
            block_start_y..block_start_y + 3,
        )
    }

    pub fn solve(&mut self) {
        /*let cell_pos = self.get_least_amount_of_possibilities();
        let mut cell = &mut self.cells[cell_pos.0][cell_pos.1];
        let current_cell_possibilities = cell.possibilities.clone();
        for num in current_cell_possibilities {

        }*/
        while self.get_least_amount_of_possibilities().2 != usize::MAX {
            let cell = self.get_least_amount_of_possibilities();
            self.collapse_block(
                cell.0,
                cell.1,
                self.cells[cell.0][cell.1]
                    .possibilities
                    .iter()
                    .nth(0)
                    .unwrap()
                    .clone(),
            );
        }
    }
}

impl fmt::Display for SudokuSpiel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (num, cells) in self.cells.iter().enumerate() {
            for (num, cell) in cells.iter().enumerate() {
                write!(f, " {} ", cell)?;
                if (num + 1) % 3 == 0 {
                    write!(f, "|")?;
                }
            }
            writeln!(f, "")?;
            if (num + 1) % 3 == 0 {
                writeln!(f, "-----------------------------")?;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut spiel = SudokuSpiel::new();
    spiel.collapse_block(5, 5, 5);
    spiel.solve();
    println!("Spiel: \n{}", spiel);
}
