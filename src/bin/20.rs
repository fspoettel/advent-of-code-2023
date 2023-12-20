use hashbrown::HashMap;

advent_of_code::solution!(20, 1);

#[derive(Clone, Copy, Debug, PartialEq)]
enum PulseType {
    Low,
    High,
}

#[derive(Debug)]
enum NodeState<'a> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, PulseType>),
}

#[derive(Debug)]
struct Node<'a> {
    id: &'a str,
    destinations: Vec<&'a str>,
}

#[derive(Debug, Default)]
struct Nodes<'a> {
    data: HashMap<&'a str, Node<'a>>,
}

#[derive(Debug)]
struct QueueItem<'a>(&'a str, &'a str, PulseType);

#[derive(Debug, Default)]
struct States<'a> {
    data: HashMap<&'a str, NodeState<'a>>,
    queue: Vec<QueueItem<'a>>,
}

impl<'a> States<'a> {
    fn drain_queue(&mut self, nodes: &'a Nodes) -> (u64, u64) {
        let mut pulse_count_low = 0;
        let mut pulse_count_high = 0;

        let pulses: Vec<QueueItem> = self.queue.drain(0..).collect();

        for QueueItem(target, destination, pulse_type) in pulses {
            let (low, high) = self.send_pulse(&nodes, target, destination, pulse_type);
            pulse_count_low += low;
            pulse_count_high += high;
        }

        (pulse_count_low, pulse_count_high)
    }

    fn send_pulse(
        &mut self,
        nodes: &'a Nodes,
        sender: &'a str,
        target: &'a str,
        pulse_type: PulseType,
    ) -> (u64, u64) {
        if let Some(node) = nodes.data.get(target) {
            let node_state = self.data.get_mut(target).unwrap();

            match node_state {
                NodeState::Broadcast => {
                    for destination in &node.destinations {
                        self.queue.push(QueueItem(target, destination, pulse_type));
                    }
                }
                NodeState::FlipFlop(current) => {
                    if pulse_type == PulseType::Low {
                        *current = !*current;

                        let pulse = if *current {
                            PulseType::High
                        } else {
                            PulseType::Low
                        };

                        for destination in &node.destinations {
                            self.queue.push(QueueItem(target, destination, pulse));
                        }
                    }
                }
                NodeState::Conjunction(current) => {
                    let current_state = current.get_mut(sender).unwrap();

                    *current_state = pulse_type;

                    let pulse = if current.values().all(|v| *v == PulseType::High) {
                        PulseType::Low
                    } else {
                        PulseType::High
                    };

                    for destination in &node.destinations {
                        self.queue.push(QueueItem(target, destination, pulse));
                    }
                }
            };
        }

        if pulse_type == PulseType::Low {
            (1, 0)
        } else {
            (0, 1)
        }
    }
}

fn parse(input: &str) -> (Nodes, States) {
    let (nodes, mut states) = input
        .lines()
        .filter_map(|l| {
            l.split_once(" -> ").map(|(node_s, destination_s)| {
                let destinations: Vec<&str> = destination_s.split(", ").collect();

                let id = if node_s == "broadcaster" {
                    node_s
                } else {
                    &node_s[1..]
                };

                let node_state = if node_s == "broadcaster" {
                    NodeState::Broadcast
                } else if node_s.starts_with('%') {
                    NodeState::FlipFlop(false)
                } else {
                    NodeState::Conjunction(HashMap::new())
                };

                (Node { id, destinations }, node_state)
            })
        })
        .fold(
            (Nodes::default(), States::default()),
            |mut acc, (node, node_state)| {
                acc.1.data.insert(node.id, node_state);
                acc.0.data.insert(node.id, node);
                acc
            },
        );

    states.data.iter_mut().for_each(|(key, val)| {
        if let NodeState::Conjunction(state) = val {
            nodes
                .data
                .values()
                .filter(|n| n.destinations.contains(key))
                .for_each(|n| {
                    state.insert(n.id, PulseType::Low);
                })
        }
    });

    (nodes, states)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (nodes, mut states) = parse(input);

    let mut pulse_count_low = 0;
    let mut pulse_count_high = 0;

    for _ in 0..1000 {
        states
            .queue
            .push(QueueItem("button", "broadcaster", PulseType::Low));

        while !states.queue.is_empty() {
            let (low, high) = states.drain_queue(&nodes);
            pulse_count_low += low;
            pulse_count_high += high;
        }
    }

    Some(pulse_count_low * pulse_count_high)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn part_one_example_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11687500));
    }
}
