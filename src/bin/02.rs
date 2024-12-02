use std::ops::Sub;

advent_of_code::solution!(2);

pub fn sign(num: i32) -> i32 {
    if num >= 0 {
        1
    } else {
        -1
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = input.lines();

    let mut safe_reports = 0;
    for report in reports {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        if is_safe(&levels) {
            safe_reports = safe_reports + 1;
            continue;
        }
    }

    Some(safe_reports)
}

fn remove_nth<T: Clone>(vec: Vec<T>, n: usize) -> Vec<T> {
    vec.into_iter()
        .enumerate()
        .filter_map(|(i, item)| if i == n { None } else { Some(item) })
        .collect()
}

pub fn is_safe(levels: &Vec<i32>) -> bool {
    let mut trend = None;
    let mut prev: Option<i32> = None;

    for (i, level) in levels.iter().enumerate() {
        if i == 0 {
            prev = Some(*level);
            continue;
        }

        let cur = level;

        let diff: i32 = (prev.unwrap() - cur).try_into().unwrap();
        let diff_a = if diff > 0 { diff } else { diff * -1 };
        prev = Some(*level);

        if trend.is_none() {
            trend = Some(sign(diff));
        } else {
            let trend = trend.unwrap();
            if sign(trend) != sign(diff) {
                return false;
            }
        }

        if diff_a < 1 || diff_a > 3 {
            return false;
        }
    }

    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = input.lines();

    let mut safe_reports = 0;
    for report in reports {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        if is_safe(&levels) {
            safe_reports = safe_reports + 1;
            continue;
        }

        for i in 0..levels.len() {
            let modified = remove_nth(levels.clone(), i);
            if is_safe(&modified) {
                println!("{:?}", modified);
                safe_reports = safe_reports + 1;
                break;
            }
        }
    }

    Some(safe_reports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
