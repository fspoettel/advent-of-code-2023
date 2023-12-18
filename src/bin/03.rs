use std::collections::VecDeque;

use advent_of_code::helpers::matrix::{Cell, Direction, Matrix};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix: Matrix<char> = Matrix::from(input);

    let mut curr = String::new();

    Some(matrix.items().fold(0, |mut acc, cell| {
        let is_digit = cell.val.is_ascii_digit();

        if is_digit {
            curr.push(cell.val);
        }

        if (!is_digit || cell.point.col == matrix.cols - 1) && !curr.is_empty() {
            if matrix
                .area(
                    (cell.point.col - curr.len()).saturating_sub(1),
                    cell.point.col,
                    cell.point.row.saturating_sub(1),
                    cell.point.row + 1,
                )
                .any(|cell| !cell.val.is_ascii_digit() && cell.val != '.')
            {
                acc += curr.parse::<u32>().unwrap();
            }

            curr = String::new();
        }

        acc
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix: Matrix<char> = Matrix::from(input);

    let sum = matrix.items().fold(0, |mut acc, cell| {
        if cell.val == '*' {
            let mut neighbours = vec![
                matrix.neighbour(&cell, &Direction::W),
                matrix.neighbour(&cell, &Direction::E),
            ];

            let n = matrix.neighbour(&cell, &Direction::N);
            if n.as_ref().is_some_and(|cell| cell.val.is_ascii_digit()) {
                neighbours.push(n);
            } else {
                neighbours.push(matrix.neighbour(&cell, &Direction::NE));
                neighbours.push(matrix.neighbour(&cell, &Direction::NW));
            }

            let s = matrix.neighbour(&cell, &Direction::S);
            if s.as_ref().is_some_and(|cell| cell.val.is_ascii_digit()) {
                neighbours.push(s);
            } else {
                neighbours.push(matrix.neighbour(&cell, &Direction::SE));
                neighbours.push(matrix.neighbour(&cell, &Direction::SW));
            }

            let nums: Vec<u32> = neighbours
                .into_iter()
                .filter_map(|cell| walk_digits(&matrix, cell))
                .collect();

            if nums.len() == 2 {
                acc += nums[0] * nums[1];
            }
        }

        acc
    });

    Some(sum)
}

fn walk_digits(matrix: &Matrix<char>, cell: Option<Cell<char>>) -> Option<u32> {
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
                .point
                .col
                .checked_sub(i)
                .and_then(|col| matrix.get(cell.point.row, col))
                .unwrap_or('.');

            if c.is_ascii_digit() {
                curr.push_front(c);
            } else {
                walk_left = false;
            }
        }

        if walk_right {
            let c = matrix
                .get(cell.point.row, cell.point.col + i)
                .unwrap_or('.');
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
