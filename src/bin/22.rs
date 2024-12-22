advent_of_code::solution!(22);

fn evolve_nth(secret: u64, n: usize) -> u64 {
    (0..n).fold(secret, |acc, _| evolve(acc))
}

fn evolve(secret: u64) -> u64 {
    let secret = ((secret * 64) ^ secret) % (1_u64 << 24);
    let secret = ((secret / 32) ^ secret) % (1_u64 << 24);
    ((secret * 2048) ^ secret) % (1_u64 << 24)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut res = 0;
    for secret in input.lines().map(|l| l.parse::<u64>().unwrap()) {
        let sec_2000 = evolve_nth(secret, 2000);
        res += sec_2000;
        println!("{}: {}", sec_2000, res);
    }

    Some(res)
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
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
