use std::collections::HashMap;

use rustworkx_core::petgraph::graph::UnGraph;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::Result;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<usize> {
    let mut node_translate = HashMap::<&str, u32>::new();
    let mut current = 0u32;

    let pairs = input.trim_end()
        .lines()
        .map(|line| {
            let (a, rest) = line.split_once(": ").unwrap();
            rest.split_whitespace().map(|b| {
                let aa = match node_translate.get(a) {
                    Option::Some(v) => *v,
                    _ => {
                        let v = current;
                        node_translate.insert(a, v);
                        current += 1;
                        v
                    }
                };
                let bb = match node_translate.get(b) {
                    Option::Some(v) => *v,
                    _ => {
                        let v = current;
                        node_translate.insert(b, v);
                        current += 1;
                        v
                    }
                };
                (aa, bb)
            }).collect::<Vec<(u32, u32)>>()
        }).flatten().collect::<Vec<(u32, u32)>>();

    let graph = UnGraph::<u32, ()>::from_edges(&pairs);
    let min_cur_res: Result<Option<(u32, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));

    let (_, partition) = min_cur_res.unwrap().unwrap();

    let p1 = partition.len();
    let ns = node_translate.len();
    let p2 = ns - p1;

    Option::Some(p1 * p2)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
