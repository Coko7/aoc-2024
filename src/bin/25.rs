use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(25);

type HeightMap = HashMap<usize, i32>;

fn parse_input(input: &str) -> (Vec<HeightMap>, Vec<HeightMap>) {
    let mut locks = vec![];
    let mut keys = vec![];

    for schematic in input.split("\n\n").collect::<Vec<_>>().iter() {
        let mut height_map: HeightMap = HashMap::new();
        assert_eq!(
            7,
            schematic.lines().count(),
            "Schematic should have 7 rows!"
        );

        let mut is_key = false;
        for (i, line) in schematic.lines().enumerate() {
            assert_eq!(5, line.len(), "Schematic should have 5 columns!");

            if i == 0 {
                is_key = line.chars().all(|c| c == '.');
            }

            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    let height = height_map.get(&j).unwrap_or(&-1);
                    height_map.insert(j, height + 1);
                }
            }
        }

        if is_key {
            keys.push(height_map);
        } else {
            locks.push(height_map);
        }
    }

    (locks, keys)
}

fn is_fit(lock: &HeightMap, key: &HeightMap) -> bool {
    (0..lock.len()).all(|i| lock.get(&i).unwrap() + key.get(&i).unwrap() <= 5)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (locks, keys) = parse_input(input);
    let total_fit = locks
        .iter()
        .flat_map(|lock| keys.iter().map(move |key| (lock, key)))
        .filter(|(lock, key)| is_fit(lock, key))
        .count();

    Some(total_fit as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
