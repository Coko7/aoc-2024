use core::panic;
use std::usize;

use itertools::Itertools;

advent_of_code::solution!(7);

fn generate_permutations(symbols: &[char], length: usize) -> Vec<String> {
    std::iter::repeat(symbols.iter())
        .take(length)
        .multi_cartesian_product()
        .map(|v| v.into_iter().collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;

    for line in input.lines() {
        let parts: Vec<_> = line.split(": ").collect();
        if let Some(expected_result) = parts.first() {
            let expected_result: u64 = expected_result.parse().unwrap();
            let operands: Vec<u64> = parts
                .last()
                .unwrap()
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect();

            let operators_needed: usize = (operands.len() - 1).try_into().unwrap();
            let permutations = generate_permutations(&['+', '*'], operators_needed);

            for permutation in permutations {
                let mut equation_res: u64 = *operands.first().unwrap();

                for (idx, operand) in operands[1..].iter().enumerate() {
                    if equation_res > expected_result {
                        break;
                    }

                    let operator = permutation.chars().nth(idx).unwrap();
                    match operator {
                        '+' => equation_res += operand,
                        '*' => equation_res *= operand,
                        val => panic!("Unknown operator: {}", val),
                    }
                }

                if equation_res == expected_result {
                    result += expected_result;
                    break;
                }
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;

    for line in input.lines() {
        let parts: Vec<_> = line.split(": ").collect();
        if let Some(expected_result) = parts.first() {
            let expected_result: u64 = expected_result.parse().unwrap();
            let operands: Vec<u64> = parts
                .last()
                .unwrap()
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect();

            let operators_needed: usize = (operands.len() - 1).try_into().unwrap();
            let permutations = generate_permutations(&['+', '*', '|'], operators_needed);

            for permutation in permutations {
                let mut equation_res: u64 = *operands.first().unwrap();

                for (idx, operand) in operands[1..].iter().enumerate() {
                    if equation_res > expected_result {
                        break;
                    }

                    let operator = permutation.chars().nth(idx).unwrap();
                    match operator {
                        '+' => equation_res += operand,
                        '*' => equation_res *= operand,
                        '|' => {
                            equation_res = format!("{}{}", equation_res, operand).parse().unwrap()
                        }
                        val => panic!("Unknown operator: {}", val),
                    }
                }

                if equation_res == expected_result {
                    result += expected_result;
                    break;
                }
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
