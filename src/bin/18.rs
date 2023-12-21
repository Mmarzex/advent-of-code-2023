advent_of_code::solution!(18);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    amount: isize,
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn advance(&self, direction: &Direction, amount: isize) -> Self {
        match direction {
            Up => Self {
                x: self.x + amount,
                y: self.y,
            },
            Down => Self {
                x: self.x - amount,
                y: self.y,
            },
            Left => Self {
                x: self.x,
                y: self.y - amount,
            },
            Right => Self {
                x: self.x,
                y: self.y + amount,
            },
        }
    }
}

fn area(instructions: impl Iterator<Item = Instruction>) -> isize {
    let (area, perimeter, _) = instructions.fold(
        (0, 0, Coordinate { x: 0, y: 0 }),
        |(area, perimeter, position), Instruction { direction, amount }| {
            let new_position = position.advance(&direction, amount);
            let new_area = area + (position.x * new_position.y - new_position.x * position.y);
            let new_perimeter = (new_position.x - position.x).abs()
                + (new_position.y - position.y).abs()
                + perimeter;
            (new_area, new_perimeter, new_position)
        },
    );
    (area.abs() + perimeter) / 2 + 1
}

pub fn part_one(input: &str) -> Option<isize> {
    let instructions = input.trim_end().lines().map(|line| {
        let (instruction, _) = line.split_once(" (").unwrap();
        let (direction, amount) = instruction.split_once(" ").unwrap();
        let direction = match direction {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => unreachable!(),
        };
        let amount = amount.parse().unwrap();
        Instruction { direction, amount }
    });
    Option::Some(area(instructions))
}

pub fn part_two(input: &str) -> Option<isize> {
    let instructions = input.trim_end().lines().map(|line| {
        let (_, hex) = line.strip_suffix(")").unwrap().split_once("(#").unwrap();
        let (amount, direction) = hex.split_at(5);
        let amount = isize::from_str_radix(amount, 16).unwrap();
        let direction = match direction {
            "3" => Up,
            "1" => Down,
            "2" => Left,
            "0" => Right,
            _ => unreachable!(),
        };
        Instruction { direction, amount }
    });
    Option::Some(area(instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(952408144115));
    }
}
