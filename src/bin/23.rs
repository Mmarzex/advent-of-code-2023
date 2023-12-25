use std::ops::{Index, IndexMut};

advent_of_code::solution!(23);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
    Walked,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Path),
            '#' => Ok(Self::Forest),
            '^' => Ok(Self::Slope(Direction::Up)),
            'v' => Ok(Self::Slope(Direction::Down)),
            '<' => Ok(Self::Slope(Direction::Left)),
            '>' => Ok(Self::Slope(Direction::Right)),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    grid: Vec<Tile>,
    rows: usize,
    columns: usize
}

impl State {
    fn parse(input: &str) -> Self {
        let input = input.trim();

        let rows = input.lines().count();

        let columns = input.lines().next().unwrap().len();

        let mut grid = Vec::with_capacity(rows * columns);

        for line in input.lines() {
            if line.len() != columns {
                unreachable!()
            }

            grid.extend(line.chars().map(|c| Tile::try_from(c).unwrap()));
        }

        Self {
            grid,
            rows,
            columns
        }
    }

    // fn lines(&self) -> StateIterator {
    //     StateIterator {
    //         state: self,
    //         current_row: 0
    //     }
    // }

    // fn get(&self, (x, y): (usize, usize)) -> Option<Tile> {
    //     if x < self.columns && y < self.rows {
    //         Some(self.grid[y * self.columns + x])
    //     } else {
    //         None
    //     }
    // }

    // fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut Tile> {
    //     if x < self.columns && y < self.rows {
    //         Some(&mut self.grid[y * self.columns + x])
    //     } else {
    //         None
    //     }
    // }
}

struct StateIterator<'a> {
    state: &'a State,
    current_row: usize
}

impl<'a> Iterator for StateIterator<'a> {
    type Item = &'a [Tile];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row < self.state.rows {
            let r = Some(&self.state[self.current_row]);
            self.current_row += 1;
            return r;
        } else {
            None
        }
    }
}

impl Index<usize> for State {
    type Output = [Tile];

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index * self.columns..index * self.columns + self.columns]
    }
}

impl IndexMut<usize> for State{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.grid[index *self.columns..index * self.columns + self.columns]
    }
}

fn walk_path(state: State, (x, y): (usize, usize), path: usize, paths: &mut Vec<usize>) {
    if y == state.rows - 1 {
        paths.push(path);
    }

    match state[y - 1][x] {
        Tile::Path | Tile::Slope(Direction::Up) => {
            let mut clone = state.clone();
            clone[y - 1][x] = Tile::Walked;
            walk_path(clone, (x, y - 1), path + 1, paths);
        }
        _ => ()
    }

    if y + 1 < state.rows {
        match state[y + 1][x] {
            Tile::Path | Tile::Slope(Direction::Down) => {
                let mut clone = state.clone();
                clone[y + 1][x] = Tile::Walked;
                walk_path(clone, (x, y + 1), path + 1, paths);
            },
            _ => ()
        }
    }

    match state[y][x - 1] {
        Tile::Path | Tile::Slope(Direction::Left) => {
            let mut clone = state.clone();
            clone[y][x - 1] = Tile::Walked;
            walk_path(clone, (x - 1, y), path + 1, paths);
        },
        _ => ()
    }

    match state[y][x + 1] {
        Tile::Path | Tile::Slope(Direction::Right) => {
            let mut clone = state.clone();
            clone[y][x + 1] = Tile::Walked;
            walk_path(clone, (x + 1, y), path + 1, paths);
        },
        _ => ()
    }
}

fn walk_path_p2(state: &mut State, (x, y): (usize, usize), path: usize, paths: &mut Vec<usize>) {
    if y == state.rows - 1 {
        paths.push(path);
    }

    match state[y - 1][x] {
        Tile::Path | Tile::Slope(_) => {
            let t = state[y -1][x];
            state[y - 1][x] = Tile::Walked;
            walk_path_p2(state, (x, y - 1), path + 1, paths);
            state[y-1][x] = t;
        }
        _ => ()
    }

    if y + 1 < state.rows {
        match state[y + 1][x] {
            Tile::Path | Tile::Slope(_) => {
                let t = state[y+1][x];
                state[y + 1][x] = Tile::Walked;
                walk_path_p2(state, (x, y + 1), path + 1, paths);
                state[y+1][x] = t;
            },
            _ => ()
        }
    }

    match state[y][x - 1] {
        Tile::Path | Tile::Slope(_) => {
            let t = state[y][x-1];
            state[y][x - 1] = Tile::Walked;
            walk_path_p2(state, (x - 1, y), path + 1, paths);
            state[y][x-1] = t;
        },
        _ => ()
    }

    match state[y][x + 1] {
        Tile::Path | Tile::Slope(_) => {
            let t = state[y][x+1];
            state[y][x + 1] = Tile::Walked;
            walk_path_p2(state, (x + 1, y), path + 1, paths);
            state[y][x+1] = t;
        },
        _ => ()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut state = State::parse(input);

    let start_index = state[0]
        .iter()
        .enumerate()
        .find(|&(_, &t)| t == Tile::Path)
        .unwrap()
        .0;

    state[0][start_index] = Tile::Walked;

    let (x, y) = (start_index, 1);


    let mut result = vec![];
    walk_path(state, (x, y), 1, &mut result);


    let res = *result.iter().max().unwrap();
    Option::Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut state = State::parse(input);

    let start_index = state[0]
        .iter()
        .enumerate()
        .find(|&(_, &t)| t == Tile::Path)
        .unwrap()
        .0;

    state[0][start_index] = Tile::Walked;

    let (x, y) = (start_index, 1);


    let mut result = vec![];
    walk_path_p2(&mut state, (x, y), 1, &mut result);


    let res = *result.iter().max().unwrap();
    Option::Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(154));
    }
}
