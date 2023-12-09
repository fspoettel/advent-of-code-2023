use itertools::Itertools;
use std::cmp::Ordering;

advent_of_code::solution!(7);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn identify(hand: &[usize]) -> Hand {
        let mut counts = hand.iter().fold(vec![0; 14], |mut acc, curr| {
            acc[*curr] += 1;
            acc
        });

        let jokers = counts.pop().unwrap();
        let max = counts.iter().max().unwrap();

        if jokers + max >= 5 {
            Hand::FiveOfKind
        } else if jokers + max >= 4 {
            Hand::FourOfKind
        } else if counts.iter().filter(|x| **x != 0).count() == 2 {
            Hand::FullHouse
        } else if jokers + max >= 3 {
            Hand::ThreeOfKind
        } else if (counts.iter().filter(|v| **v == 2).count() + jokers) >= 2 {
            Hand::TwoPair
        } else if jokers + max >= 2 {
            Hand::OnePair
        } else {
            Hand::HighCard
        }
    }
}

type Parsed = (Vec<usize>, Hand, u32);

fn sort_by_rank(a: &Parsed, b: &Parsed) -> Ordering {
    let by_rank = b.1.cmp(&a.1);
    if by_rank != std::cmp::Ordering::Equal {
        by_rank
    } else {
        let (a, b) = a.0.iter().zip(&b.0).find(|(a, b)| a != b).unwrap();
        b.cmp(a)
    }
}

fn solve(input: &str, allow_jokers: bool) -> u32 {
    input
        .lines()
        .filter_map(|l| {
            let (hand, bet) = l.split_once(' ')?;
            let bet = bet.parse::<u32>().ok()?;

            let hand = hand
                .chars()
                .map(|c| match c {
                    'A' => 0,
                    'K' => 1,
                    'Q' => 2,
                    'J' => {
                        if allow_jokers {
                            13
                        } else {
                            3
                        }
                    }
                    'T' => 4,
                    '9' => 5,
                    '8' => 6,
                    '7' => 7,
                    '6' => 8,
                    '5' => 9,
                    '4' => 10,
                    '3' => 11,
                    '2' => 12,
                    _ => unreachable!(),
                })
                .collect_vec();

            let identified = Hand::identify(&hand);
            Some((hand, identified, bet))
        })
        .sorted_unstable_by(sort_by_rank)
        .enumerate()
        .map(|(i, (_, _, bet))| (i as u32 + 1) * bet)
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
