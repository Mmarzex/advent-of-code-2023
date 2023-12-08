use num::Integer;
use std::collections::HashMap;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let splits = input.trim_end().split("\n\n").collect::<Vec<&str>>();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in splits[1].lines() {
        let split_one = line.split(" = ").collect::<Vec<&str>>();
        let key = split_one[0];
        let split_two = split_one[1]
            .split(", ")
            .map(|l| l.trim_start_matches('(').trim_end_matches(')'))
            .collect::<Vec<&str>>();

        nodes.insert(key, (split_two[0], split_two[1]));
    }

    let instructions = splits[0].chars().cycle();

    let mut steps: u32 = 0;
    let mut key = "AAA";
    for instruction in instructions {
        key = match instruction {
            'L' => nodes.get(key).unwrap().0,
            'R' => nodes.get(key).unwrap().1,
            _ => unreachable!(),
        };
        steps += 1;
        if key == "ZZZ" {
            return Option::Some(steps);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let splits = input.trim_end().split("\n\n").collect::<Vec<&str>>();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in splits[1].lines() {
        let split_one = line.split(" = ").collect::<Vec<&str>>();
        let key = split_one[0];
        let split_two = split_one[1]
            .split(", ")
            .map(|l| l.trim_start_matches('(').trim_end_matches(')'))
            .collect::<Vec<&str>>();
        nodes.insert(key, (split_two[0], split_two[1]));
    }

    let cycle = splits[0].chars().collect::<Vec<char>>();

    let starting_keys = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| *k)
        .collect::<Vec<&str>>();
    let mut results: Vec<usize> = vec![];
    for starting_key in starting_keys {
        let mut steps: usize = 0;
        let mut key = starting_key;
        loop {
            let instruction = cycle[steps % cycle.len()];
            key = match instruction {
                'L' => nodes.get(key).unwrap().0,
                'R' => nodes.get(key).unwrap().1,
                _ => unreachable!(),
            };
            steps += 1;
            if key.ends_with('Z') {
                results.push(steps);
                break;
            }
        }
    }

    Option::Some(results.iter().fold(1, |a, b| a.lcm(b)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
