use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reqs = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let games = input
        .trim_end()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let game_id = parts[0].split(" ").nth(1).unwrap();
            let color_tuples = parts[1]
                .split("; ")
                .map(|it| {
                    it.split(", ")
                        .map(|c| {
                            let p = c.split(" ").collect::<Vec<&str>>();
                            let num = p[0].parse::<u32>().unwrap();
                            let color = p[1];
                            (color, num)
                        })
                        .collect::<Vec<(&str, u32)>>()
                })
                .collect::<Vec<Vec<(&str, u32)>>>();
            (game_id, color_tuples)
        })
        .filter(|game| {
            let is_valid = game.1.iter().all(|tries| {
                let inner_valid = tries.iter().all(|(color, num)| {
                    let r = reqs.get(color).unwrap();
                    num <= reqs.get(color).unwrap()
                });
                inner_valid
            });
            is_valid
        })
        .map(|game| game.0.parse::<u32>().unwrap())
        .sum();
    Option::Some(games)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input
        .trim_end()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let game_id = parts[0].split(" ").nth(1).unwrap();
            let color_tuples = parts[1]
                .split("; ")
                .map(|it| {
                    it.split(", ")
                        .map(|c| {
                            let p = c.split(" ").collect::<Vec<&str>>();
                            let num = p[0].parse::<u32>().unwrap();
                            let color = p[1];
                            (color, num)
                        })
                        .collect::<Vec<(&str, u32)>>()
                })
                .collect::<Vec<Vec<(&str, u32)>>>();

            let blue_max = color_tuples
                .clone()
                .iter()
                .flatten()
                .filter(|c| c.0 == "blue")
                .map(|c| c.1)
                .max()
                .unwrap();

            let green_max = color_tuples
                .clone()
                .iter()
                .flatten()
                .filter(|c| c.0 == "green")
                .map(|c| c.1)
                .max()
                .unwrap();

            let red_max = color_tuples
                .clone()
                .iter()
                .flatten()
                .filter(|c| c.0 == "red")
                .map(|c| c.1)
                .max()
                .unwrap();

            blue_max * green_max * red_max
        })
        .sum();
    Option::Some(games)
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
        assert_eq!(result, Option::Some(2286));
    }
}
