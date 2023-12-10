use advent_of_code::helpers::matrix::{Cell, Direction, Matrix, Neighbour, CARDINALS};
use hashbrown::HashMap;
use once_cell::sync::Lazy;

advent_of_code::solution!(10);

type Pipe = Vec<(Direction, Direction)>;

fn resolve_direction(pipe: &Pipe, dir: &Direction) -> Option<Direction> {
    pipe.iter().find(|y| y.0 == *dir).map(|y| y.1)
}

static PIPES: Lazy<HashMap<char, Pipe>> = Lazy::new(|| {
    HashMap::from_iter([
        (
            '|',
            vec![(Direction::N, Direction::N), (Direction::S, Direction::S)],
        ),
        (
            '-',
            vec![(Direction::E, Direction::E), (Direction::W, Direction::W)],
        ),
        (
            'L',
            vec![(Direction::S, Direction::E), (Direction::W, Direction::N)],
        ),
        (
            'J',
            vec![(Direction::S, Direction::W), (Direction::E, Direction::N)],
        ),
        (
            '7',
            vec![(Direction::E, Direction::S), (Direction::N, Direction::W)],
        ),
        (
            'F',
            vec![(Direction::N, Direction::E), (Direction::W, Direction::S)],
        ),
        ('.', vec![]),
    ])
});

fn find_loop(matrix: &Matrix, start: Cell, start_dir: Direction) -> Option<Vec<Cell>> {
    let mut visited = vec![start];

    let mut current: Neighbour = (start_dir, matrix.neighbour(&start, &start_dir));

    loop {
        match current.1 {
            Some(cell) => {
                visited.push(cell);

                if cell.val == 'S' {
                    break;
                }

                let next = PIPES
                    .get(&cell.val)
                    .and_then(|pipe| resolve_direction(pipe, &current.0))
                    .map(|next_dir| (next_dir, matrix.neighbour(&cell, &next_dir)));

                if let Some(next) = next {
                    current = next;
                } else {
                    return None;
                }
            }
            None => {
                return None;
            }
        }
    }

    Some(visited)
}

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = Matrix::from(input);
    let start = matrix.items().find(|c| c.val == 'S').unwrap();

    let trail = CARDINALS
        .iter()
        .find_map(|dir| find_loop(&matrix, start, *dir))
        .unwrap();

    Some(trail.len() / 2)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn part_one_example_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn part_two_example_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        // assert_eq!(result, Some(4));
        assert_eq!(result, None)
    }

    #[test]
    fn part_two_example_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        // assert_eq!(result, Some(10));
        assert_eq!(result, None)
    }
}
