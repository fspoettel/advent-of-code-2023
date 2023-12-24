use itertools::Itertools;

advent_of_code::solution!(24);

#[derive(Clone, Debug)]
struct HailStone {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
}

impl HailStone {
    fn intersects_in_area(&self, other: &HailStone, area: (i64, i64)) -> bool {
        true
    }
}

fn parse_val(s: &str) -> Option<(i64, i64, i64)> {
    s.split(",")
        .filter_map(|s| s.trim().parse().ok())
        .collect_tuple()
}

fn parse(input: &str) -> Vec<HailStone> {
    input
        .lines()
        .filter_map(|l| {
            let (position_s, velocity_s) = l.split_once(" @ ")?;
            let position = parse_val(position_s)?;
            let velocity = parse_val(velocity_s)?;
            Some(HailStone { position, velocity })
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse(input)
            .into_iter()
            .combinations(2)
            .filter(|vals| vals[0].intersects_in_area(&vals[1], (200000000000000, 400000000000000)))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
