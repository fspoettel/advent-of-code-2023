use std::cmp::Ordering;

use hashbrown::HashMap;
use itertools::Itertools;

advent_of_code::solution!(7);

static CARD_RANK: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

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
    fn rank(hand: &str, allow_jokers: bool) -> usize {
        let hand = Hand::identify(hand, allow_jokers);
        HAND_RANK.iter().position(|x| *x == hand).unwrap()
    }

    fn identify(hand: &str, allow_jokers: bool) -> Hand {
        let mut counts = hand.chars().fold(HashMap::new(), |mut acc, curr| {
            *acc.entry(curr).or_default() += 1;
            acc
        });

        let jokers = if allow_jokers {
            *counts.get(&'J').unwrap_or(&0)
        } else {
            0
        };

        if allow_jokers {
            counts.remove_entry(&'J');
        }

        if jokers == 5 || counts.values().any(|v| *v + jokers >= 5) {
            Hand::FiveOfKind
        } else if counts.values().any(|v| *v + jokers >= 4) {
            Hand::FourOfKind
        } else if counts.len() == 2 {
            Hand::FullHouse
        } else if counts.values().any(|v| *v + jokers >= 3) {
            Hand::ThreeOfKind
        } else if (counts.values().filter(|v| **v == 2).count() as u8 + jokers) >= 2 {
            Hand::TwoPair
        } else if counts.values().any(|v| *v + jokers >= 2) {
            Hand::OnePair
        } else {
            Hand::HighCard
        }
    }
}

fn sort_by_rank(a: &str, b: &str, allow_jokers: bool) -> Ordering {
    let a_rank = Hand::rank(a, allow_jokers);
    let b_rank = Hand::rank(b, allow_jokers);

    let by_rank = b_rank.cmp(&a_rank);

    if by_rank != std::cmp::Ordering::Equal {
        by_rank
    } else {
        let (a, b) = a.chars().zip(b.chars()).find(|(a, b)| a != b).unwrap();

        if allow_jokers && (a == 'J' || b == 'J') {
            if a == 'J' {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }

        let a_pos = CARD_RANK.into_iter().position(|c| c == a);
        let b_pos = CARD_RANK.into_iter().position(|c| c == b);
        b_pos.cmp(&a_pos)
    }
}

fn solve(input: &str, allow_jokers: bool) -> u32 {
    input
        .lines()
        .filter_map(|l| {
            let (hand, bet) = l.split_once(' ')?;
            bet.parse::<u32>().ok().map(|bet| (hand, bet))
        })
        .sorted_by(|(a, _), (b, _)| sort_by_rank(a, b, allow_jokers))
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
