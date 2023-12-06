use itertools::Itertools;

advent_of_code::solution!(6);

fn simulate(times: &[f64], highscores: &[f64]) -> f64 {
    times
        .iter()
        .zip(highscores)
        .map(|(time, highscore)| {
            let x = time * time - 4.0 * highscore;
            let a = (time + x.sqrt()) / 2.0;
            let b = (time - x.sqrt()) / 2.0;
            a.ceil() - b.floor() - 1.0
        })
        .product()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines().filter_map(|l| {
        l.split_once(':').map(|l| {
            l.1.split_ascii_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect_vec()
        })
    });

    lines.next().and_then(|times| {
        lines
            .next()
            .map(|highscores| simulate(&times, &highscores) as u32)
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines().filter_map(|l| {
        l.split_once(':').and_then(|l| {
            String::from_iter(l.1.chars().filter(|x| !x.is_whitespace()))
                .parse()
                .ok()
        })
    });

    lines.next().and_then(|time| {
        lines
            .next()
            .map(|highscore| simulate(&[time], &[highscore]) as u32)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
