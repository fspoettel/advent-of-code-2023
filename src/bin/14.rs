use advent_of_code::helpers::strings::transpose;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut total = 0;
    // let mut output: Vec<String> = vec![];

    for col in transpose(input) {
        let mut latest_pos = 0;
        let mut chars: Vec<char> = col.chars().collect();

        for i in 0..chars.len() {
            let c = chars[i];
            if c == 'O' {
                chars.swap(i, latest_pos);
                latest_pos += 1;
                total += chars.len() - latest_pos + 1;
            } else if c == '#' {
                latest_pos = i + 1;
            }
        }

        // output.push(String::from_iter(chars));
    }

    Some(total)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
