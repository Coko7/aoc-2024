advent_of_code::solution!(13);

#[derive(Debug)]
struct Machine {
    btn_a: (u64, u64),
    btn_b: (u64, u64),
    prize: (u64, u64),
    prize_offset: u64,
}

fn parse_button(input: &str) -> (u64, u64) {
    let parts: Vec<u64> = input
        .split(",")
        .map(|p| p.split("+").last().unwrap().parse::<u64>().unwrap())
        .collect();

    assert_eq!(parts.len(), 2);
    (*parts.first().unwrap(), *parts.last().unwrap())
}

fn parse_prize(input: &str) -> (u64, u64) {
    let parts: Vec<u64> = input
        .split(",")
        .map(|p| p.split("=").last().unwrap().parse::<u64>().unwrap())
        .collect();

    assert_eq!(parts.len(), 2);
    (*parts.first().unwrap(), *parts.last().unwrap())
}

impl Machine {
    fn new(input: &str, prize_offset: u64) -> Machine {
        let mut lines = input.lines();

        let btn_a = parse_button(lines.next().unwrap());
        let btn_b = parse_button(lines.next().unwrap());
        let prize = parse_prize(lines.next().unwrap());

        Machine {
            btn_a,
            btn_b,
            prize,
            prize_offset,
        }
    }

    fn new_machines(input: &str, prize_offset: u64) -> Vec<Machine> {
        input
            .split("\n\n")
            .map(|s| Machine::new(s, prize_offset))
            .collect()
    }

    fn real_prize(&self) -> (u64, u64) {
        (
            self.prize.0 + self.prize_offset,
            self.prize.1 + self.prize_offset,
        )
    }

    fn find_optimal_presses(&self) -> Option<(u64, u64)> {
        let (a0, a1) = (self.btn_a.0 as f64, self.btn_a.1 as f64);
        let (b0, b1) = (self.btn_b.0 as f64, self.btn_b.1 as f64);

        // Real prize is relevant for part 2 where an offset is used
        // For part 1, offset is 0 so self.prize = self.real_prize()
        let real_prize = self.real_prize();
        let (z0, z1) = (real_prize.0 as f64, real_prize.1 as f64);

        // Isolate n and m in following equations:
        // - z0 = a0 * n + b0 * m
        // - z1 = a1 * n + b1 * m
        // Where:
        // - n: how many times to press A button
        // - m: how many times to press B button
        let n = (b1 * z0 - b0 * z1) / (a0 * b1 - a1 * b0);
        let m = (z0 - a0 * n) / b0;

        // If n and m have fractional part, that means no solution with 'integer' numbers
        if n.fract() != 0.0 || m.fract() != 0.0 {
            return None;
        }

        Some((n as u64, m as u64))
    }

    fn find_prize_cost(&self) -> Option<u64> {
        match self.find_optimal_presses() {
            // Pressing A button costs 3 tokens
            // Pressing B button costs 1 token
            Some((a, b)) => Some(a * 3 + b * 1),
            None => None,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        Machine::new_machines(input, 0)
            .iter()
            .map(|m| m.find_prize_cost().unwrap_or(0))
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        Machine::new_machines(input, 10000000000000)
            .iter()
            .map(|m| m.find_prize_cost().unwrap_or(0))
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
