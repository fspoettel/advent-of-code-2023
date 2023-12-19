use hashbrown::HashMap;

advent_of_code::solution!(19);

struct Rule<'a> {
    matches: char,
    value: u32,
    comparator: char,
    destination_pass: &'a str,
    destination_deny: Option<&'a str>,
}

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn parse(input: &str) -> Option<(HashMap<&str, Workflow>, Vec<Part>)> {
    let (workflows_s, parts_s) = input.split_once("\n\n")?;

    let parts = parts_s
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                let parts: Vec<u32> = l[1..l.len() - 1]
                    .split(',')
                    .filter_map(|p| p.split_once('=').and_then(|(_, x)| x.parse().ok()))
                    .collect();
                Some(Part {
                    x: parts[0],
                    m: parts[1],
                    a: parts[2],
                    s: parts[3],
                })
            }
        })
        .collect();

    let workflows = workflows_s.lines().try_fold(HashMap::new(), |mut acc, l| {
        let (id, workflow_s) = l.split_once('{')?;

        let mut rules = vec![];

        let mut rules_s: Vec<&str> = workflow_s.split(',').collect();
        let destination_deny = rules_s.pop()?;

        for (i, rule) in rules_s.iter().enumerate() {
            let matches = rule.chars().next()?;
            let comparator = rule.chars().nth(1)?;
            let (value, destination_pass) = rule[2..].split_once(':')?;
            let value = value.parse().ok()?;

            rules.push(Rule {
                matches,
                comparator,
                value,
                destination_pass,
                destination_deny: if i == rules_s.len() - 1 {
                    Some(&destination_deny[..destination_deny.len() - 1])
                } else {
                    None
                },
            });
        }

        acc.insert(id, Workflow { rules });
        Some(acc)
    })?;

    Some((workflows, parts))
}

fn apply_rule(rule: &Rule, value: u32) -> bool {
    match rule.comparator {
        '>' => value > rule.value,
        '<' => value < rule.value,
        _ => unreachable!(),
    }
}

fn apply_workflow<'a: 'b, 'b>(workflow: &'a Workflow<'a>, part: &'a Part) -> &'b str {
    let mut ret = "R";

    for rule in &workflow.rules {
        let pass = match rule.matches {
            'x' => apply_rule(rule, part.x),
            'm' => apply_rule(rule, part.m),
            'a' => apply_rule(rule, part.a),
            's' => apply_rule(rule, part.s),
            _ => unreachable!(),
        };

        if pass {
            ret = rule.destination_pass;
            break;
        } else if rule.destination_deny.is_some() {
            ret = rule.destination_deny.unwrap();
            break;
        }
    }

    ret
}

pub fn part_one(input: &str) -> Option<u32> {
    let (workflows, parts) = parse(input)?;

    let sum = parts
        .into_iter()
        .filter(|part| {
            let mut current = "in";

            while current != "A" && current != "R" {
                current = apply_workflow(workflows.get(current).unwrap(), part);
            }

            current == "A"
        })
        .map(|part| part.x + part.m + part.a + part.s)
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
