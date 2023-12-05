use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Range {
    length: usize,
    source_start: usize,
    destination_start: usize,
}

#[derive(Debug)]
struct Map {
    source: String,
    destination: String,
    index: Vec<Range>,
}

impl TryFrom<&str> for Map {
    type Error = &'static str;

    fn try_from(chunk: &str) -> Result<Self, Self::Error> {
        let mut source = None;
        let mut destination = None;
        let mut index = vec![];

        chunk.lines().try_for_each(|l| {
            if l.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) {
                let (from_s, to_s) = l.split_once(' ')?.0.split_once("-to-")?;
                source = Some(from_s);
                destination = Some(to_s);
            } else {
                let mut splits = l.split(' ');
                let destination_start = splits.next().and_then(|x| x.parse().ok())?;
                let source_start = splits.next().and_then(|x| x.parse().ok())?;
                let length = splits.next().and_then(|x| x.parse().ok())?;

                index.push(Range {
                    source_start,
                    destination_start,
                    length,
                });
            }

            Some(())
        });

        Ok(Self {
            source: source.ok_or("could not parse `from`.")?.to_string(),
            destination: destination.ok_or("could not parse `to`.")?.to_string(),
            index,
        })
    }
}

fn parse(input: &str) -> Option<(Vec<usize>, Vec<Map>)> {
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
        .filter_map(|chunk| Map::try_from(chunk).ok())
        .collect_vec();

    Some((seeds, maps))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (seeds, maps) = parse(input)?;

    seeds
        .into_iter()
        .map(|seed| {
            maps.iter().fold(seed, |acc, map| {
                let range = map.index
                    .iter()
                    .find(|range| {
                        acc > range.source_start && acc <= (range.source_start + range.length)
                    });

                match range {
                    Some(range) => {
                        acc - range.source_start + range.destination_start
                    },
                    None => acc
                }
            })
        })
        .min()
}

pub fn part_two(input: &str) -> Option<u32> {
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
        assert_eq!(result, None);
    }
}
