advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let cards = input
        .trim()
        .lines()
        .map(|line| {
            let mut s = line.split(':').nth(1).unwrap().split(" | ");
            let winning = s
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let numbers = s
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let mut count: u32 = 0;
            for winning_num in winning {
                if numbers.contains(&winning_num) {
                    if count == 0 {
                        count = 1;
                    } else {
                        count *= 2;
                    }
                }
            }
            count
        })
        .sum();

    Option::Some(cards)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input
        .trim()
        .lines()
        .map(|line| {
            let mut s = line.split(':').nth(1).unwrap().split(" | ");
            let winning = s
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let numbers = s
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (winning, numbers)
        })
        .collect::<Vec<(Vec<u32>, Vec<u32>)>>();

    let mut counts: Vec<usize> = vec![1; cards.len()];

    for i in 0..cards.len() {
        // Check for winning
        let (winning, numbers) = cards[i].clone();
        let mut count: usize = 0;
        for winning_num in winning.clone() {
            if numbers.contains(&winning_num) {
                count += 1;
            }
        }

        let current = *counts.get(i).unwrap_or(&0);
        for j in 0..count.min(cards.len() - 1) {
            counts[(i + 1) + j] += current;
        }
    }

    let res = counts.iter().map(|n| (*n) as u32).sum();
    Option::Some(res)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(30));
    }
}
