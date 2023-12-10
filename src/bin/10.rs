use advent_of_code::helpers::matrix::{Direction, Matrix, Neighbour};
use hashbrown::HashMap;
use itertools::Itertools;
use once_cell::sync::Lazy;

advent_of_code::solution!(10);

struct Pipe {
    data: Vec<(Direction, Direction)>,
}

impl Pipe {
    fn resolve_direction(&self, dir: &Direction) -> Option<Direction> {
        self.data.iter().find(|y| y.0 == *dir).map(|y| y.1)
    }
}

static PIPES: Lazy<HashMap<char, Pipe>> = Lazy::new(|| {
    HashMap::from_iter([
        (
            '|',
            Pipe {
                data: vec![(Direction::N, Direction::N), (Direction::S, Direction::S)],
            },
        ),
        (
            '-',
            Pipe {
                data: vec![(Direction::E, Direction::E), (Direction::W, Direction::W)],
            },
        ),
        (
            'L',
            Pipe {
                data: vec![(Direction::S, Direction::E), (Direction::W, Direction::N)],
            },
        ),
        (
            'J',
            Pipe {
                data: vec![(Direction::S, Direction::W), (Direction::E, Direction::N)],
            },
        ),
        (
            '7',
            Pipe {
                data: vec![(Direction::E, Direction::S), (Direction::N, Direction::W)],
            },
        ),
        (
            'F',
            Pipe {
                data: vec![(Direction::N, Direction::E), (Direction::W, Direction::S)],
            },
        ),
        ('.', Pipe { data: vec![] }),
    ])
});

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = Matrix::from(input);
    let start = matrix.items().find(|c| c.val == 'S').unwrap();

    let mut i = 1;
    let mut trails: Vec<Neighbour> = matrix.all_neighbours(start, false).collect_vec();

    loop {
        let mut found = false;

        for trail in trails.iter_mut() {
            if let Some(cell) = &trail.1 {
                if cell.val == 'S' {
                    found = true;
                }

                let next = PIPES
                    .get(&cell.val)
                    .and_then(|pipe| pipe.resolve_direction(&trail.0))
                    .map(|dir| (dir, matrix.neighbour(cell, &dir)));

                if let Some(next) = next {
                    *trail = next;
                }
            }
        }

        if found {
            break;
        } else {
            i += 1;
        }
    }

    Some(i / 2)
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
