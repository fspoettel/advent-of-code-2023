use advent_of_code::helpers::matrix::{Cell, Direction, Matrix, CARDINALS};
use hashbrown::HashSet;

advent_of_code::solution!(23, 1);

#[derive(Clone)]
struct State {
    visited: HashSet<Cell<char>>,
    current: Cell,
    goal: Cell,
}

fn get_slope_direction(c: char) -> Option<Direction> {
    match c {
        '^' => Some(Direction::N),
        '>' => Some(Direction::E),
        'v' => Some(Direction::S),
        '<' => Some(Direction::W),
        _ => None,
    }
}

fn explore(matrix: &Matrix, state: &State, handle_slopes: bool) -> (Vec<State>, Vec<State>) {
    let slope_direction = get_slope_direction(state.current.val);

    let mut next_states = vec![];
    let mut valid_paths = vec![];

    matrix
        .neighbours(&state.current, &CARDINALS)
        .for_each(|(dir, point)| {
            let dir_valid = !handle_slopes
                || match slope_direction {
                    Some(s) => s == dir,
                    None => true,
                };

            if dir_valid {
                if let Some(point) = point {
                    if point.val != '#' && !state.visited.contains(&point) {
                        let mut next_state = state.clone();

                        next_state.current = point;
                        next_state.visited.insert(state.current);

                        if next_state.current == next_state.goal {
                            valid_paths.push(next_state);
                        } else {
                            next_states.push(next_state);
                        }
                    }
                }
            }
        });

    (next_states, valid_paths)
}

fn all_paths(matrix: &Matrix, state: State, handle_slopes: bool) -> usize {
    let mut stack = vec![state];
    let mut longest_path = 0;

    while let Some(current_state) = stack.pop() {
        let (next_states, valid_paths) = explore(matrix, &current_state, handle_slopes);

        for path in valid_paths {
            if path.visited.len() > longest_path {
                // println!("found next longest path: {}", path.visited.len());
                longest_path = path.visited.len();
            }
        }

        stack.extend(next_states);
    }

    longest_path
}

fn get_start_state(matrix: &Matrix) -> Option<State> {
    Some(State {
        current: matrix.get_cell(0, 1)?,
        goal: matrix.get_cell(matrix.rows - 1, matrix.cols - 2)?,
        visited: HashSet::new(),
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let matrix: Matrix<char> = Matrix::from(input);
    get_start_state(&matrix).map(|state| all_paths(&matrix, state, true))
}

// this is super slow but eventually prints the correct value.
// FIXME: refactor this to work on path segments rather than individual pounts.
pub fn part_two(input: &str) -> Option<usize> {
    let matrix: Matrix<char> = Matrix::from(input);
    get_start_state(&matrix).map(|state| all_paths(&matrix, state, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
