use itertools::Itertools;

advent_of_code::solution!(22);

#[allow(unused)]
#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl TryFrom<&str> for Point {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (x, y, z) = value
            .split(',')
            .filter_map(|x| x.parse().ok())
            .collect_tuple()
            .ok_or("Could not parse string")?;
        Ok(Point { x, y, z })
    }
}

#[allow(unused)]
#[derive(Debug)]
struct Brick<'a> {
    start: Point,
    end: Point,
    id: &'a str,
}

impl<'a> TryFrom<&'a str> for Brick<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut parts = value.split_ascii_whitespace();

        let (start_s, end_s) = parts
            .next()
            .and_then(|p| p.split_once('~'))
            .ok_or("could not parse point.")?;

        let start = Point::try_from(start_s)?;
        let end = Point::try_from(end_s)?;
        let id = parts.last().ok_or("could not parse id.")?;

        Ok(Brick { id, start, end })
    }
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .filter_map(|l| Brick::try_from(l).ok())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let bricks = parse(input);
    println!("{:?}", bricks);

    None
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
        // assert_eq!(result, Some(5));
        assert_eq!(result, None);
    }

    #[test]
    fn part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
