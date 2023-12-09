advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let sequences = input
        .trim_end()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    fn diffs(s: &Vec<i32>) -> Vec<i32> {
        s.windows(2).map(|w| w[1] - w[0]).collect()
    }

    fn solve(s: &Vec<i32>) -> i32 {
        let zeroed = s.iter().all(|v| *v == 0);
        if zeroed {
            0
        } else {
            s.last().unwrap() + solve(&diffs(s))
        }
    }

    let result = sequences.iter().map(solve).sum();
    Option::Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let sequences = input
        .trim_end()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    fn diffs(s: &Vec<i32>) -> Vec<i32> {
        s.windows(2).map(|w| w[1] - w[0]).collect()
    }

    fn solve(s: &Vec<i32>) -> i32 {
        let zeroed = s.iter().all(|v| *v == 0);
        if zeroed {
            0
        } else {
            s.first().unwrap() - solve(&diffs(s))
        }
    }

    let result = sequences.iter().map(solve).sum();
    Option::Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(2));
    }
}
