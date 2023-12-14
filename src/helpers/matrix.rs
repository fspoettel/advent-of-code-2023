use itertools::Itertools;
use std::hash::{Hash, Hasher};

/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    N,
    E,
    S,
    W,
    NE,
    SE,
    SW,
    NW,
}

impl Direction {
    pub fn invert(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            Direction::NW => Direction::SE,
            Direction::NE => Direction::SW,
            Direction::SE => Direction::NW,
            Direction::SW => Direction::NE,
        }
    }
}

pub static CARDINALS: [Direction; 4] = [Direction::N, Direction::E, Direction::S, Direction::W];

pub static ORDINALS: [Direction; 4] = [Direction::NW, Direction::NE, Direction::SE, Direction::SW];

/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Copy, Eq)]
pub struct Cell<T: Copy = char> {
    pub val: T,
    pub col: usize,
    pub row: usize,
}

impl<T: Copy> PartialEq for Cell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.col == other.col && self.row == other.row
    }
}

impl<T: Copy> Hash for Cell<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.col.hash(state);
        self.row.hash(state);
    }
}

/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Matrix<T: Copy = char> {
    pub cells: Vec<Vec<T>>,
    pub cols: usize,
    pub rows: usize,
}

impl From<&str> for Matrix<char> {
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

impl<T: Copy> Matrix<T> {
    pub fn get(&self, row: usize, col: usize) -> Option<T> {
        self.cells.get(row).and_then(|l| l.get(col).copied())
    }

    pub fn get_row(&self, row: usize) -> Option<&Vec<T>> {
        self.cells.get(row)
    }

    pub fn get_row_mut(&mut self, row: usize) -> Option<&mut Vec<T>> {
        self.cells.get_mut(row)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.cells.get_mut(row).and_then(|l| l.get_mut(col))
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<Cell<T>> {
        self.get(row, col).map(|val| Cell { col, row, val })
    }

    pub fn items(&self) -> impl Iterator<Item = Cell<T>> + '_ {
        (0..self.rows)
            .cartesian_product(0..self.cols)
            .map(|(row, col)| self.get_cell(row, col).unwrap())
    }

    pub fn neighbour(&self, cell: &Cell<T>, dir: &Direction) -> Option<Cell<T>> {
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

    pub fn neighbours(
        &self,
        start: Cell<T>,
        directions: Vec<Direction>,
    ) -> impl Iterator<Item = (Direction, Option<Cell<T>>)> + '_ {
        directions.into_iter().map(move |dir| {
            let neighbour = self.neighbour(&start, &dir);
            (dir, neighbour)
        })
    }

    pub fn all_neighbours(
        &self,
        start: Cell<T>,
        include_ordinals: bool,
    ) -> impl Iterator<Item = (Direction, Option<Cell<T>>)> + '_ {
        let mut neighbours = Vec::from(CARDINALS);

        if include_ordinals {
            neighbours.extend(Vec::from(ORDINALS));
        }

        neighbours.into_iter().map(move |dir| {
            let neighbour = self.neighbour(&start, &dir);
            (dir, neighbour)
        })
    }

    pub fn area(
        &self,
        col_start: usize,
        col_end: usize,
        row_start: usize,
        row_end: usize,
    ) -> impl Iterator<Item = Cell<T>> + '_ {
        (row_start..=row_end)
            .cartesian_product(col_start..=col_end)
            .filter_map(|(row, col)| self.get(row, col).map(|val| Cell { col, row, val }))
    }
}
