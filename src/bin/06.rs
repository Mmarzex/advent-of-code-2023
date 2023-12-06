advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input
        .trim_end()
        .lines()
        .map(|line| {
            let re = regex::Regex::new(r"\d+").unwrap();
            re.find_iter(line)
                .map(|m| m.as_str().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let races = lines[0]
        .iter()
        .zip(lines[1].iter())
        .collect::<Vec<(&u32, &u32)>>();

    let valid_races = races
        .iter()
        .map(|(time, distance)| {
            (0..=**time)
                .filter(|t| (**time - t) * t > **distance)
                .count()
        })
        .product();

    Option::Some(valid_races)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input
        .trim_end()
        .lines()
        .map(|line| {
            let re = regex::Regex::new(r"\d+").unwrap();
            re.find_iter(line)
                .map(|m| m.as_str())
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<usize>>();

    let valid_races = (0..=lines[0])
        .filter(|t| (lines[0] - t) * t > lines[1])
        .count();

    Option::Some(valid_races)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(71503));
    }
}
