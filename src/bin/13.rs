advent_of_code::solution!(13);

#[derive(Debug)]
struct Machine {
    btn_a: (u32, u32),
    btn_b: (u32, u32),
    prize: (u32, u32),
}

fn parse_button(input: &str) -> (u32, u32) {
    let parts: Vec<u32> = input
        .split(",")
        .map(|p| p.split("+").last().unwrap().parse::<u32>().unwrap())
        .collect();

    assert_eq!(parts.len(), 2);

    (*parts.first().unwrap(), *parts.last().unwrap())
}

fn parse_prize(input: &str) -> (u32, u32) {
    let parts: Vec<u32> = input
        .split(",")
        .map(|p| p.split("=").last().unwrap().parse::<u32>().unwrap())
        .collect();
    assert_eq!(parts.len(), 2);

    (*parts.first().unwrap(), *parts.last().unwrap())
}

impl Machine {
    fn new(input: &str) -> Machine {
        let mut lines = input.lines();

        let btn_a = parse_button(lines.next().unwrap());
        let btn_b = parse_button(lines.next().unwrap());
        let prize = parse_prize(lines.next().unwrap());

        Machine {
            btn_a,
            btn_b,
            prize,
        }
    }

    fn new_machines(input: &str) -> Vec<Machine> {
        input.split("\n\n").map(|s| Machine::new(s)).collect()
    }

    fn try_solve(&self) -> Option<(u32, u32)> {
        for i in 0..10_000 {
            for j in 0..10_000 {
                let x = self.btn_a.0 * i + self.btn_b.0 * j;
                let y = self.btn_a.1 * i + self.btn_b.1 * j;

                if x > self.prize.0 || y > self.prize.1 {
                    break;
                }

                if x == self.prize.0 && y == self.prize.1 {
                    return Some((i, j));
                }
            }
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let machines = Machine::new_machines(input);

    let mut tokens = 0;
    for machine in machines.iter() {
        if let Some(sol) = machine.try_solve() {
            tokens += sol.0 * 3 + sol.1 * 1
        }
    }

    Some(tokens)
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
