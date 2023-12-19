use std::collections::VecDeque;

use itertools::Itertools;

advent_of_code::solution!(13);

fn reflection_index(pattern: &VecDeque<Vec<bool>>) -> Option<usize> {
    (1..pattern.len()).find(|&offset| {
        let a = pattern.iter().take(offset).rev();
        let b = pattern.iter().skip(offset);
        let mut c = a.zip(b);
        c.all(|(aa, bb)| aa == bb)
    })
}

fn reflection_index_p2(pattern: &VecDeque<Vec<bool>>) -> Option<usize> {
    (1..pattern.len()).find(|&offset| {
        let a = pattern.iter().take(offset).rev();
        let b = pattern.iter().skip(offset);
        let c = a.zip(b);
        let differences: usize = c
            .map(|(aa, bb)| {
                aa.iter()
                    .zip(bb.iter())
                    .filter(|(aaa, bbb)| aaa != bbb)
                    .count()
            })
            .sum();

        differences == 1
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let patterns = input
        .trim_end()
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => true,
                            '#' => false,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect()
        })
        .collect::<Vec<VecDeque<Vec<bool>>>>();
    let result = patterns
        .iter()
        .map(|pattern| {
            if let Some(idx) = reflection_index(pattern) {
                return idx * 100;
            }

            let pivot = (0..pattern[0].len())
                .map(|i| pattern.iter().map(|row| row[i]).collect())
                .collect();

            if let Some(idx) = reflection_index(&pivot) {
                return idx;
            }

            0
        })
        .sum();

    Option::Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let patterns = input
        .trim_end()
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => true,
                            '#' => false,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect()
        })
        .collect::<Vec<VecDeque<Vec<bool>>>>();
    let result = patterns
        .iter()
        .map(|pattern| {
            if let Some(idx) = reflection_index_p2(pattern) {
                return idx * 100;
            }

            let pivot = (0..pattern[0].len())
                .map(|i| pattern.iter().map(|row| row[i]).collect())
                .collect();

            if let Some(idx) = reflection_index_p2(&pivot) {
                return idx;
            }

            0
        })
        .sum();

    Option::Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(400));
    }
}
