use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(24);

type Wire = String;

#[derive(Debug)]
enum Gate {
    AND = 0,
    OR = 1,
    XOR = 2,
}

#[derive(Debug)]
enum GateParseError {
    InvalidValue,
}

impl Gate {
    fn execute(&self, a: bool, b: bool) -> bool {
        match self {
            Gate::AND => a & b,
            Gate::OR => a | b,
            Gate::XOR => a ^ b,
        }
    }
}

impl FromStr for Gate {
    type Err = GateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "AND" => Ok(Self::AND),
            "OR" => Ok(Self::OR),
            "XOR" => Ok(Self::XOR),
            _ => Err(GateParseError::InvalidValue),
        }
    }
}

fn process_input(
    input: &str,
) -> (
    HashMap<Wire, Option<bool>>,
    HashMap<Wire, (Wire, Wire, Gate)>,
) {
    let (val_defs, dep_defs) = input.split_once("\n\n").unwrap();

    let mut wire_values = HashMap::new();
    for val_def in val_defs.lines() {
        let (wire, val) = val_def.split_once(":").unwrap();
        let val: bool = val.trim().parse::<u32>().unwrap() == 1;
        wire_values.insert(wire.to_string(), Some(val));
    }

    let mut dependencies = HashMap::new();
    for dep_def in dep_defs.lines() {
        let (ins, out) = dep_def.split_once("->").unwrap();
        let out = out.trim().to_string();

        let mut parts = ins.trim().split(" ");

        let wire1 = parts.next().unwrap().to_string();
        let gate = Gate::from_str(parts.next().unwrap()).unwrap();
        let wire2 = parts.next().unwrap().to_string();

        dependencies.insert(out, (wire1, wire2, gate));
    }

    (wire_values, dependencies)
}

fn compute_wire(
    wire: Wire,
    wire_values: &mut HashMap<Wire, Option<bool>>,
    dependencies: &HashMap<Wire, (Wire, Wire, Gate)>,
) {
    let (in1, in2, gate) = dependencies.get(&wire).unwrap();

    let in1_val = *wire_values.get(in1).unwrap_or(&None);
    let in2_val = *wire_values.get(in2).unwrap_or(&None);

    if in1_val.is_none() {
        compute_wire(in1.to_string(), wire_values, dependencies);
    }
    if in2_val.is_none() {
        compute_wire(in2.to_string(), wire_values, dependencies);
    }

    let in1_val = wire_values.get(in1).unwrap().unwrap();
    let in2_val = wire_values.get(in2).unwrap().unwrap();

    let wire_val = gate.execute(in1_val, in2_val);
    wire_values.insert(wire.to_string(), Some(wire_val));
}

fn compute_z_wires(
    wire_values: &mut HashMap<Wire, Option<bool>>,
    dependencies: &HashMap<Wire, (Wire, Wire, Gate)>,
) {
    for (wire, _deps) in dependencies.iter() {
        if wire.starts_with('z') {
            compute_wire(wire.to_string(), wire_values, dependencies);
        }
    }
}

fn get_z_num_val(wire_values: &HashMap<Wire, Option<bool>>) -> u64 {
    let z_wires: HashMap<u32, Option<bool>> = wire_values
        .iter()
        .filter(|(wire, _)| wire.starts_with('z'))
        .map(|(k, v)| (k.strip_prefix('z').unwrap().parse::<u32>().unwrap(), *v))
        .collect();

    let mut res = 0;
    for (&pos, &val) in z_wires.iter() {
        let val = val.map_or(0, |b| if b { 1 } else { 0 });
        res += val << pos;
    }
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut wire_values, dependencies) = process_input(input);
    compute_z_wires(&mut wire_values, &dependencies);
    let z_num = get_z_num_val(&wire_values);
    Some(z_num)
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
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
