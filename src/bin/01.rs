advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let answer: u32 = input
        .trim_end()
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_digit(10))
                .collect::<Vec<char>>()
        })
        .map(|v| {
            let value = format!("{}{}", v.first().unwrap(), v.last().unwrap());
            let vv = value.parse::<u32>().unwrap();
            vv
        })
        .sum();

    Option::Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let answer: u32 = input
        .trim_end()
        .lines()
        .map(|line| {
            line.replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<Vec<char>>()
        })
        .map(|v| {
            let value = format!("{}{}", v.first().unwrap(), v.last().unwrap());
            let vv = value.parse::<u32>().unwrap();
            vv
        })
        .sum();

    Option::Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(281));
    }
}
