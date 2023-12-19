use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Galaxy,
}

use std::collections::HashSet;

use Tile::*;

impl Tile {
    fn from(c: char) -> Tile {
        match c {
            '.' => Empty,
            '#' => Galaxy,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    y: usize,
    x: usize,
}

impl Coord {
    fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }

    fn manhattan_distance(&self, other: &Self) -> usize {
        self.y.abs_diff(other.y) + self.x.abs_diff(other.x)
    }
}

fn empty_rows(grid: &[Vec<Tile>]) -> HashSet<usize> {
    grid.iter()
        .enumerate()
        .filter_map(|(idx, row)| {
            if !row.contains(&Galaxy) {
                Some(idx)
            } else {
                None
            }
        })
        .collect()
}

fn empty_columns(grid: &[Vec<Tile>]) -> HashSet<usize> {
    let mut columns: Vec<Vec<Tile>> = vec![vec![Tile::Empty; grid.len()]; grid[0].len()];
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            columns[x][y] = *c;
        }
    }

    empty_rows(&columns)
}

fn galaxies(grid: &[Vec<Tile>], addition: usize) -> Vec<Coord> {
    let rows = empty_rows(grid);
    let columns = empty_columns(grid);

    let mut coords = vec![];

    let mut curr_y = 0;
    let mut curr_x = 0;

    for (y, row) in grid.iter().enumerate() {
        if rows.contains(&y) {
            curr_y += addition;
            continue;
        }
        for (x, tile) in row.iter().enumerate() {
            if columns.contains(&x) {
                curr_x += addition;
                continue;
            }

            if *tile == Galaxy {
                coords.push(Coord::new(curr_y, curr_x));
            }
            curr_x += 1;
        }
        curr_x = 0;
        curr_y += 1;
    }

    coords
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = input
        .trim_end()
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect::<Vec<Vec<Tile>>>();

    let galaxy_coords = galaxies(&grid, 2);

    let result = galaxy_coords
        .iter()
        .combinations(2)
        .map(|c| c[0].manhattan_distance(c[1]))
        .sum();

    Option::Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = input
        .trim_end()
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect::<Vec<Vec<Tile>>>();

    let galaxy_coords = galaxies(&grid, 1_000_000);

    let result = galaxy_coords
        .iter()
        .combinations(2)
        .map(|c| c[0].manhattan_distance(c[1]))
        .sum();

    Option::Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
