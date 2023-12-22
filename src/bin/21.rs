use std::collections::HashSet;

advent_of_code::solution!(21);

#[derive(Debug, PartialEq)]
enum Tile {
    Garden,
    Rock,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    col: isize,
    row: isize,
}

impl Coord {
    fn neighbors(&self, rows: isize, cols: isize) -> Vec<Self> {
        let mut res = Vec::new();

        if self.row > 0 {
            res.push(Coord {
                col: self.col,
                row: self.row - 1,
            });
        }

        if self.row < rows - 1 {
            res.push(Coord {
                col: self.col,
                row: self.row + 1,
            });
        }

        if self.col > 0 {
            res.push(Coord {
                col: self.col - 1,
                row: self.row,
            });
        }

        if self.col < cols - 1 {
            res.push(Coord {
                col: self.col + 1,
                row: self.row,
            });
        }

        res
    }

    fn infinite_neighbors(&self) -> Vec<Self> {
        let res = vec![
            Coord {
                col: self.col,
                row: self.row - 1,
            },
            Coord {
                col: self.col,
                row: self.row + 1,
            },
            Coord {
                col: self.col - 1,
                row: self.row,
            },
            Coord {
                col: self.col + 1,
                row: self.row,
            },
        ];

        res
    }
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Coord) {
    let mut start = Coord { col: 0, row: 0 };
    let mut grid = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '.' => Tile::Garden,
                '#' => Tile::Rock,
                'S' => {
                    start.col = x as isize;
                    start.row = y as isize;
                    Tile::Garden
                }
                _ => unreachable!(),
            };
            row.push(tile);
        }
        grid.push(row);
    }
    (grid, start)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, start) = parse(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let mut seen = HashSet::new();

    seen.insert(start);

    for _ in 0..64 {
        let mut new_seen = HashSet::new();

        for pos in seen {
            for n in pos
                .neighbors(rows as isize, cols as isize)
                .into_iter()
                .filter(|pos| grid[pos.row as usize][pos.col as usize] == Tile::Garden)
            {
                new_seen.insert(n);
            }
        }

        seen = new_seen
    }

    Option::Some(seen.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let goal = 26_501_365;
    let (grid, start) = parse(input);
    let size = grid.len();
    let to_edge = size / 2;

    let mut results = vec![];
    let mut seen = HashSet::new();
    seen.insert(start);

    for idx in 1.. {
        let mut new_seen = HashSet::new();

        for pos in seen {
            for n in pos.infinite_neighbors().into_iter().filter(|pos| {
                let y = pos.row.rem_euclid(size as isize) as usize;
                let x = pos.col.rem_euclid(size as isize) as usize;
                grid[y][x] == Tile::Garden
            }) {
                new_seen.insert(n);
            }
        }
        seen = new_seen;

        if idx == to_edge + size * results.len() {
            results.push(seen.len());

            if results.len() == 3 {
                let n = goal / size;

                let a0 = results[0];
                let a1 = results[1];
                let a2 = results[2];

                let b0 = a0;
                let b1 = a1 - a0;
                let b2 = a2 - a1;

                return Option::Some(b0 + b1 * n + (n * (n - 1) / 2) * (b2 - b1));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
