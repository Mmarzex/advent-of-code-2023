use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

advent_of_code::solution!(22);

#[derive(Debug, Clone)]
struct Cube {
    x: RangeInclusive<u32>,
    y: RangeInclusive<u32>,
    z: RangeInclusive<u32>,
}

impl Cube {
    fn from(lhs: &str, rhs: &str) -> Self {
        let mut lhs = lhs.split(',');
        let x1 = lhs.next().unwrap().parse::<u32>().unwrap();
        let y1 = lhs.next().unwrap().parse::<u32>().unwrap();
        let z1 = lhs.next().unwrap().parse::<u32>().unwrap();
        let mut rhs = rhs.split(',');
        let x2 = rhs.next().unwrap().parse::<u32>().unwrap();
        let y2 = rhs.next().unwrap().parse::<u32>().unwrap();
        let z2 = rhs.next().unwrap().parse::<u32>().unwrap();

        Self {
            x: (x1..=x2),
            y: (y1..=y2),
            z: (z1..=z2),
        }
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.x.end() >= other.x.start()
            && other.x.end() >= self.x.start()
            && self.y.end() >= other.y.start()
            && other.y.end() >= self.y.start()
    }
}

#[derive(Debug)]
struct State {
    cubes: Vec<Cube>,
    rests_on: HashMap<usize, Vec<usize>>,
    supports: HashMap<usize, Vec<usize>>,
}

impl State {
    fn parse(input: &str) -> Self {
        let mut cubes = parse_cubes(input);
        cubes.sort_by_key(|cube| *cube.z.start());
        Self {
            cubes,
            rests_on: HashMap::default(),
            supports: HashMap::default(),
        }
    }

    fn settle(&mut self) {
        for cube_id in 0..self.cubes.len() {
            let cube = self.cubes[cube_id].clone();
            let mut z = 1;
            let mut rest_on = vec![];
            for (idx, settled) in self.cubes[0..cube_id].iter().enumerate() {
                if cube.overlaps(settled) {
                    rest_on.push(idx);
                    z = z.max(*settled.z.end() + 1);
                }
            }
            for (idx, settled) in rest_on.into_iter().map(|elem| (elem, &self.cubes[elem])) {
                if settled.z.end() + 1 == z {
                    self.rests_on.entry(cube_id).or_default().push(idx);
                    self.supports.entry(idx).or_default().push(cube_id);
                }
            }
            let cube = &mut self.cubes[cube_id];
            cube.z = z..=(cube.z.end() - cube.z.start() + z);
        }
    }

    fn can_disintegrate(&self, idx: usize) -> bool {
        match self.supports.get(&idx) {
            None => true,
            Some(supported) => supported
                .iter()
                .all(|supported| self.rests_on.get(supported).unwrap().len() > 1),
        }
    }

    fn count_disintegrated(&self, idx: usize) -> usize {
        let mut falling = HashSet::from([idx]);

        let mut check = vec![];
        let mut check_next = self.supports.get(&idx).cloned().unwrap_or_default();

        while !check_next.is_empty() {
            std::mem::swap(&mut check, &mut check_next);
            for check in check.drain(..) {
                if self.rests_on[&check]
                    .iter()
                    .all(|idx| falling.contains(idx))
                {
                    falling.insert(check);
                    check_next.extend(
                        self.supports
                            .get(&check)
                            .iter()
                            .flat_map(|elem| elem.iter())
                            .copied(),
                    );
                }
            }
        }

        falling.len() - 1
    }
}

fn parse_cubes(input: &str) -> Vec<Cube> {
    input
        .trim_end()
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once("~").unwrap();
            Cube::from(lhs, rhs)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut state = State::parse(input);
    state.settle();
    let res = state
        .cubes
        .iter()
        .enumerate()
        .filter(|(idx, _)| state.can_disintegrate(*idx))
        .count();

    Option::Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut state = State::parse(input);
    state.settle();
    let res = state
        .cubes
        .iter()
        .enumerate()
        .map(|(idx, _)| state.count_disintegrated(idx))
        .sum();

    Option::Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(7));
    }
}
