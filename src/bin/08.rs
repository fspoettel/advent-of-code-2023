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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
