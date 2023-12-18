advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

use std::collections::HashSet;

use Tile::*;

impl Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Tile::NS,
            '-' => Tile::EW,
            'L' => Tile::NE,
            'J' => Tile::NW,
            '7' => Tile::SW,
            'F' => Tile::SE,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    y: usize,
    x: usize,
}

impl Coord {
    fn new(y: usize, x: usize) -> Self {
        Coord { y, x }
    }

    fn neighbors(&self, grid: &[Vec<Tile>]) -> Vec<Coord> {
        let mut neighbors: Vec<Coord> = vec![];
        let height = grid.len() - 1;
        let width = grid[0].len() - 1;

        match grid[self.y][self.x] {
            Ground => (),
            Start => {
                if self.y > 0 && matches!(grid[self.y - 1][self.x], NS | SW | SE) {
                    neighbors.push(Coord::new(self.y - 1, self.x))
                }
                if self.y < height && matches!(grid[self.y + 1][self.x], NS | NW | NE) {
                    neighbors.push(Coord::new(self.y + 1, self.x))
                }
                if self.x > 0 && matches!(grid[self.y][self.x - 1], EW | SE | NE) {
                    neighbors.push(Coord::new(self.y, self.x - 1))
                }
                if self.x < width && matches!(grid[self.y][self.x + 1], EW | NW | SW) {
                    neighbors.push(Coord::new(self.y, self.x + 1))
                }
            }
            NS => {
                if self.y > 0 && matches!(grid[self.y - 1][self.x], NS | SW | SE | Start) {
                    neighbors.push(Coord::new(self.y - 1, self.x))
                }
                if self.y < height && matches!(grid[self.y + 1][self.x], NS | NW | NE | Start) {
                    neighbors.push(Coord::new(self.y + 1, self.x))
                }
            }
            EW => {
                if self.x > 0 && matches!(grid[self.y][self.x - 1], EW | SE | NE | Start) {
                    neighbors.push(Coord::new(self.y, self.x - 1))
                }
                if self.x < width && matches!(grid[self.y][self.x + 1], EW | SW | NW | Start) {
                    neighbors.push(Coord::new(self.y, self.x + 1))
                }
            }
            NE => {
                if self.y > 0 && matches!(grid[self.y - 1][self.x], NS | SW | SE | Start) {
                    neighbors.push(Coord::new(self.y - 1, self.x))
                }
                if self.x < width && matches!(grid[self.y][self.x + 1], EW | NW | SW | Start) {
                    neighbors.push(Coord::new(self.y, self.x + 1))
                }
            }
            NW => {
                if self.y > 0 && matches!(grid[self.y - 1][self.x], NS | SW | SE | Start) {
                    neighbors.push(Coord::new(self.y - 1, self.x))
                }
                if self.x > 0 && matches!(grid[self.y][self.x - 1], EW | SE | NE | Start) {
                    neighbors.push(Coord::new(self.y, self.x - 1))
                }
            }
            SW => {
                if self.y < height && matches!(grid[self.y + 1][self.x], NS | NW | NE | Start) {
                    neighbors.push(Coord::new(self.y + 1, self.x))
                }
                if self.x > 0 && matches!(grid[self.y][self.x - 1], EW | SE | NE | Start) {
                    neighbors.push(Coord::new(self.y, self.x - 1))
                }
            }
            SE => {
                if self.y < height && matches!(grid[self.y + 1][self.x], NS | NW | NE | Start) {
                    neighbors.push(Coord::new(self.y + 1, self.x))
                }

                if self.x < width && matches!(grid[self.y][self.x + 1], EW | NW | SW | Start) {
                    neighbors.push(Coord::new(self.y, self.x + 1))
                }
            }
        }

        neighbors
    }
}

fn get_loop(start: Coord, grid: &[Vec<Tile>]) -> HashSet<Coord> {
    let mut coords = HashSet::new();
    coords.insert(start);
    let mut queue = start.neighbors(grid);

    while let Some(pos) = queue.pop() {
        for neighbor in pos.neighbors(grid) {
            if !coords.contains(&neighbor) {
                queue.push(neighbor);
                coords.insert(neighbor);
            }
        }
    }

    coords
}

fn start_pipe_type(grid: &Vec<Vec<Tile>>, start: Coord) -> Tile {
    let neighbors = start.neighbors(grid);
    let north = neighbors.iter().find(|c| c.y < start.y).is_some();
    let south = neighbors.iter().find(|c| c.y > start.y).is_some();
    let west = neighbors.iter().find(|c| c.x < start.x).is_some();
    let east = neighbors.iter().find(|c| c.x > start.x).is_some();

    match (north, west, south, east) {
        (true, true, _, _) => NW,
        (true, _, true, _) => NS,
        (true, _, _, true) => NE,
        (_, true, true, _) => SW,
        (_, _, true, true) => SE,
        (_, true, _, true) => EW,
        _ => unreachable!(),
    }
}

fn clean_grid(start: Coord, loop_coords: &HashSet<Coord>, grid: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let start_pipe = start_pipe_type(&grid, start);

    grid.into_iter()
        .enumerate()
        .map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .map(|(x, tile)| match tile {
                    Start => start_pipe,
                    pipe if loop_coords.contains(&Coord::new(y, x)) => pipe,
                    _ => Ground,
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut start = Coord::new(0, 0);
    let grid: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let tile = Tile::from(c);
                    if tile == Tile::Start {
                        start = Coord::new(y, x);
                    }
                    tile
                })
                .collect()
        })
        .collect();

    let coords = get_loop(start, &grid);
    Option::Some(coords.len() / 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut start = Coord::new(0, 0);
    let grid: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let tile = Tile::from(c);
                    if tile == Tile::Start {
                        start = Coord::new(y, x);
                    }
                    tile
                })
                .collect()
        })
        .collect();

    let coords = get_loop(start, &grid);

    let grid = clean_grid(start, &coords, grid);

    let mut inside = false;
    let result = grid
        .into_iter()
        .flatten()
        .filter(|tile| match tile {
            Ground => inside,
            NS | NW | NE => {
                inside = !inside;
                false
            }
            _ => false,
        })
        .count();
    Option::Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
