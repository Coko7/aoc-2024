advent_of_code::solution!(13);

#[derive(Debug)]
struct Machine {
    btn_a: (u64, u64),
    btn_b: (u64, u64),
    prize: (u64, u64),
    offset: u64,
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
    fn new(input: &str, offset: u64) -> Machine {
        let mut lines = input.lines();

        let btn_a = parse_button(lines.next().unwrap());
        let btn_b = parse_button(lines.next().unwrap());
        let prize = parse_prize(lines.next().unwrap());

        Machine {
            btn_a,
            btn_b,
            prize,
            offset,
        }
    }

    fn real_prize(&self) -> (u64, u64) {
        (self.prize.0 + self.offset, self.prize.1 + self.offset)
    }

    fn new_machines(input: &str, offset: u64) -> Vec<Machine> {
        input
            .split("\n\n")
            .map(|s| Machine::new(s, offset))
            .collect()
    }

    fn calc_presses(&self, a: u64, b: u64) -> (u64, u64) {
        let x = self.btn_a.0 * a + self.btn_b.0 * b;
        let y = self.btn_a.1 * a + self.btn_b.1 * b;
        (x, y)
    }

    fn try_smart_solve(&self) -> Option<(u64, u64)> {
        let (a0, a1) = (self.btn_a.0 as f64, self.btn_a.1 as f64);
        let (b0, b1) = (self.btn_b.0 as f64, self.btn_b.1 as f64);

        let prize = self.real_prize();
        let (z0, z1) = (prize.0 as f64, prize.1 as f64);

        let a_presses = (b1 * z0 - b0 * z1) / (a0 * b1 - a1 * b0);
        let b_presses = (z0 - a0 * a_presses) / b0;

        if a_presses.fract() != 0.0 || b_presses.fract() != 0.0 {
            return None;
        }

        Some((a_presses as u64, b_presses as u64))
    }

    fn try_solve(&self) -> Option<(u64, u64)> {
        let mut a = self.offset;
        let mut b = self.offset;

        loop {
            loop {
                let x = self.btn_a.0 * a + self.btn_b.0 * b;
                let y = self.btn_a.1 * a + self.btn_b.1 * b;

                let prize = self.real_prize();
                if x > prize.0 || y > prize.1 {
                    break;
                }

                if x == prize.0 && y == prize.1 {
                    return Some((a, b));
                }

                b += 1;
            }
            a += 1;
            b = 0;

            let res = self.calc_presses(a, b);
            if res.0 > self.prize.0 || res.1 > self.prize.1 {
                break;
            }
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = Machine::new_machines(input, 0);

    let mut tokens = 0;
    for machine in machines.iter() {
        if let Some(sol) = machine.try_smart_solve() {
            tokens += sol.0 * 3 + sol.1 * 1
        }
    }

    Some(tokens)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = Machine::new_machines(input, 10000000000000);

    let mut tokens = 0;
    for machine in machines.iter() {
        if let Some(sol) = machine.try_smart_solve() {
            tokens += sol.0 * 3 + sol.1 * 1
        }
    }

    Some(tokens)
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
        assert_eq!(result, None);
    }
}
