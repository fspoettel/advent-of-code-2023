use itertools::Itertools;

advent_of_code::solution!(5);

struct Range {
    length: usize,
    source_start: usize,
    destination_start: usize,
}

fn parse_map(chunk: &str) -> Option<Vec<Range>> {
    let ranges: Vec<Range> = chunk
        .lines()
        .filter_map(|l| {
            if l.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                let mut splits = l.split(' ').filter_map(|x| x.parse::<usize>().ok());
                Some(Range {
                    destination_start: splits.next()?,
                    source_start: splits.next()?,
                    length: splits.next()?,
                })
            } else {
                None
            }
        })
        .collect_vec();

    if ranges.is_empty() {
        None
    } else {
        Some(ranges)
    }
}

fn parse(input: &str) -> Option<(Vec<usize>, Vec<Vec<Range>>)> {
    let seeds = input
        .lines()
        .next()?
        .split_once(':')?
        .1
        .split_ascii_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect_vec();

    let maps = input
        .split("\n\n")
        .skip(1)
        .filter_map(parse_map)
        .collect_vec();

    Some((seeds, maps))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (seeds, maps) = parse(input)?;
    seeds
        .into_iter()
        .map(|seed| {
            maps.iter().fold(seed, |acc, map| {
                let range = map.iter().find(|range| {
                    acc >= range.source_start && acc < (range.source_start + range.length)
                });

                match range {
                    Some(range) => acc - range.source_start + range.destination_start,
                    None => acc,
                }
            })
        })
        .min()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (seeds, maps) = parse(input)?;

    let ranges: Vec<(usize, usize)> = seeds
        .chunks(2)
        .map(|vals| {
            let x = vals[0] - 1;
            let y = x + vals[1];
            (x, y)
        })
        .collect_vec();

    for i in 0..usize::MAX {
        let start_value = maps.iter().rev().fold(i, |acc, map| {
            let range = map.iter().find(|range| {
                acc >= range.destination_start && acc < (range.destination_start + range.length)
            });

            match range {
                Some(range) => acc + range.source_start - range.destination_start,
                None => acc,
            }
        });

        if ranges
            .iter()
            .any(|r| start_value >= r.0 && start_value <= r.1)
        {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
