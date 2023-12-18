advent_of_code::solution!(13);

fn transpose(input: &str) -> Vec<String> {
    let lines: Vec<&str> = input.lines().collect();

    let mut columns: Vec<String> = vec![String::new(); lines.first().map(|x| x.len()).unwrap()];

    lines.iter().for_each(|l| {
        for (i, c) in l.chars().enumerate() {
            columns[i].push(c);
        }
    });

    columns
}

pub fn differs_by_max_or_less(a: &str, b: &str, max: usize) -> Option<usize> {
    let mut differences = 0;

    for (a, b) in a.chars().zip(b.chars()) {
        if a != b {
            differences += 1;
        }

        if differences > max {
            return None;
        }
    }

    Some(differences)
}

fn find_reflection(lines: &[(usize, &str)], repair_count: usize) -> Option<usize> {
    lines.windows(2).find_map(|window| {
        let mut repair_count = repair_count;
        let idx = window[0].0;

        if let Some(diff) = differs_by_max_or_less(window[0].1, window[0].1, repair_count) {
            repair_count -= diff;
        } else {
            return None;
        }

        let mut i = 0;

        while idx.checked_sub(i).is_some() && idx + i + 1 < lines.len() {
            if let Some(diff) =
                differs_by_max_or_less(lines[idx - i].1, lines[idx + 1 + i].1, repair_count)
            {
                repair_count -= diff;
            } else {
                return None;
            }

            i += 1;
        }

        if repair_count == 0 {
            Some(idx + 1)
        } else {
            None
        }
    })
}

fn solve(input: &str, repair_count: usize) -> usize {
    input
        .split("\n\n")
        .filter_map(|chunk| {
            let lines: Vec<(usize, &str)> = chunk.lines().enumerate().collect();

            // optimization: pass lines as reference to avoid re-allocating row strings.
            find_reflection(lines.as_slice(), repair_count)
                .map(|x| x * 100)
                .or_else(|| {
                    // transpose to cols. this allocates new string, which we pass by reference to `find_reflection`
                    let columns: Vec<(usize, String)> =
                        transpose(chunk).into_iter().enumerate().collect();

                    let columns: Vec<(usize, &str)> =
                        columns.iter().map(|s| (s.0, s.1.as_str())).collect();

                    find_reflection(&columns, repair_count)
                })
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, 0))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
