use itertools::Itertools;
use std::cmp::Ordering;

advent_of_code::solution!(7);

#[derive(PartialEq)]
enum Hand {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

static HAND_RANK: [Hand; 7] = [
    Hand::FiveOfKind,
    Hand::FourOfKind,
    Hand::FullHouse,
    Hand::ThreeOfKind,
    Hand::TwoPair,
    Hand::OnePair,
    Hand::HighCard,
];

impl Hand {
    fn rank(hand: &[usize]) -> usize {
        let hand = Hand::identify(hand);
        HAND_RANK.iter().position(|x| *x == hand).unwrap()
    }

    fn identify(hand: &[usize]) -> Hand {
        let mut counts = hand.iter().fold(vec![0; 14], |mut acc, curr| {
            acc[*curr] += 1;
            acc
        });

        let jokers = counts.pop().unwrap();

        if jokers == 5 || counts.iter().any(|v| *v + jokers >= 5) {
            Hand::FiveOfKind
        } else if counts.iter().any(|v| *v + jokers >= 4) {
            Hand::FourOfKind
        } else if counts.iter().filter(|x| **x != 0).count() == 2 {
            Hand::FullHouse
        } else if counts.iter().any(|v| *v + jokers >= 3) {
            Hand::ThreeOfKind
        } else if (counts.iter().filter(|v| **v == 2).count() + jokers) >= 2 {
            Hand::TwoPair
        } else if counts.iter().any(|v| *v + jokers >= 2) {
            Hand::OnePair
        } else {
            Hand::HighCard
        }
    }
}

fn sort_by_rank(a: &[usize], b: &[usize]) -> Ordering {
    let by_rank = Hand::rank(b).cmp(&Hand::rank(a));
    if by_rank != std::cmp::Ordering::Equal {
        by_rank
    } else {
        let (a, b) = a.iter().zip(b).find(|(a, b)| a != b).unwrap();
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

            Some((hand, bet))
        })
        .sorted_by(|(a, _), (b, _)| sort_by_rank(a, b))
        .enumerate()
        .map(|(i, (_, bet))| (i as u32 + 1) * bet)
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
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
