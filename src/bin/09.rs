advent_of_code::solution!(9);

fn resolve_sequence(sequence: &[i32], carry: i32) -> i32 {
    let diffs: Vec<i32> = sequence
        .windows(2)
        .map(|slice| slice[1] - slice[0])
        .collect();

    let carry = carry + diffs[diffs.len() - 1];

    if diffs.iter().any(|x| *x != 0) {
        resolve_sequence(&diffs, carry)
    } else {
        carry
    }
}

fn solve(input: &str, find_previous: bool) -> i32 {
    input
        .lines()
        .filter_map(|l| {
            let mut sequence: Vec<i32> = l
                .split_whitespace()
                .filter_map(|v| v.parse().ok())
                .collect();

            if find_previous {
                sequence.reverse();
            }

            let next_value = resolve_sequence(&sequence, *sequence.last()?);
            Some(next_value)
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(solve(input, false))
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(solve(input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
