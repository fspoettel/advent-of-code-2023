use advent_of_code::helpers::math::least_common_multiple;
use hashbrown::HashMap;

advent_of_code::solution!(8);

struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse(input: &str) -> Option<(Vec<char>, HashMap<&str, Node>)> {
    let (instruction_s, graph_s) = input.split_once("\n\n")?;
    let instructions = instruction_s.chars().collect();

    let graph = graph_s.lines().try_fold(HashMap::new(), |mut acc, curr| {
        let (id, connection_s) = curr.split_once(" = ")?;
        let (left_s, right_s) = connection_s.split_once(", ")?;
        acc.insert(
            id,
            Node {
                left: &left_s[1..],
                right: &right_s[..right_s.len() - 1],
            },
        );
        Some(acc)
    });

    graph.map(|graph| (instructions, graph))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (instructions, graph) = parse(input)?;

    let target = "ZZZ";
    let mut current = "AAA";
    let mut steps = 0;

    while current != target {
        let current_node = graph.get(current).unwrap();
        match instructions[steps % instructions.len()] {
            'L' => current = current_node.left,
            'R' => current = current_node.right,
            _ => unreachable!(),
        }

        steps += 1;
    }

    Some(steps)
}

// this works because:
// - no 'A' is hit after step 0.
// - one 'A' always hits the same 'Z'.
// - the 'A'->'Z' path is traversed once, then never again.
// - all 'Z'->'Z' cycles have the same length for the same node.
// - all 'A'->'Z' cycles have the same length as the following 'Z'..'Z' cycle.
pub fn part_two(input: &str) -> Option<usize> {
    let (instructions, graph) = parse(input)?;

    let mut steps = 0;

    let mut current_nodes: Vec<&str> = graph
        .keys()
        .filter(|key| key.ends_with('A'))
        .copied()
        .collect();

    let mut cycle_lengths = vec![0; current_nodes.len()];

    while cycle_lengths.iter().any(|x| *x == 0) {
        let instruction = instructions[steps % instructions.len()];

        for (i, current) in current_nodes.iter_mut().enumerate() {
            let current_node = graph.get(*current).unwrap();

            let next = match instruction {
                'L' => current_node.left,
                'R' => current_node.right,
                _ => unreachable!(),
            };

            if next.ends_with('Z') {
                cycle_lengths[i] = steps + 1;
            }

            *current = next;
        }

        steps += 1;
    }

    Some(least_common_multiple(&cycle_lengths))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }
}
