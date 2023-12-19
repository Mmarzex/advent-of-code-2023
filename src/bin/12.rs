use itertools::Itertools;

advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Unknown,
    Operational,
    Damaged,
}

use Tile::*;

impl Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Damaged,
            '.' => Operational,
            '?' => Unknown,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Record {
    tiles: Vec<Tile>,
    sizes: Vec<usize>,
}

impl Record {
    fn arrangements(&self) -> usize {
        if let Some(idx) = self.tiles.iter().position(|s| s == &Unknown) {
            let mut damaged_copy = self.tiles.clone();
            damaged_copy[idx] = Damaged;

            let damaged = Record {
                tiles: damaged_copy,
                sizes: self.sizes.to_vec(),
            };

            let mut operational_copy = self.tiles.clone();
            operational_copy[idx] = Operational;

            let operational = Record {
                tiles: operational_copy,
                sizes: self.sizes.to_vec(),
            };

            damaged.arrangements() + operational.arrangements()
        } else if self.valid() {
            1
        } else {
            0
        }
    }
    fn valid(&self) -> bool {
        self.tiles
            .iter()
            .group_by(|item| *item)
            .into_iter()
            .filter_map(|(key, group)| {
                if *key == Damaged {
                    Some(group.count())
                } else {
                    None
                }
            })
            .eq(self.sizes.iter().copied())
    }
}

fn possible_arrangements(mut tiles: Vec<Tile>, sizes: Vec<usize>) -> usize {
    tiles.push(Operational);
    let mut cache = vec![vec![None; tiles.len()]; sizes.len()];
    count_possible_arrangements(&tiles, &sizes, &mut cache)
}

fn count_possible_arrangements(
    tiles: &[Tile],
    sizes: &[usize],
    cache: &mut [Vec<Option<usize>>],
) -> usize {
    if sizes.is_empty() {
        return if tiles.contains(&Damaged) { 0 } else { 1 };
    }
    if tiles.len() < sizes.iter().sum::<usize>() + sizes.len() {
        return 0;
    }

    if let Some(cached) = cache[sizes.len() - 1][tiles.len() - 1] {
        return cached;
    }

    let mut arrangements = 0;
    if tiles[0] != Damaged {
        arrangements += count_possible_arrangements(&tiles[1..], sizes, cache);
    }

    let next_size = sizes[0];
    if !tiles[..next_size].contains(&Operational) && tiles[next_size] != Damaged {
        arrangements += count_possible_arrangements(&tiles[next_size + 1..], &sizes[1..], cache);
    }
    cache[sizes.len() - 1][tiles.len() - 1] = Some(arrangements);
    arrangements
}

pub fn part_one(input: &str) -> Option<usize> {
    let records = input
        .trim_end()
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            let tiles = split[0].chars().map(Tile::from).collect::<Vec<Tile>>();
            let sizes = split[1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Record { tiles, sizes }
        })
        .collect::<Vec<Record>>();

    let result = records.iter().map(|r| r.arrangements()).sum();
    Option::Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let records = input
        .trim_end()
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            let tiles = split[0].chars().map(Tile::from).collect::<Vec<Tile>>();
            let sizes = split[1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Record {
                tiles: tiles
                    .iter()
                    .copied()
                    .chain([Unknown])
                    .cycle()
                    .take(tiles.len() * 5 + 4)
                    .collect(),
                sizes: sizes
                    .iter()
                    .copied()
                    .cycle()
                    .take(sizes.len() * 5)
                    .collect(),
            }
        })
        .collect::<Vec<Record>>();

    let result = records
        .iter()
        .map(|r| possible_arrangements(r.tiles.clone(), r.sizes.clone()))
        .sum();
    Option::Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(525152));
    }
}
