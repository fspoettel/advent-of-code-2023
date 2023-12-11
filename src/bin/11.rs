use advent_of_code::helpers::grid::Point;
use hashbrown::HashSet;
use itertools::Itertools;

advent_of_code::solution!(11);

fn solve(input: &str, scaling_factor: i64) -> u64 {
    let scaling_factor = scaling_factor - 1;

    let mut x_idx: HashSet<i64> = HashSet::new();
    let mut y_idx: HashSet<i64> = HashSet::new();
    let mut galaxies: Vec<Point<i64>> = vec![];

    input.lines().enumerate().for_each(|(y, l)| {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                x_idx.insert(x as i64);
                y_idx.insert(y as i64);
                galaxies.push(Point(x as i64, y as i64));
            }
        }
    });

    galaxies
        .iter()
        .map(|Point(x, y)| {
            let x_scaling = x - x_idx.iter().filter(|&other_x| other_x < x).count() as i64;
            let y_scaling = y - y_idx.iter().filter(|&other_y| other_y < y).count() as i64;
            Point(
                x + (scaling_factor * x_scaling),
                y + (scaling_factor * y_scaling),
            )
        })
        .combinations(2)
        .map(|pair| pair[0].distance(&pair[1]))
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 1_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 2);
        assert_eq!(result, 374);
    }

    #[test]
    fn example_two() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn example_three() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, 8410);
    }
}
