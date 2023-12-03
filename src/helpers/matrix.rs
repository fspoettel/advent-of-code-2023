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
    pub x: usize,
    pub y: usize,
}

pub struct Matrix {
    pub cells: Vec<Vec<char>>,
    pub w: usize,
    pub h: usize,
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
            w: cells[0].len(),
            h: cells.len(),
            cells,
        }
    }
}

impl Matrix {
    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        self.cells.get(y).and_then(|l| l.get(x).copied())
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        self.get(x, y).map(|val| Cell { x, y, val })
    }

    pub fn items(&self) -> impl Iterator<Item = Cell> + '_ {
        (0..self.h)
            .cartesian_product(0..self.w)
            .map(|(y, x)| self.get_cell(x, y).unwrap())
    }

    pub fn neighbour(&self, cell: &Cell, dir: &Direction) -> Option<Cell> {
        match dir {
            Direction::NW => {
                let y = cell.y.checked_sub(1)?;
                let x = cell.x.checked_sub(1)?;
                let val = self.get(x, y)?;
                Some(Cell { x, y, val })
            }
            Direction::N => {
                let x = cell.x;
                let y = cell.y.checked_sub(1)?;
                let val = self.get(x, y)?;
                Some(Cell { x, y, val })
            }
            Direction::NE => {
                let x = cell.x + 1;
                let y = cell.y.checked_sub(1)?;
                let val = self.get(x, y)?;
                Some(Cell { x, y, val })
            }
            Direction::W => {
                let x = cell.x.checked_sub(1)?;
                let y = cell.y;
                let val = self.get(x, y)?;
                Some(Cell { x, y, val })
            }
            Direction::E => {
                let x = cell.x + 1;
                let y = cell.y;
                let val = self.get(x, y)?;
                Some(Cell { x, y, val })
            }
            Direction::SW => {
                let x = cell.x.checked_sub(1)?;
                let y = cell.y + 1;
                let val = self.get(x, y)?;
                Some(Cell { x, y, val })
            }
            Direction::S => {
                let y = cell.y + 1;
                let x = cell.x;
                let val = self.get(x, y)?;
                Some(Cell { x, y, val })
            }
            Direction::SE => {
                let x = cell.x + 1;
                let y = cell.y + 1;
                let val = self.get(x, y)?;
                Some(Cell { x, y, val })
            }
        }
    }

    pub fn area(
        &self,
        x1: usize,
        x2: usize,
        y1: usize,
        y2: usize,
    ) -> impl Iterator<Item = Cell> + '_ {
        (y1..=y2)
            .cartesian_product(x1..=x2)
            .filter_map(|(y, x)| self.get(x, y).map(|val| Cell { x, y, val }))
    }
}
