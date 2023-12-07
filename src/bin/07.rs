use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(7);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
struct Hand {
    bid: usize,
    hand: Vec<char>,
}

impl Hand {
    fn get_score(&self) -> usize {
        let mut hand = [0; 15];
        for card in self.hand.iter() {
            let v = value_of_card(*card, 11);
            hand[v] += 1
        }
        hand.sort();
        let result = match hand[11..15] {
            [0, 0, 0, 5] => 7,
            [0, 0, 1, 4] => 6,
            [0, 0, 2, 3] => 5,
            [0, 1, 1, 3] => 4,
            [0, 1, 2, 2] => 3,
            [1, 1, 1, 2] => 2,
            _ => 1,
        };
        result
    }
    fn get_score_old(&self) -> usize {
        let mut card_groups: HashMap<char, usize> = HashMap::new();
        for card in self.hand.clone() {
            card_groups.entry(card).and_modify(|c| *c += 1).or_insert(1);
        }

        // If only 1 then it must be five of a kind
        if card_groups.len() == 1 {
            return 7;
        }

        // If two then it could be four of a kind or a full house
        if card_groups.len() == 2 {
            let is_four = card_groups.iter().any(|(_, count)| *count == 4);
            if is_four {
                return 6;
            }
            return 5;
        }

        // If three then it is three of a kind or two pair
        if card_groups.len() == 3 {
            let is_three = card_groups.iter().any(|(_, count)| *count == 3);
            if is_three {
                return 4;
            }
            return 3;
        }

        let is_pair = card_groups.iter().any(|(_, count)| *count == 2);
        if is_pair {
            return 2;
        }

        println!("Card_groups: {:?}", card_groups);
        1
    }

    fn get_score_p2(&self) -> usize {
        let mut hand = [0; 15];
        for card in self.hand.iter() {
            let v = value_of_card(*card, 1);
            hand[v] += 1
        }
        let j = hand[1];
        hand[1] = 0;
        hand.sort();
        hand[14] += j;
        let result = match hand[11..15] {
            [0, 0, 0, 5] => 7,
            [0, 0, 1, 4] => 6,
            [0, 0, 2, 3] => 5,
            [0, 1, 1, 3] => 4,
            [0, 1, 2, 2] => 3,
            [1, 1, 1, 2] => 2,
            _ => 1,
        };
        result
    }

    fn get_compared_score(&self, other: &Self, j_value: usize) -> Ordering {
        for (i, v) in self.hand.iter().enumerate() {
            let a = value_of_card(*v, j_value);
            let b = value_of_card(other.hand[i], j_value);
            let ord = a.cmp(&b);
            if ord != Ordering::Equal {
                return ord;
            }
        }
        unreachable!()
    }

    fn cmp_p2(&self, other: &Self) -> Ordering {
        let score_1 = self.get_score_p2();
        let score_2 = other.get_score_p2();
        match score_1.cmp(&score_2) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.get_compared_score(other, 1),
        }
    }
}

fn value_of_card(card: char, j_value: usize) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => j_value,
        'T' => 10,
        _ => card.to_string().parse::<usize>().unwrap(),
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let score_1 = self.get_score();
        let score_2 = other.get_score();
        match score_1.cmp(&score_2) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.get_compared_score(other, 11),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let hands = input
        .trim_end()
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            Hand {
                hand: split[0].chars().collect(),
                bid: split[1].parse().unwrap(),
            }
        })
        .collect::<Vec<Hand>>();

    let mut sorted_hands = hands.clone();
    sorted_hands.sort_by(|a, b| a.cmp(b));
    let mut result: usize = 0;
    for (i, hand) in sorted_hands.iter().enumerate() {
        result += (i + 1) * hand.bid;
    }
    Option::Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let hands = input
        .trim_end()
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            Hand {
                hand: split[0].chars().collect(),
                bid: split[1].parse().unwrap(),
            }
        })
        .collect::<Vec<Hand>>();

    let mut sorted_hands = hands.clone();
    sorted_hands.sort_by(|a, b| a.cmp_p2(b));
    let mut result: usize = 0;
    for (i, hand) in sorted_hands.iter().enumerate() {
        result += (i + 1) * hand.bid;
    }
    Option::Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(5905));
    }
}
