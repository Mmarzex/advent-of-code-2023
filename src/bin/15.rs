advent_of_code::solution!(15);

// #[derive(Debug, Clone)]
// struct Lense {
//     label: String,
//     length: u32
// }

fn get_hash(input: &str) -> u32 {
    let mut result = 0;
    for c in input.chars() {
        let ascii_value = c as u32;
        result += ascii_value;
        result = result * 17;
        result = result % 256;
    }
    result
}

fn get_box_id(input: &str) -> u32 {
    let split_res = input.split(['=', '-'].as_ref()).collect::<Vec<&str>>();
    let label = split_res[0];
    get_hash(label)
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.trim_end().split(",").map(get_hash).sum();
    Option::Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = input.trim_end().split(",").collect::<Vec<&str>>();

    let mut boxes: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];
    for instruction in instructions {
        let command = match instruction.contains('=') {
            true => '=',
            false => '-',
        };
        let split_res = instruction
            .split(['=', '-'].as_ref())
            .collect::<Vec<&str>>();
        let label = split_res[0];
        let box_id = get_hash(label);
        let ubox_id = box_id as usize;

        if command == '=' {
            let length = split_res[1].parse::<u32>().unwrap();
            let existing_index = boxes[ubox_id].iter().position(|&l| l.0 == label);
            match existing_index {
                Some(idx) => {
                    let _ = std::mem::replace(&mut boxes[ubox_id][idx], (label, length));
                }
                None => {
                    boxes[ubox_id].push((label, length));
                }
            };
        } else {
            let existing_index = boxes[ubox_id].iter().position(|&l| l.0 == label);
            match existing_index {
                Some(idx) => {
                    boxes[ubox_id].remove(idx);
                }
                _ => (),
            };
        }
    }

    let mut result = 0;
    for (box_id, lense_box) in boxes.iter().enumerate() {
        for (slot_id, lense) in lense_box.iter().enumerate() {
            result += (box_id as u32 + 1) * (slot_id as u32 + 1) * lense.1;
        }
    }
    Option::Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(145));
    }
}
