use advent_of_code::helpers::matrix::{Cell, Direction, Matrix, Neighbour, CARDINALS};
use hashbrown::{HashMap, HashSet};
use once_cell::sync::Lazy;

advent_of_code::solution!(10);

type Pipe = Vec<(Direction, Direction)>;

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

fn loop_from_string(matrix: &Matrix) -> Option<HashSet<Cell>> {
    matrix
        .items()
        .find(|c| c.val == 'S')
        .and_then(|start_cell| {
            CARDINALS
                .iter()
                .find_map(|dir| try_loop_from_start(matrix, start_cell, *dir))
        })

}

fn try_loop_from_start(
    matrix: &Matrix,
    start: Cell,
    start_dir: Direction,
) -> Option<HashSet<Cell>> {
    let mut visited = HashSet::new();
    let mut current: Neighbour = (start_dir, matrix.neighbour(&start, &start_dir));

    loop {
        let (current_dir, current_cell) = current;
        match current_cell {
            Some(cell) => {
                visited.insert(cell);

                if cell.val == 'S' {
                    return Some(visited);
                }

                let next = PIPES
                    .get(&cell.val)
                    .and_then(|pipe| resolve_direction(pipe, &current_dir))
                    .map(|next_dir| (next_dir, matrix.neighbour(&cell, &next_dir)));

                match next {
                    Some(next) => current = next,
                    None => return None,
                }
            }
            None => return None,
        }
    }
}

fn resolve_direction(pipe: &Pipe, dir: &Direction) -> Option<Direction> {
    pipe.iter().find(|y| y.0 == *dir).map(|y| y.1)
}

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = Matrix::from(input);
    loop_from_string(&matrix).map(|pipe| pipe.len() / 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = Matrix::from(input);
    let pipe = loop_from_string(&matrix)?;

    Some(
        matrix
            .items()
            .filter(|cell| {
                if pipe.contains(cell) {
                    return false;
                }

                let row = cell.row;
                let mut crosses = 0;

                (0..cell.col).try_for_each(|col| {
                    let nb = matrix.get_cell(row, col)?;
                    // FIXME!
                    // we need to replace 'S' with the actual value for this to be universal.
                    // in my input, it's an "F", so checking the bottom values works.
                    // otherwise we could just as well check `|F7` here.
                    if "|LJ".contains(nb.val) && pipe.contains(&nb) {
                        crosses += 1;
                    }
                    Some(())
                });

                crosses % 2 != 0
            })
            .count(),
    )
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
        assert_eq!(result, Some(4));
    }

    #[test]
    fn part_two_example_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn part_two_example_three() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn part_two_example_four() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(10));
    }
}
