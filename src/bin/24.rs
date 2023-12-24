use itertools::Itertools;

advent_of_code::solution!(24, 1);

#[derive(Clone, Debug)]
struct HailStone {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
}

impl HailStone {
    fn intersect(&self, other: &HailStone) -> Option<(f64, f64)> {
        let x1 = self.position.0 as f64;
        let y1 = self.position.1 as f64;

        let x2 = x1 + self.velocity.0 as f64;
        let y2 = y1 + self.velocity.1 as f64;

        let x3 = other.position.0 as f64;
        let y3 = other.position.1 as f64;

        let x4 = x3 + other.velocity.0 as f64;
        let y4 = y3 + other.velocity.1 as f64;

        if (x1 == x2 && y1 == y2) || (x3 == x4 && y3 == y4) {
            return None;
        }

        let denominator = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);

        if denominator == 0.0 {
            return None;
        }

        let ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denominator;
        let ub = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / denominator;

        if ua <= 0.0 || ub <= 0.0 {
            return None;
        }

        let x = x1 + ua * (x2 - x1);
        let y = y1 + ua * (y2 - y1);

        Some((x, y))
    }
}

fn parse_val(s: &str) -> Option<(i64, i64, i64)> {
    s.split(',')
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

fn solve(input: &str, min: f64, max: f64) -> Option<usize> {
    Some(
        parse(input)
            .into_iter()
            .combinations(2)
            .filter(|vals| {
                let point = vals[0].intersect(&vals[1]);
                point.is_some_and(|(x, y)| x >= min && x <= max && y >= min && y <= max)
            })
            .count(),
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 200000000000000.0, 400000000000000.0)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = solve(
            &advent_of_code::template::read_file("examples", DAY),
            7.0,
            27.0,
        );
        assert_eq!(result, Some(2));
    }

    #[test]
    fn part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
