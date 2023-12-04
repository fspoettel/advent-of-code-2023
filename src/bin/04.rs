use hashbrown::HashSet;

advent_of_code::solution!(4);

type Card<'a> = (HashSet<&'a str>, HashSet<&'a str>);

fn parse_card(line: &str) -> Option<Card> {
    line.split_once(':')?.1.split_once('|').map(|(a, b)| {
        (
            a.split_whitespace().collect(),
            b.split_whitespace().collect(),
        )
    })
}

fn count_hits(card: &Card) -> usize {
    card.0.intersection(&card.1).count()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let hits = count_hits(&parse_card(l)?) as u32;
                Some(if hits < 2 { hits } else { 2_u32.pow(hits - 1) })
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let cards: Vec<Card> = input.lines().filter_map(parse_card).collect();
    let mut counts = vec![1; cards.len()];

    cards.iter().enumerate().for_each(|(i, card)| {
        let hits = count_hits(card);
        for j in (i + 1)..(i + 1 + hits) {
            counts[j] += counts[i];
        }
    });

    Some(counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
