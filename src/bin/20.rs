use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module<'a> {
    FlipFlop {
        on: bool,
    },
    Conjunction {
        memory: HashMap<&'a str, PulseStrength>,
    },
    Broadcaster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PulseStrength {
    Low,
    High,
}

#[derive(Debug, Clone)]
struct Pulse<'a> {
    from: &'a str,
    to: &'a str,
    strength: PulseStrength,
}

impl<'a> Pulse<'a> {
    fn send(
        self,
        module_map: &mut HashMap<&'a str, Module>,
        destination_map: &HashMap<&'a str, Vec<&'a str>>,
        queue: &mut VecDeque<Pulse<'a>>,
    ) {
        let Some(module) = module_map.get_mut(self.to) else {
            return;
        };

        let send = match module {
            Module::FlipFlop { on } => match self.strength {
                PulseStrength::High => None,
                PulseStrength::Low => {
                    *on = !*on;
                    let strength = if *on {
                        PulseStrength::High
                    } else {
                        PulseStrength::Low
                    };
                    Some(strength)
                }
            },
            Module::Conjunction { memory } => {
                *memory.get_mut(self.from).unwrap() = self.strength;
                let strength = if memory
                    .values()
                    .all(|&strength| strength == PulseStrength::High)
                {
                    PulseStrength::Low
                } else {
                    PulseStrength::High
                };
                Some(strength)
            }
            Module::Broadcaster => Some(self.strength),
        };

        if let Some(strength) = send {
            for &to in destination_map.get(self.to).unwrap() {
                let pulse = Pulse {
                    from: self.to,
                    to,
                    strength,
                };
                queue.push_back(pulse);
            }
        };
    }
}

fn parse(input: &str) -> (HashMap<&str, Vec<&str>>, HashMap<&str, Module>) {
    let mut destination_map = HashMap::new();
    let mut module_map = HashMap::new();

    let line_iterator = input.trim_end().lines().map(|line| {
        let (lhs, rhs) = line.split_once(" -> ").unwrap();
        let outputs: Vec<&str> = rhs.split(", ").collect();
        let module = match &lhs[0..1] {
            "b" => Module::Broadcaster,
            "%" => Module::FlipFlop { on: false },
            "&" => Module::Conjunction {
                memory: HashMap::new(),
            },
            _ => unreachable!(),
        };
        let name = if module == Module::Broadcaster {
            lhs
        } else {
            &lhs[1..]
        };

        (name, module, outputs)
    });

    for (name, module, destinations) in line_iterator {
        destination_map.insert(name, destinations);
        module_map.insert(name, module);
    }

    for (source, destinations) in &destination_map {
        for destination in destinations {
            if let Some(Module::Conjunction { memory }) = module_map.get_mut(destination) {
                memory.insert(source, PulseStrength::Low);
            }
        }
    }

    (destination_map, module_map)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (destination_map, mut module_map) = parse(input);

    let mut low = 0;
    let mut high = 0;

    for _ in 0..1_000 {
        let mut queue = VecDeque::new();
        queue.push_back(Pulse {
            from: "button",
            to: "broadcaster",
            strength: PulseStrength::Low,
        });
        while let Some(pulse) = queue.pop_front() {
            match pulse.strength {
                PulseStrength::Low => low += 1,
                PulseStrength::High => high += 1,
            }
            pulse.send(&mut module_map, &destination_map, &mut queue)
        }
    }

    Option::Some(low * high)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (destination_map, mut module_map) = parse(input);

    let (before_rx, _) = destination_map
        .iter()
        .find(|(_, destinations)| destinations.contains(&"rx"))
        .unwrap();

    let Module::Conjunction { memory } = module_map.get(before_rx).unwrap() else {
        unreachable!()
    };

    let mut cache: HashMap<&str, Option<usize>> = memory.keys().map(|&name| (name, None)).collect();

    for idx in 1.. {
        let mut queue = VecDeque::new();
        queue.push_back(Pulse {
            from: "button",
            to: "broadcaster",
            strength: PulseStrength::Low,
        });

        while let Some(pulse) = queue.pop_front() {
            if pulse.to == *before_rx && pulse.strength == PulseStrength::High {
                *cache.get_mut(pulse.from).unwrap() = Some(idx);

                if cache.values().all(|idx| idx.is_some()) {
                    return Option::Some(
                        cache
                            .values()
                            .map(|idx| idx.unwrap())
                            .fold(1, |acc, curr| lcm(acc, curr)),
                    );
                }
            }

            pulse.send(&mut module_map, &destination_map, &mut queue);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
