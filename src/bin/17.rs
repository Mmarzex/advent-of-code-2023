use pathfinding::matrix::{directions, Matrix};
use pathfinding::prelude::astar;

advent_of_code::solution!(17);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct State {
    position: (usize, usize),
    direction: (isize, isize),
    distance: usize,
}

fn shortest_path<const MIN: usize, const MAX: usize>(grid: &Matrix<usize>) -> usize {
    let start = State {
        position: (0, 0),
        direction: (0, 0),
        distance: 0,
    };

    let end = (grid.rows - 1, grid.columns - 1);

    let path = astar(
        &start,
        |state| match state.distance >= MIN || (state.direction.0 == 0 && state.direction.1 == 0) {
            true => compute_neighbors::<MAX>(state, grid, &start),
            false => compute_successor::<MIN>(state, grid),
        },
        |state| (end.0.abs_diff(state.position.0) + end.1.abs_diff(state.position.1)),
        |state| state.position == end && state.distance >= MIN,
    )
    .unwrap();

    path.1
}

fn compute_neighbors<const MAX: usize>(
    state: &State,
    grid: &Matrix<usize>,
    start: &State,
) -> Vec<(State, usize)> {
    [directions::N, directions::S, directions::E, directions::W]
        .iter()
        .flat_map(|direction| {
            grid.move_in_direction(state.position, *direction)
                .map(|point| (point, *direction, *grid.get(point).unwrap()))
        })
        .filter(|(position, direction, _)| {
            let is_inverse = state.direction.0 == -direction.0 && state.direction.1 == -direction.1;

            !is_inverse && *position != start.position
        })
        .flat_map(|(position, direction, weight)| {
            let distance = match state.direction == direction {
                true => state.distance + 1,
                false => 1,
            };

            match distance <= MAX {
                true => {
                    let next_state = State {
                        position,
                        direction,
                        distance,
                    };
                    Some((next_state, weight))
                }
                false => None,
            }
        })
        .collect()
}

fn compute_successor<const MIN: usize>(state: &State, grid: &Matrix<usize>) -> Vec<(State, usize)> {
    match grid.move_in_direction(state.position, state.direction) {
        Some(point) => {
            let weight = *grid.get(point).unwrap();
            let new_state = State {
                position: point,
                direction: state.direction,
                distance: state.distance + 1,
            };

            vec![(new_state, weight)]
        }

        None => vec![],
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = input
        .trim_end()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .collect::<Matrix<usize>>();
    Option::Some(shortest_path::<1, 3>(&grid))
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = input
        .trim_end()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .collect::<Matrix<usize>>();
    Option::Some(shortest_path::<4, 10>(&grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(94));
    }
}
