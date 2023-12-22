use std::ops::RangeInclusive;

use hashbrown::HashMap;
use itertools::Itertools;

advent_of_code::solution!(22);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Brick {
    start: Point,
    end: Point,
    id: usize,
}

impl Brick {
    fn x_bounds(&self) -> RangeInclusive<u32> {
        self.start.x.min(self.end.x)..=self.start.x.max(self.end.x)
    }

    fn y_bounds(&self) -> RangeInclusive<u32> {
        self.start.y.min(self.end.y)..=self.start.y.max(self.end.y)
    }

    fn z_bounds(&self) -> RangeInclusive<u32> {
        self.start.z.min(self.end.z)..=self.start.z.max(self.end.z)
    }

    fn rests_on(&self, other: &Brick) -> bool {
        let self_x = self.x_bounds();
        let self_y = self.y_bounds();
        let self_z = self.z_bounds();
        let other_x = other.x_bounds();
        let other_y = other.y_bounds();
        let other_z = other.z_bounds();
        let x_overlap = self_x.start() <= other_x.end() && self_x.end() >= other_x.start();
        let y_overlap = self_y.start() <= other_y.end() && self_y.end() >= other_y.start();
        let adjacent_z = *self_z.start() == other_z.end() + 1;
        adjacent_z && (x_overlap && y_overlap)
    }
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .enumerate()
        .filter_map(|(id, l)| {
            let mut parts = l.split_ascii_whitespace();

            let (start_s, end_s) = parts
                .next()
                .and_then(|p| p.split_once('~'))
                .ok_or("could not parse point.")
                .ok()?;

            let start = Point::try_from(start_s).ok()?;
            let end = Point::try_from(end_s).ok()?;

            Some(Brick { id, start, end })
        })
        .collect()
}

fn simulate_falling(bricks: &mut [Brick]) {
    bricks.sort_by_key(|brick| brick.start.z);
    bricks.reverse();

    loop {
        let mut any_moved = false;

        for i in 0..bricks.len() {
            let mut is_resting = false;

            for j in 0..bricks.len() {
                let other = &bricks[j];

                if bricks[i].rests_on(other) || *bricks[i].z_bounds().start() == 1 {
                    is_resting = true;
                    break;
                }
            }

            if !is_resting {
                any_moved = true;
                bricks[i].end.z -= 1;
                bricks[i].start.z -= 1;
            }
        }

        if !any_moved {
            break;
        }
    }
}

fn build_graph(bricks: &[Brick]) -> HashMap<usize, Vec<usize>> {
    let mut graph = HashMap::new();

    for i in 0..bricks.len() {
        let mut dependencies = Vec::new();

        for j in 0..bricks.len() {
            if i != j && bricks[i].rests_on(&bricks[j]) {
                dependencies.push(bricks[j].id);
            }
        }

        graph.insert(bricks[i].id, dependencies);
    }

    graph
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut bricks = parse(input);
    simulate_falling(&mut bricks);

    let graph = build_graph(&bricks);

    Some(
        bricks
            .into_iter()
            .filter(|brick| {
                graph
                    .values()
                    .all(|val| !val.contains(&brick.id) || val.len() > 1)
            })
            .count(),
    )
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(7));
        assert_eq!(result, None);
    }
}
