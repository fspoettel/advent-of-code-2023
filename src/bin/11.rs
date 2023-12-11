use hashbrown::HashSet;
use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(Debug)]
struct Pos(i64, i64);

impl Pos {
    fn distance(&self, other: &Pos) -> u64 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

fn solve(input: &str, scaling_factor: i64) -> u64 {
    let scaling_factor = scaling_factor - 1;

    let mut x_idx: HashSet<i64> = HashSet::new();
    let mut y_idx: HashSet<i64> = HashSet::new();
    let mut unscaled: Vec<Pos> = vec![];

    input.lines().enumerate().for_each(|(y, l)| {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                x_idx.insert(x as i64);
                y_idx.insert(y as i64);
                unscaled.push(Pos(x as i64, y as i64));
            }
        }
    });

    let scaled: Vec<Pos> = unscaled
        .iter()
        .map(|pos| {
            let x_scaling = pos.0 - x_idx.iter().filter(|&&x| x < pos.0).count() as i64;
            let y_scaling = pos.1 - y_idx.iter().filter(|&&y| y < pos.1).count() as i64;
            Pos(
                pos.0 + (scaling_factor * x_scaling),
                pos.1 + (scaling_factor * y_scaling),
            )
        })
        .collect();

    scaled
        .iter()
        .combinations(2)
        .map(|pair| pair[0].distance(pair[1]))
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
