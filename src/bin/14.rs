advent_of_code::solution!(14);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    RoundRock,
    CubeRock,
}

use std::mem::replace;

use Tile::*;

impl Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Empty,
            '#' => CubeRock,
            'O' => RoundRock,
            _ => unreachable!(),
        }
    }
}

fn tilt_grid(grid: &mut Vec<Vec<Tile>>) {
    for x in 0..grid[0].len() {
        let mut empty_row = 0;
        for y in 0..grid.len() {
            let curr = grid[y][x];
            match curr {
                CubeRock => empty_row = y + 1,
                RoundRock => {
                    let r = replace(&mut grid[empty_row][x], curr);
                    let _ = replace(&mut grid[y][x], r);
                    empty_row += 1;
                }
                Empty => (),
            }
        }
    }
}

fn cycle(mut grid: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    for _ in 0..4 {
        tilt_grid(&mut grid);
        let rotated = turn(&grid);
        grid = rotated;
    }
    grid
}

fn turn(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let size = grid.len();
    let mut r = vec![vec![Empty; size]; size];
    for y in 0..size {
        for x in 0..size {
            r[x][size - 1 - y] = grid[y][x];
        }
    }
    r
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = input
        .trim_end()
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect::<Vec<Vec<Tile>>>();

    tilt_grid(&mut grid);

    let result = grid
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, row)| row.iter().filter(|t| t == &&RoundRock).count() * (idx + 1))
        .sum();
    Option::Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = input
        .trim_end()
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect::<Vec<Vec<Tile>>>();

    let mut seen = vec![grid.clone()];

    loop {
        grid = cycle(grid);

        if let Some(idx) = seen.iter().position(|x| x == &grid) {
            let len = seen.len() - idx;

            let final_idx = idx + (1_000_000_000 - idx) % len;

            let result = seen[final_idx]
                .iter()
                .rev()
                .enumerate()
                .map(|(idx, row)| row.iter().filter(|t| t == &&RoundRock).count() * (idx + 1))
                .sum();

            return Option::Some(result);
        }

        seen.push(grid.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(64));
    }
}
