use itertools::Itertools;

pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

pub struct Cell {
    pub val: char,
    pub col: usize,
    pub row: usize,
}

pub struct Matrix {
    pub cells: Vec<Vec<char>>,
    pub cols: usize,
    pub rows: usize,
}

impl From<&str> for Matrix {
    fn from(s: &str) -> Self {
        let cells: Vec<Vec<char>> = s
            .lines()
            .filter_map(|l| {
                if !l.is_empty() {
                    Some(l.chars().collect())
                } else {
                    None
                }
            })
            .collect();

        Self {
            cols: cells[0].len(),
            rows: cells.len(),
            cells,
        }
    }
}

impl Matrix {
    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        self.cells.get(row).and_then(|l| l.get(col).copied())
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<Cell> {
        self.get(row, col).map(|val| Cell { col, row, val })
    }

    pub fn items(&self) -> impl Iterator<Item = Cell> + '_ {
        (0..self.rows)
            .cartesian_product(0..self.cols)
            .map(|(row, col)| self.get_cell(row, col).unwrap())
    }

    pub fn neighbour(&self, cell: &Cell, dir: &Direction) -> Option<Cell> {
        match dir {
            Direction::NW => {
                let row = cell.row.checked_sub(1)?;
                let col = cell.col.checked_sub(1)?;
                let val = self.get(row, col)?;
                Some(Cell { col, row, val })
            }
            Direction::N => {
                let col = cell.col;
                let row = cell.row.checked_sub(1)?;
                let val = self.get(row, col)?;
                Some(Cell { col, row, val })
            }
            Direction::NE => {
                let col = cell.col + 1;
                let row = cell.row.checked_sub(1)?;
                let val = self.get(row, col)?;
                Some(Cell { col, row, val })
            }
            Direction::W => {
                let col = cell.col.checked_sub(1)?;
                let row = cell.row;
                let val = self.get(row, col)?;
                Some(Cell { col, row, val })
            }
            Direction::E => {
                let col = cell.col + 1;
                let row = cell.row;
                let val = self.get(row, col)?;
                Some(Cell { col, row, val })
            }
            Direction::SW => {
                let col = cell.col.checked_sub(1)?;
                let row = cell.row + 1;
                let val = self.get(row, col)?;
                Some(Cell { col, row, val })
            }
            Direction::S => {
                let row = cell.row + 1;
                let col = cell.col;
                let val = self.get(row, col)?;
                Some(Cell { col, row, val })
            }
            Direction::SE => {
                let col = cell.col + 1;
                let row = cell.row + 1;
                let val = self.get(row, col)?;
                Some(Cell { col, row, val })
            }
        }
    }

    pub fn area(
        &self,
        col_start: usize,
        col_end: usize,
        row_start: usize,
        row_end: usize,
    ) -> impl Iterator<Item = Cell> + '_ {
        (row_start..=row_end)
            .cartesian_product(col_start..=col_end)
            .filter_map(|(row, col)| self.get(row, col).map(|val| Cell { col, row, val }))
    }
}
