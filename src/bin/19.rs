use std::ops::RangeInclusive;

use hashbrown::HashMap;

advent_of_code::solution!(19);

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
}

struct Rule<'a> {
    matches: char,
    value: u64,
    comparator: char,
    destination_pass: &'a str,
    destination_deny: Option<&'a str>,
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Clone, Default)]
struct PartRange {
    x: Vec<RangeInclusive<u64>>,
    m: Vec<RangeInclusive<u64>>,
    a: Vec<RangeInclusive<u64>>,
    s: Vec<RangeInclusive<u64>>,
}

#[derive(Clone)]
struct State<'a> {
    range: PartRange,
    path: Vec<&'a str>,
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        Self {
            range: Default::default(),
            path: vec!["in"],
        }
    }
}

fn parse(input: &str) -> Option<(HashMap<&str, Workflow>, Vec<Part>)> {
    let (workflows_s, parts_s) = input.split_once("\n\n")?;

    let parts = parts_s
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                let parts: Vec<u64> = l[1..l.len() - 1]
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

fn apply_rule(rule: &Rule, value: u64) -> bool {
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

pub fn part_one(input: &str) -> Option<u64> {
    parse(input).map(|(workflows, parts)| {
        parts
            .into_iter()
            .filter_map(|part| {
                let mut current = "in";

                while current != "A" && current != "R" {
                    current = apply_workflow(workflows.get(current).unwrap(), &part);
                }

                if current == "A" {
                    Some(part.x + part.m + part.a + part.s)
                } else {
                    None
                }
            })
            .sum()
    })
}

fn resolve_range(rule: &Rule, should_pass: bool) -> (char, RangeInclusive<u64>) {
    let floor = 1;
    let ceil = 4000;

    let range = match rule.comparator {
        '>' => {
            if should_pass {
                (rule.value + 1)..=ceil
            } else {
                floor..=rule.value
            }
        }
        '<' => {
            if should_pass {
                floor..=(rule.value - 1)
            } else {
                rule.value..=ceil
            }
        }
        _ => unreachable!(),
    };

    (rule.matches, range)
}

fn update_ranges(state: &mut State, range: (char, RangeInclusive<u64>)) {
    match range.0 {
        'x' => state.range.x.push(range.1),
        'm' => state.range.m.push(range.1),
        'a' => state.range.a.push(range.1),
        's' => state.range.s.push(range.1),
        _ => unreachable!(),
    }
}

fn resolve_state<'a>(workflows: &'a HashMap<&str, Workflow>, state: State<'a>) -> Vec<State<'a>> {
    let mut next_states = vec![];

    let current = state.path.last().expect("empty paths are not allowed.");

    // we are at the end.
    if *current == "A" || *current == "R" {
        return vec![state];
    }

    let workflow = workflows.get(current).expect("undefined workflow");

    let mut current_state = state.clone();

    for rule in &workflow.rules {
        let mut pass_state = current_state.clone();
        update_ranges(&mut pass_state, resolve_range(rule, true));
        pass_state.path.push(rule.destination_pass);
        next_states.extend(resolve_state(workflows, pass_state));

        let reject_range = resolve_range(rule, false);
        update_ranges(&mut current_state, reject_range);
        if let Some(deny) = rule.destination_deny {
            current_state.path.push(deny);
            next_states.extend(resolve_state(workflows, current_state.clone()));
        }
    }

    next_states
}

fn collapse_range(range: &[RangeInclusive<u64>]) -> u64 {
    let range = range.iter().fold((1, 4000), |mut acc, curr| {
        if curr.start() > &acc.0 {
            acc.0 = *curr.start()
        }
        if curr.end() < &acc.1 {
            acc.1 = *curr.end()
        }
        acc
    });

    1 + range.1 - range.0
}

fn count_ranges(ranges: &PartRange) -> u64 {
    collapse_range(&ranges.x)
        * collapse_range(&ranges.m)
        * collapse_range(&ranges.a)
        * collapse_range(&ranges.s)
}

pub fn part_two(input: &str) -> Option<u64> {
    parse(input).map(|(workflows, _)| {
        resolve_state(&workflows, State::default())
            .into_iter()
            .flat_map(|state| {
                if state.path.last().unwrap() == &"A" {
                    Some(count_ranges(&state.range))
                } else {
                    None
                }
            })
            .sum()
    })
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
        assert_eq!(result, Some(167409079868000));
    }
}
