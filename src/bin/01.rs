advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let first = l.chars().find(char::is_ascii_digit);
                let last = l.chars().rfind(char::is_ascii_digit);
                Some(first?.to_digit(10)? * 10 + last?.to_digit(10)?)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let first = (0..l.len()).find_map(|i| find_pattern(l, i, false))?;
                let last = (0..l.len()).find_map(|i| find_pattern(l, i, true))?;
                Some(first.to_digit(10)? * 10 + last.to_digit(10)?)
            })
            .sum(),
    )
}

const PATTERNS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_pattern(s: &str, idx: usize, reverse: bool) -> Option<char> {
    let i = if reverse { s.len() - idx - 1 } else { idx };

    let c = s.chars().nth(i);
    if c.is_some_and(|c| c.is_ascii_digit()) {
        return Some(c.unwrap());
    }

    for (j, pattern) in PATTERNS.into_iter().enumerate() {
        if reverse && (pattern.len() > i + 1) {
            continue;
        }

        let range = if reverse {
            i + 1 - pattern.len()..i + 1
        } else {
            i..i + pattern.len()
        };

        if s.get(range).is_some_and(|val| val == pattern) {
            return Some(char::from_digit((j + 1) as u32, 10).unwrap());
        }
    }

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
        assert_eq!(result, Some(142));
    }

    #[test]
    fn part_two_example() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
