advent_of_code::solution!(2);

struct Roll {
    r: u32,
    g: u32,
    b: u32,
}

struct Game {
    id: u32,
    rolls: Vec<Roll>,
}

fn parse_game(line: &str) -> Option<Game> {
    let (id_s, rolls_s) = line.split_once(": ")?;

    let id = id_s.split_once(' ')?.1.parse().ok()?;

    let rolls = rolls_s
        .split("; ")
        .map(|rolls_s| {
            let mut roll = Roll { r: 0, g: 0, b: 0 };

            rolls_s.split(", ").try_for_each(|colors_str| {
                let (roll_s, color_s) = colors_str.split_once(' ')?;
                let count = roll_s.parse().ok()?;
                match color_s.chars().next()? {
                    'r' => roll.r = count,
                    'g' => roll.g = count,
                    'b' => roll.b = count,
                    _ => unreachable!(),
                };
                Some(())
            });

            roll
        })
        .collect();

    Some(Game { id, rolls })
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let game = parse_game(l)?;
                if game
                    .rolls
                    .iter()
                    .all(|round| round.r <= 14 && round.g <= 13 && round.b <= 12)
                {
                    Some(game.id)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let bag = parse_game(l)?.rolls.iter().fold(
                    Roll { r: 0, g: 0, b: 0 },
                    |mut bag, &Roll { r, g, b }| {
                        bag.r = std::cmp::max(bag.r, r);
                        bag.g = std::cmp::max(bag.g, g);
                        bag.b = std::cmp::max(bag.b, b);
                        bag
                    },
                );
                Some(bag.r * bag.g * bag.b)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
