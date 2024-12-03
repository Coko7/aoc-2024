advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // /!\ Input has been pre-processed using grep/sed /!\
    //
    // grep -o "mul([0-9]*,[0-9]*)" 03_og.txt | sed 's/mul(\(.*\))/\1/' > 03-1.txt
    let mut sum = 0;
    for line in input.lines() {
        match line {
            "do()" => {}
            "don't()" => {}
            val => {
                sum = sum + parse_mul(val);
            }
        }
    }

    Some(sum.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    // /!\ Input has been pre-processed using grep/sed /!\
    //
    // grep -o "do()\|don't()\|mul([0-9]*,[0-9]*)" 03_og.txt | sed 's/mul(\(.*\))/\1/' > 03-2.txt
    let mut enabled = true;
    let mut sum = 0;
    for line in input.lines() {
        match line {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            val if enabled => {
                let res = parse_mul(val);
                sum = sum + res;
            }
            _ => {}
        }
    }

    Some(sum.try_into().unwrap())
}

pub fn parse_mul(line: &str) -> i32 {
    let nums: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    let res = nums.first().unwrap() * nums.last().unwrap();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
