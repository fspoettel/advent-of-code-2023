advent_of_code::solution!(2);

struct Game {
    r_max: u32,
    g_max: u32,
    b_max: u32,
}

fn parse_game(line: &str) -> Option<Game> {
    let mut game = Game {
        r_max: 0,
        g_max: 0,
        b_max: 0,
    };

    line.split_once(": ")?
        .1
        .split([',', ';'])
        .try_for_each(|roll_s| {
            let (count_s, color_s) = roll_s.trim().split_once(' ')?;
            let count = count_s.parse().ok()?;
            match color_s.as_bytes().first()? {
                b'r' => game.r_max = std::cmp::max(count, game.r_max),
                b'g' => game.g_max = std::cmp::max(count, game.g_max),
                b'b' => game.b_max = std::cmp::max(count, game.b_max),
                _ => unreachable!(),
            };
            Some(())
        });

    Some(game)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .enumerate()
            .filter_map(|(i, l)| {
                parse_game(l).and_then(|game| {
                    if game.r_max <= 12 && game.g_max <= 13 && game.b_max <= 14 {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| parse_game(l).map(|game| game.r_max * game.g_max * game.b_max))
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
