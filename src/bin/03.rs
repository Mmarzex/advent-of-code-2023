use std::collections::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = input.trim_end().lines().collect::<Vec<&str>>();

    let pat = regex::Regex::new(r"\d+").unwrap();

    let mut parts_sum: u32 = 0;

    for (h, line) in schematic.iter().enumerate() {
        for m in pat.find_iter(line) {
            let sx = m.start().saturating_sub(1);
            let sy = h.saturating_sub(1);
            let ex = m.end();
            let ey = h + 1;
            for y in sy..=ey {
                for x in sx..=ex {
                    if y < schematic.len()
                        && x < schematic[y].len()
                        && !schematic[y].chars().nth(x).unwrap().is_numeric()
                        && schematic[y].chars().nth(x).unwrap() != '.'
                    {
                        parts_sum += m.as_str().parse::<u32>().unwrap();
                    }
                }
            }
        }
    }
    Option::Some(parts_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let schematic = input.trim_end().lines().collect::<Vec<&str>>();

    let pat = regex::Regex::new(r"\d+").unwrap();

    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for (h, line) in schematic.iter().enumerate() {
        for m in pat.find_iter(line) {
            let sx = m.start().saturating_sub(1);
            let sy = h.saturating_sub(1);
            let ex = m.end();
            let ey = h + 1;
            let num = m.as_str().parse::<u32>().unwrap();
            for y in sy..=ey {
                for x in sx..=ex {
                    if y < schematic.len()
                        && x < schematic[y].len()
                        && !schematic[y].chars().nth(x).unwrap().is_numeric()
                        && schematic[y].chars().nth(x).unwrap() != '.'
                    {
                        if schematic[y].chars().nth(x).unwrap() == '*' {
                            gears.entry((x, y)).or_insert(vec![]).push(num);
                        }
                    }
                }
            }
        }
    }
    let res: u32 = gears
        .iter()
        .filter(|(_, values)| values.len() == 2)
        .map(|(_, values)| values[0] * values[1])
        .sum();

    Option::Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(467835));
    }
}
