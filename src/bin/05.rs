use std::{ops::Range, slice::SliceIndex};

advent_of_code::solution!(5);

#[derive(Debug)]
struct MapEntry {
    destination: Range<usize>,
    source: Range<usize>,
}

impl MapEntry {
    fn from_line(line: &str) -> MapEntry {
        let split_line = line
            .split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        MapEntry {
            destination: (split_line[0]..(split_line[0] + split_line[2])),
            source: (split_line[1]..(split_line[1] + split_line[2])),
        }
    }

    fn get_destination(&mut self, source: usize) -> Option<usize> {
        if self.is_in_entry(source) {
            let idx = self
                .source
                .clone()
                .enumerate()
                .find(|s| s.1 == source)
                .unwrap()
                .0;
            Option::Some(self.destination.clone().nth(idx).unwrap())
        } else {
            Option::None
        }
    }

    fn is_in_entry(&self, source: usize) -> bool {
        self.source.clone().contains(&source)
    }
}

fn get_destination_v1(entries: &mut Vec<MapEntry>, source: usize) -> usize {
    entries
        .iter_mut()
        .find_map(|e| e.get_destination(source))
        .unwrap_or(source)
}

fn get_destination(entries: &[MapEntry], source: usize) -> usize {
    for entry in entries {
        if entry.source.contains(&source) {
            let shift = source - entry.source.start;
            return entry.destination.start + shift;
        }
    }

    source
}

pub fn part_one(input: &str) -> Option<usize> {
    let splits = input.trim_end().split("\n\n").collect::<Vec<&str>>();

    let seeds = splits[0]
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let seeds_to_soil = splits[1]
        .split("\n")
        .skip(1)
        .map(MapEntry::from_line)
        .collect::<Vec<MapEntry>>();
    let soil_to_fertilizer = splits[2]
        .split("\n")
        .skip(1)
        .map(MapEntry::from_line)
        .collect::<Vec<MapEntry>>();
    let fertilizer_to_water = splits[3]
        .split("\n")
        .skip(1)
        .map(MapEntry::from_line)
        .collect::<Vec<MapEntry>>();
    let water_to_light = splits[4]
        .split("\n")
        .skip(1)
        .map(MapEntry::from_line)
        .collect::<Vec<MapEntry>>();
    let light_to_temperature = splits[5]
        .split("\n")
        .skip(1)
        .map(MapEntry::from_line)
        .collect::<Vec<MapEntry>>();
    let temperature_to_humidity = splits[6]
        .split("\n")
        .skip(1)
        .map(MapEntry::from_line)
        .collect::<Vec<MapEntry>>();
    let humidity_to_location = splits[7]
        .split("\n")
        .skip(1)
        .map(MapEntry::from_line)
        .collect::<Vec<MapEntry>>();
    println!("Seeds: {:?}", seeds);
    println!("{:?}", seeds_to_soil);

    let mut min_location: usize = usize::MAX;
    for seed in seeds {
        let soil = get_destination(&seeds_to_soil, seed);
        println!("Seed: {}, Soil: {}", seed, soil);
        let fertilizer = get_destination(&soil_to_fertilizer, soil);
        println!("Soil: {}, fertilizer: {}", soil, fertilizer);
        let water = get_destination(&fertilizer_to_water, fertilizer);
        println!("fertilizer: {}, water: {}", fertilizer, water);
        let light = get_destination(&water_to_light, water);
        println!("water: {}, light: {}", water, light);
        let temperature = get_destination(&light_to_temperature, light);
        println!("light: {}, temperature: {}", light, temperature);
        let humidity = get_destination(&temperature_to_humidity, temperature);
        println!("temperature: {}, humidity: {}", temperature, humidity);
        let location = get_destination(&humidity_to_location, humidity);
        println!("humidity: {}, location: {}", humidity, location);
        min_location = min_location.min(location);
    }
    Option::Some(min_location)
}

pub fn part_two(input: &str) -> Option<usize> {
    let splits = input.trim_end().split("\n\n").collect::<Vec<&str>>();

    let seeds = splits[0]
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2)
        .flat_map(|line| {
            let &[start, length] = line else {
                unreachable!()
            };
            start..(start + length)
        })
        .collect::<Vec<usize>>();

    let seeds_to_soil = splits[1]
        .split("\n")
        .skip(1)
        .map(MapEntry::from_line)
        .collect::<Vec<MapEntry>>();
    let soil_to_fertilizer = splits[2]
        .split("\n")
        .skip(1)
        .map(MapEntry::from_line)
        .collect::<Vec<MapEntry>>();
    let fertilizer_to_water = splits[3]
        .split("\n")
        .skip(1)
        .map(MapEntry::from_line)
        .collect::<Vec<MapEntry>>();
    let water_to_light = splits[4]
        .split("\n")
        .skip(1)
        .map(MapEntry::from_line)
        .collect::<Vec<MapEntry>>();
    let light_to_temperature = splits[5]
        .split("\n")
        .skip(1)
        .map(MapEntry::from_line)
        .collect::<Vec<MapEntry>>();
    let temperature_to_humidity = splits[6]
        .split("\n")
        .skip(1)
        .map(MapEntry::from_line)
        .collect::<Vec<MapEntry>>();
    let humidity_to_location = splits[7]
        .split("\n")
        .skip(1)
        .map(MapEntry::from_line)
        .collect::<Vec<MapEntry>>();

    let mut min_location: usize = usize::MAX;
    let num_of_seeds = seeds.len();
    for (idx, seed) in seeds.iter().enumerate() {
        println!(
            "Num of Seeds: {}, idx: {} left: {}",
            num_of_seeds,
            idx,
            num_of_seeds - idx
        );
        let soil = get_destination(&seeds_to_soil, *seed);
        let fertilizer = get_destination(&soil_to_fertilizer, soil);
        let water = get_destination(&fertilizer_to_water, fertilizer);
        let light = get_destination(&water_to_light, water);
        let temperature = get_destination(&light_to_temperature, light);
        let humidity = get_destination(&temperature_to_humidity, temperature);
        let location = get_destination(&humidity_to_location, humidity);
        min_location = min_location.min(location);
    }

    Option::Some(min_location)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Option::Some(46));
    }
}
