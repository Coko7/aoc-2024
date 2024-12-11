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
        if *stone == 0 {
            res.push(1);
            continue;
        }

        let str_val = stone.to_string();
        if str_val.len() % 2 == 0 {
            let (left, right) = str_val.split_at(str_val.len() / 2);

            let left = left.parse::<u64>().unwrap();
            let right = right.parse::<u64>().unwrap();

            res.push(left);
            res.push(right);
            continue;
        }

        res.push(*stone * 2024);
    }

    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones = init_stones(input);

    for _ in 0..25 {
        stones = blink(&stones);
    }

    Some(stones.len().try_into().unwrap())
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
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
