use advent_of_code::helpers::matrix::Matrix;
use hashbrown::HashSet;

advent_of_code::solution!(14);

fn apply_cycle(matrix: &mut Matrix) {
    matrix.cells.iter_mut().for_each(|row| {
        let mut latest_pos = 0;
        for i in 0..row.len() {
            let c = row[i];
            if c == 'O' {
                row.swap(i, latest_pos);
                latest_pos += 1;
            } else if c == '#' {
                latest_pos = i + 1;
            }
        }
    });
}

fn count_load(matrix: &Matrix) -> usize {
    let mut total = 0;

    for row in matrix.cells.iter() {
        for (i, &c) in row.iter().rev().enumerate() {
            if c == 'O' {
                total += i + 1;
            }
        }
    }

    total
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut matrix = Matrix::from(input);
    matrix.transpose();
    apply_cycle(&mut matrix);
    Some(count_load(&matrix))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut matrix = Matrix::from(input);
    matrix.transpose();

    let mut seen = HashSet::new();

    let mut i = 0;

    let mut cycle_start_index: usize = 0;
    let mut cycle_start: Option<Vec<Vec<char>>> = None;

    while i < 1_000_000_000 {
        apply_cycle(&mut matrix);
        matrix.rotate_counterclockwise();

        apply_cycle(&mut matrix);
        matrix.rotate_counterclockwise();

        apply_cycle(&mut matrix);
        matrix.rotate_counterclockwise();

        apply_cycle(&mut matrix);

        if seen.contains(&matrix.cells) {
            matrix.rotate_counterclockwise();

            if cycle_start.as_ref().is_some_and(|x| x == &matrix.cells) {
                let cycle_length = i - cycle_start_index;
                i += 1 + ((1_000_000_000 - i) / cycle_length) * cycle_length;
                continue;
            } else if cycle_start.as_ref().is_none() {
                cycle_start = Some(matrix.cells.clone());
                cycle_start_index = i;
            }
        } else {
            seen.insert(matrix.cells.clone());
            matrix.rotate_counterclockwise();
        }

        i += 1;
    }

    Some(count_load(&matrix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
