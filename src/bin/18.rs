use advent_of_code::helpers::grid::Point;
use itertools::Itertools;

advent_of_code::solution!(18);

fn solve(instructions: &[(char, i64)]) -> i64 {
    let mut current = Point(0, 0);
    let mut border = 0;
    let mut shoelace = (0, 0);

    instructions.iter().for_each(|(dir, amount)| {
        let next = match dir {
            'U' => Point(current.0, current.1 - amount),
            'D' => Point(current.0, current.1 + amount),
            'L' => Point(current.0 - amount, current.1),
            'R' => Point(current.0 + amount, current.1),
            _ => unreachable!(),
        };

        border += amount;
        shoelace.0 += current.0 * next.1;
        shoelace.1 += current.1 * next.0;
        current = next;
    });

    let area = (shoelace.0.abs_diff(shoelace.1) / 2) as i64;
    let interior = 1 + area - border / 2;
    interior + border
}

pub fn part_one(input: &str) -> Option<i64> {
    let instructions = input
        .lines()
        .filter_map(|l| {
            let mut parts = l.split_ascii_whitespace();
            let dir = parts.next().and_then(|p| p.chars().next())?;
            let amount: i64 = parts.next().and_then(|p| p.parse().ok())?;
            Some((dir, amount))
        })
        .collect_vec();

    Some(solve(&instructions))
}

pub fn part_two(input: &str) -> Option<i64> {
    let instructions = input
        .lines()
        .filter_map(|l| {
            let (_, hex) = l.split_once('#')?;

            let amount = u32::from_str_radix(&hex[0..5], 16).ok()?;

            let dir = match &hex[5..].chars().next()? {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => unreachable!(),
            };

            Some((dir, amount as i64))
        })
        .collect_vec();

    Some(solve(&instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
