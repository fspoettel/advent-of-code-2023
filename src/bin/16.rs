use advent_of_code::helpers::matrix::{Cell, Direction, Matrix, Point};
use hashbrown::HashSet;

advent_of_code::solution!(16);

#[derive(PartialEq, Eq, Hash)]
struct BeamLog(Cell, Direction);

struct Beam(Cell, Direction);

fn get_beam_directions(cell: &Cell, current_dir: Direction) -> Vec<Direction> {
    match cell.val {
        '.' => vec![current_dir],
        '/' => match current_dir {
            Direction::N => vec![Direction::E],
            Direction::E => vec![Direction::N],
            Direction::S => vec![Direction::W],
            Direction::W => vec![Direction::S],
            _ => unreachable!(),
        },
        '\\' => match current_dir {
            Direction::N => vec![Direction::W],
            Direction::E => vec![Direction::S],
            Direction::S => vec![Direction::E],
            Direction::W => vec![Direction::N],
            _ => unreachable!(),
        },
        '-' => match current_dir {
            Direction::E | Direction::W => vec![current_dir],
            Direction::N | Direction::S => vec![Direction::E, Direction::W],
            _ => unreachable!(),
        },
        '|' => match current_dir {
            Direction::E | Direction::W => vec![Direction::S, Direction::N],
            Direction::N | Direction::S => vec![current_dir],
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn trace_beam(matrix: &Matrix, cell: Cell, direction: Direction) -> usize {
    let mut beams: Vec<Beam> = get_beam_directions(&cell, direction)
        .into_iter()
        .map(|dir| Beam(cell, dir))
        .collect();

    let mut visited: HashSet<BeamLog> = HashSet::new();
    visited.insert(BeamLog(cell, direction));

    while !beams.is_empty() {
        let mut next_beams: Vec<Beam> = vec![];

        for beam in beams.iter() {
            let next = matrix.neighbour(&beam.0, &beam.1);

            if let Some(next) = next {
                let log = BeamLog(next, beam.1);
                if !visited.contains(&log) {
                    visited.insert(log);

                    next_beams.extend(
                        get_beam_directions(&next, beam.1)
                            .into_iter()
                            .map(|dir| Beam(next, dir)),
                    );
                }
            }
        }

        beams = next_beams;
    }

    let mut set: HashSet<Point> = HashSet::new();

    visited.into_iter().for_each(|p| {
        set.insert(p.0.point);
    });

    set.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = Matrix::from(input);
    matrix
        .get_cell(0, 0)
        .map(|p| trace_beam(&matrix, p, Direction::E))
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = Matrix::from(input);

    (0..matrix.cols)
        .flat_map(|x| {
            [
                trace_beam(&matrix, matrix.get_cell(0, x).unwrap(), Direction::S),
                trace_beam(
                    &matrix,
                    matrix.get_cell(matrix.rows - 1, x).unwrap(),
                    Direction::N,
                ),
            ]
        })
        .chain((0..matrix.rows).flat_map(|y| {
            [
                trace_beam(&matrix, matrix.get_cell(y, 0).unwrap(), Direction::E),
                trace_beam(
                    &matrix,
                    matrix.get_cell(y, matrix.rows - 1).unwrap(),
                    Direction::W,
                ),
            ]
        }))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
