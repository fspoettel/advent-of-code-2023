use std::{cmp::Ordering, collections::BinaryHeap};

use advent_of_code::helpers::matrix::{Cell, Direction, Matrix, CARDINALS};
use hashbrown::HashMap;

advent_of_code::solution!(17);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position(Cell<u32>, Direction, u32);

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    position: Position,
    cost: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path_one(matrix: &Matrix<u32>, start: Cell<u32>, end: Cell<u32>) -> Option<u32> {
    let mut dist: HashMap<Position, u32> = HashMap::new();
    let mut frontier: BinaryHeap<State> = BinaryHeap::new();

    for dir in CARDINALS {
        dist.insert(Position(start, dir, 0), 0);

        frontier.push(State {
            cost: 0,
            position: Position(start, dir, 0),
        });
    }

    while let Some(State { cost, position }) = frontier.pop() {
        let Position(current_cell, current_dir, current_steps) = position;

        if current_cell == end {
            return Some(cost);
        }

        if cost > *dist.get(&position).unwrap_or(&u32::MAX) {
            continue;
        }

        for neighbour in matrix.neighbours(&position.0, &CARDINALS) {
            let (neighbour_dir, neighbour_cell) = neighbour;

            let next_state = neighbour_cell.and_then(|neighbour_cell: Cell<u32>| {
                if neighbour_dir == current_dir.invert() {
                    None
                } else if neighbour_dir == current_dir && current_steps < 3 {
                    Some(State {
                        cost: cost + neighbour_cell.val,
                        position: Position(neighbour_cell, neighbour_dir, current_steps + 1),
                    })
                } else if neighbour_dir != current_dir {
                    Some(State {
                        cost: cost + neighbour_cell.val,
                        position: Position(neighbour_cell, neighbour_dir, 1),
                    })
                } else {
                    None
                }
            });

            if let Some(next_state) = next_state {
                if next_state.cost < *dist.get(&next_state.position).unwrap_or(&u32::MAX) {
                    frontier.push(next_state);
                    dist.insert(next_state.position, next_state.cost);
                }
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix: Matrix<u32> = Matrix::from(input);

    shortest_path_one(
        &matrix,
        matrix.get_cell(0, 0)?,
        matrix.get_cell(matrix.rows - 1, matrix.cols - 1)?,
    )
}

fn shortest_path_two(matrix: &Matrix<u32>, start: Cell<u32>, end: Cell<u32>) -> Option<u32> {
    let mut dist: HashMap<Position, u32> = HashMap::new();
    let mut frontier: BinaryHeap<State> = BinaryHeap::new();

    for dir in CARDINALS {
        dist.insert(Position(start, dir, 0), 0);

        frontier.push(State {
            cost: 0,
            position: Position(start, dir, 0),
        });
    }

    while let Some(State { cost, position }) = frontier.pop() {
        let Position(current_cell, current_dir, current_steps) = position;

        if current_cell == end {
            return Some(cost);
        }

        if cost > *dist.get(&position).unwrap_or(&u32::MAX) {
            continue;
        }

        for neighbour in matrix.neighbours(&position.0, &CARDINALS) {
            let (neighbour_dir, neighbour_cell) = neighbour;

            let next_state = neighbour_cell.and_then(|neighbour_cell: Cell<u32>| {
                if neighbour_dir == current_dir.invert() {
                    None
                } else if neighbour_dir == current_dir && current_steps < 10 {
                    Some(State {
                        cost: cost + neighbour_cell.val,
                        position: Position(neighbour_cell, neighbour_dir, current_steps + 1),
                    })
                } else if neighbour_dir != current_dir && current_steps >= 4 {
                    Some(State {
                        cost: cost + neighbour_cell.val,
                        position: Position(neighbour_cell, neighbour_dir, 1),
                    })
                } else {
                    None
                }
            });

            if let Some(next_state) = next_state {
                if next_state.cost < *dist.get(&next_state.position).unwrap_or(&u32::MAX) {
                    frontier.push(next_state);
                    dist.insert(next_state.position, next_state.cost);
                }
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix: Matrix<u32> = Matrix::from(input);

    shortest_path_two(
        &matrix,
        matrix.get_cell(0, 0)?,
        matrix.get_cell(matrix.rows - 1, matrix.cols - 1)?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
