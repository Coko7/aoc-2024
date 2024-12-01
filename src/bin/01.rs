use std::ops::Sub;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut parts = line.split(";");
        let first: u32 = parts.nth(0).unwrap().parse().unwrap();
        left.push(first);

        let second: u32 = parts.last().unwrap().parse().unwrap();
        right.push(second);
    }

    let mut res = 0;
    for i in 0..left.len() {
        let mut min_left = 0;
        let mut min_right = 0;

        if let Some(&min) = left.iter().min() {
            min_left = min;
            if let Some(idx) = left.iter().position(|&el| el == min) {
                left.remove(idx);
            }
        }

        if let Some(&min) = right.iter().min() {
            min_right = min;
            if let Some(idx) = right.iter().position(|&el| el == min) {
                right.remove(idx);
            }
        }

        let diff = abs_diff(min_left, min_right);
        res = res + diff;
    }

    Some(res)
}

fn abs_diff<T: PartialOrd + Sub<Output = T>>(a: T, b: T) -> T {
    if a > b {
        a - b
    } else {
        b - a
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut parts = line.split(";");
        let first: u32 = parts.nth(0).unwrap().parse().unwrap();
        left.push(first);

        let second: u32 = parts.last().unwrap().parse().unwrap();
        right.push(second);
    }

    let mut res = 0;
    for num in left.iter() {
        let occurences: u32 = right
            .iter()
            .filter(|&x| x == num)
            .count()
            .try_into()
            .unwrap();
        let score = num * occurences;
        res = res + score;
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
