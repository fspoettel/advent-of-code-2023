use std::collections::VecDeque;

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
}

impl From<&str> for Matrix {
    fn from(s: &str) -> Self {
        Self {
            cells: s
                .lines()
                .filter_map(|l| {
                    if !l.is_empty() {
                        Some(l.chars().collect())
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}

impl Matrix {
    pub fn w(&self) -> usize {
        self.cells[0].len()
    }

    pub fn h(&self) -> usize {
        self.cells.len()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        self.cells.get(y).and_then(|l| l.get(x).copied())
    }

    pub fn items(&self) -> impl Iterator<Item = Cell> + '_ {
        (0..self.h())
            .cartesian_product(0..self.w())
            .map(|(y, x)| Cell {
                val: self.get(x, y).unwrap(),
                x,
                y,
            })
    }

    pub fn neighbour(&self, cell: &Cell, dir: &Direction) -> Option<Cell> {
        match dir {
            Direction::NW => {
                let y = cell.y.checked_sub(1)?;
                let x = cell.x.checked_sub(1)?;
                let val = self.get(x, y)?;
                Some(Cell { val, x, y })
            }
            Direction::N => {
                let x = cell.x;
                let y = cell.y.checked_sub(1)?;
                let val = self.get(x, y)?;
                Some(Cell { val, x, y })
            }
            Direction::NE => {
                let x = cell.x + 1;
                let y = cell.y.checked_sub(1)?;
                let val = self.get(x, y)?;
                Some(Cell { val, x, y })
            }
            Direction::W => {
                let x = cell.x.checked_sub(1)?;
                let y = cell.y;
                let val = self.get(x, y)?;
                Some(Cell { val, x, y })
            }
            Direction::E => {
                let x = cell.x + 1;
                let y = cell.y;
                let val = self.get(x, y)?;
                Some(Cell { val, x, y })
            }
            Direction::SW => {
                let x = cell.x.checked_sub(1)?;
                let y = cell.y + 1;
                let val = self.get(x, y)?;
                Some(Cell { val, x, y })
            }
            Direction::S => {
                let y = cell.y + 1;
                let x = cell.x;
                let val = self.get(x, y)?;
                Some(Cell { val, x, y })
            }
            Direction::SE => {
                let x = cell.x + 1;
                let y = cell.y + 1;
                let val = self.get(x, y)?;
                Some(Cell { val, x, y })
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
            .filter_map(|(y, x)| self.get(x, y).map(|val| Cell { val, x, y }))
    }

    pub fn walk_digits(&self, cell: Option<Cell>) -> Option<u32> {
        let cell = cell?;
        if !cell.val.is_ascii_digit() {
            return None;
        }

        let mut curr = VecDeque::from([cell.val]);
        let mut i = 1;
        let mut walk_left = true;
        let mut walk_right = true;

        while walk_left || walk_right {
            if walk_left {
                let c = cell
                    .x
                    .checked_sub(i)
                    .and_then(|x| self.get(x, cell.y))
                    .unwrap_or('.');

                if c.is_ascii_digit() {
                    curr.push_front(c);
                } else {
                    walk_left = false;
                }
            }

            if walk_right {
                let c = self.get(cell.x + i, cell.y).unwrap_or('.');
                if c.is_ascii_digit() {
                    curr.push_back(c);
                } else {
                    walk_right = false;
                }
            }

            i += 1;
        }

        if !curr.is_empty() {
            String::from_iter(curr.iter()).parse().ok()
        } else {
            None
        }
    }
}
