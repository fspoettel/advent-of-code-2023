use advent_of_code::helpers::matrix::{Direction, Matrix};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = Matrix::from(input);

    let mut curr = String::new();

    Some(matrix.items().fold(0, |mut acc, cell| {
        if cell.val.is_ascii_digit() {
            curr.push(cell.val);
        }

        if !curr.is_empty() && (!cell.val.is_ascii_digit() || cell.x == matrix.w() - 1) {
            if matrix
                .area(
                    (cell.x - curr.len()).saturating_sub(1),
                    cell.x,
                    cell.y.saturating_sub(1),
                    cell.y + 1,
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
    let matrix = Matrix::from(input);

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
                .filter_map(|cell| matrix.walk_digits(cell))
                .collect();

            if nums.len() == 2 {
                acc += nums[0] * nums[1];
            }
        }

        acc
    });

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
