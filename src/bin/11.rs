use std::{collections::HashMap, usize};

advent_of_code::solution!(11);

fn init_stones(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    let mut res = Vec::new();
    for stone in stones.iter() {
        match stone {
            0 => res.push(1),
            val if val.to_string().len() % 2 == 0 => {
                let str_val = val.to_string();
                let (left, right) = str_val.split_at(str_val.len() / 2);

                let left = left.parse::<u64>().unwrap();
                let right = right.parse::<u64>().unwrap();

                res.push(left);
                res.push(right);
            }
            val => res.push(val * 2024),
        }
    }
    res
}

fn count_blink(stone: u64, iteration: u32, cache: &mut HashMap<(u64, u32), usize>) -> usize {
    if let Some(&count) = cache.get(&(stone, iteration)) {
        return count;
    }

    if iteration == 0 {
        return 1;
    }

    let count = match stone {
        0 => count_blink(1, iteration - 1, cache),
        val if val.to_string().len() % 2 == 0 => {
            let str_val = val.to_string();
            let (left, right) = str_val.split_at(str_val.len() / 2);

            let left = left.parse::<u64>().unwrap();
            let right = right.parse::<u64>().unwrap();

            count_blink(left, iteration - 1, cache) + count_blink(right, iteration - 1, cache)
        }
        val => count_blink(val * 2024, iteration - 1, cache),
    };

    if cache.get(&(stone, iteration)).is_none() {
        cache.insert((stone, iteration), count);
    }

    count
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut cache: HashMap<(u64, u32), usize> = HashMap::new();
    Some(
        init_stones(input)
            .iter()
            .fold(0, |acc, &stone| acc + count_blink(stone, 25, &mut cache)),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cache: HashMap<(u64, u32), usize> = HashMap::new();
    Some(
        init_stones(input)
            .iter()
            .fold(0, |acc, &stone| acc + count_blink(stone, 75, &mut cache)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
