advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    SplitHorizontal,
    SplitVertical,
    MirrorForward,
    MirrorBack,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Beam {
    position: Coordinate,
    direction: Direction,
}

impl Beam {
    fn move_beam(mut self, rows: usize, cols: usize) -> Option<Self> {
        match self.direction {
            Up if self.position.y > 0 => self.position.y -= 1,
            Down if self.position.y < rows - 1 => self.position.y += 1,
            Left if self.position.x > 0 => self.position.x -= 1,
            Right if self.position.x < cols - 1 => self.position.x += 1,
            _ => return None,
        }
        Some(self)
    }
}

use std::collections::{HashSet, VecDeque};

use Direction::*;
use Tile::*;

fn run(start: Beam, grid: &[Vec<Tile>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut queue: VecDeque<Beam> = VecDeque::new();
    let mut seen: HashSet<Beam> = HashSet::new();
    let mut energized: HashSet<Coordinate> = HashSet::new();
    queue.push_back(start);

    while let Some(mut beam) = queue.pop_front() {
        if seen.contains(&beam) {
            continue;
        }
        energized.insert(beam.position);
        seen.insert(beam);

        let directions = match (grid[beam.position.y][beam.position.x], beam.direction) {
            (Empty, _)
            | (SplitHorizontal, Left)
            | (SplitHorizontal, Right)
            | (SplitVertical, Up)
            | (SplitVertical, Down) => vec![beam.direction],
            (SplitHorizontal, _) => vec![Left, Right],
            (SplitVertical, _) => vec![Up, Down],
            (MirrorForward, Up) | (MirrorBack, Down) => vec![Right],
            (MirrorForward, Down) | (MirrorBack, Up) => vec![Left],
            (MirrorForward, Left) | (MirrorBack, Right) => vec![Down],
            (MirrorForward, Right) | (MirrorBack, Left) => vec![Up],
        };

        for direction in directions {
            beam.direction = direction;
            if let Some(beam) = beam.move_beam(rows, cols) {
                queue.push_back(beam);
            }
        }
    }
    energized.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = input
        .trim_end()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Empty,
                    '-' => SplitHorizontal,
                    '|' => SplitVertical,
                    '/' => MirrorForward,
                    '\\' => MirrorBack,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect::<Vec<Vec<Tile>>>();

    let start = Beam {
        position: Coordinate { x: 0, y: 0 },
        direction: Right,
    };

    Option::Some(run(start, &grid))
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = input
        .trim_end()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Empty,
                    '-' => SplitHorizontal,
                    '|' => SplitVertical,
                    '/' => MirrorForward,
                    '\\' => MirrorBack,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect::<Vec<Vec<Tile>>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut starting_beams = vec![];

    for y in 0..height {
        for x in 0..width {
            if y == 0 && x == 0 {
                starting_beams.push(Beam {
                    position: Coordinate { x: 0, y: 0 },
                    direction: Right,
                });
                starting_beams.push(Beam {
                    position: Coordinate { x: 0, y: 0 },
                    direction: Down,
                });
            } else if y == 0 && x == width - 1 {
                starting_beams.push(Beam {
                    position: Coordinate { x: width - 1, y: 0 },
                    direction: Left,
                });
                starting_beams.push(Beam {
                    position: Coordinate { x: width - 1, y: 0 },
                    direction: Down,
                });
            } else if y == height - 1 && x == 0 {
                starting_beams.push(Beam {
                    position: Coordinate {
                        x: 0,
                        y: height - 1,
                    },
                    direction: Right,
                });
                starting_beams.push(Beam {
                    position: Coordinate {
                        x: 0,
                        y: height - 1,
                    },
                    direction: Up,
                });
            } else if y == height - 1 && x == width - 1 {
                starting_beams.push(Beam {
                    position: Coordinate {
                        x: width - 1,
                        y: height - 1,
                    },
                    direction: Up,
                });
                starting_beams.push(Beam {
                    position: Coordinate {
                        x: width - 1,
                        y: height - 1,
                    },
                    direction: Left,
                });
            } else if y == 0 {
                starting_beams.push(Beam {
                    position: Coordinate { x, y: 0 },
                    direction: Down,
                });
            } else if x == 0 {
                starting_beams.push(Beam {
                    position: Coordinate { x: 0, y },
                    direction: Right,
                });
            } else if y == height - 1 {
                starting_beams.push(Beam {
                    position: Coordinate { x, y: height - 1 },
                    direction: Up,
                });
            } else if x == width - 1 {
                starting_beams.push(Beam {
                    position: Coordinate { x: width - 1, y },
                    direction: Left,
                });
            }
        }
    }

    starting_beams.iter().map(|start| run(*start, &grid)).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(51));
    }
}
