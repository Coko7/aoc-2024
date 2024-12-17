use core::panic;
use std::usize;

advent_of_code::solution!(17);

#[derive(Debug)]
struct Computer {
    // A,B,C registers
    a_reg: u64,
    b_reg: u64,
    c_reg: u64,
    // Program Counter
    pc: usize,
    std_in: Vec<u8>,
    std_out: Vec<u8>,
}

impl Computer {
    fn from_input(input: &str) -> Computer {
        assert_eq!(input.lines().count(), 5, "There should be five lines!");

        let mut lines = input.lines();
        let a_reg = Computer::parse_reg_def(lines.next().unwrap(), "Register A:");
        let b_reg = Computer::parse_reg_def(lines.next().unwrap(), "Register B:");
        let c_reg = Computer::parse_reg_def(lines.next().unwrap(), "Register C:");

        lines.next();

        let std_in = Computer::parse_program_input(lines.next().unwrap());
        assert_eq!(
            std_in.len() % 2,
            0,
            "There should be an even number of instr-operands"
        );

        Computer {
            a_reg,
            b_reg,
            c_reg,
            pc: 0,
            std_in,
            std_out: Vec::new(),
        }
    }

    fn program_len(&self) -> usize {
        self.std_in.len()
    }

    fn reset(&mut self) {
        self.a_reg = 0;
        self.b_reg = 0;
        self.c_reg = 0;
        self.pc = 0;
        self.std_out = Vec::new();
    }

    fn parse_reg_def(input: &str, prefix: &str) -> u64 {
        input
            .strip_prefix(prefix)
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap()
    }

    fn parse_program_input(input: &str) -> Vec<u8> {
        input
            .strip_prefix("Program:")
            .unwrap()
            .trim()
            .split(',')
            .map(|num_s| num_s.parse::<u8>().unwrap())
            .collect()
    }

    fn process_operand(&self, operand: u8, is_combo: bool) -> u64 {
        assert!(operand < 8, "Operand is not 3-bit: {}", operand);

        match is_combo {
            false => operand as u64,
            true => match operand {
                0..=3 => operand as u64,
                4 => self.a_reg,
                5 => self.b_reg,
                6 => self.c_reg,
                7 => panic!("7 is reserved and should never appear!"),
                val => panic!("{} is not a valid 3-bit number", val),
            },
        }
    }

    fn out(&self) -> String {
        join_u8_vec(&self.std_out)
    }

    fn run_until_end(&mut self) {
        while self.pc < self.std_in.len() {
            self.run_one_cycle();
        }
    }

    fn run_one_cycle(&mut self) {
        let cur_pc = self.pc;
        self.pc += 2;

        let op_code = self.std_in[cur_pc];
        let instruction = Instr::from_opcode(op_code);
        let operand = self.std_in[cur_pc + 1];

        // if instruction == Instr::JNZ {
        //     println!("A: {}", self.a_reg);
        //     println!("A8: {}", self.a_reg % 8);
        // }

        self.execute_instruction(instruction, operand);
        // println!("{}{} => {:?}", op_code, operand, self);
    }

    fn execute_instruction(&mut self, instruction: Instr, operand: u8) {
        match instruction {
            Instr::ADV => self.a_reg /= 2_u64.pow(self.process_operand(operand, true) as u32),
            Instr::BXL => self.b_reg ^= self.process_operand(operand, false),
            Instr::BST => self.b_reg = self.process_operand(operand, true) % 8,
            Instr::JNZ => {
                if self.a_reg != 0 {
                    self.pc = self.process_operand(operand, false) as usize;
                }
            }
            Instr::BXC => self.b_reg ^= self.c_reg,
            Instr::OUT => self
                .std_out
                .push((self.process_operand(operand, true) % 8) as u8),
            Instr::BDV => {
                self.b_reg = self.a_reg / 2_u64.pow(self.process_operand(operand, true) as u32)
            }
            Instr::CDV => {
                self.c_reg = self.a_reg / 2_u64.pow(self.process_operand(operand, true) as u32)
            }
        }
    }
}

fn join_u8_vec(vec: &Vec<u8>) -> String {
    let str_nums: Vec<String> = vec.iter().map(|n| n.to_string()).collect();
    str_nums.join(",")
}

#[derive(PartialEq, Eq)]
enum Instr {
    ADV = 0,
    BXL = 1,
    BST = 2,
    JNZ = 3,
    BXC = 4,
    OUT = 5,
    BDV = 6,
    CDV = 7,
}

impl Instr {
    fn to_opcode(&self) -> u8 {
        match self {
            Instr::ADV => 0,
            Instr::BXL => 1,
            Instr::BST => 2,
            Instr::JNZ => 3,
            Instr::BXC => 4,
            Instr::OUT => 5,
            Instr::BDV => 6,
            Instr::CDV => 7,
        }
    }

    fn from_opcode(code: u8) -> Instr {
        assert!(code < 8, "OP_CODE should be 3-bit: {}", code);
        match code {
            0 => Self::ADV,
            1 => Self::BXL,
            2 => Self::BST,
            3 => Self::JNZ,
            4 => Self::BXC,
            5 => Self::OUT,
            6 => Self::BDV,
            7 => Self::CDV,
            _ => panic!("Unknown OP_CODE: {}", code),
        }
    }
}

fn solve_self_replication(computer: &mut Computer) -> u64 {
    let mut a_reg = 0;
    for i in 0..computer.program_len() {
        a_reg = find_a_for_iteration(a_reg, i, computer).unwrap();
    }
    a_reg
}

fn find_a_for_iteration(last_a: u64, iteration: usize, computer: &mut Computer) -> Option<u64> {
    let min = last_a * 8;
    let max = min + 64;

    'outer: for number in min..(max + 1) {
        computer.reset();
        computer.a_reg = number;
        computer.run_until_end();

        let sub_std_in = computer.std_in[computer.program_len() - iteration - 1..].to_vec();
        if computer.std_out != sub_std_in {
            continue 'outer;
        }

        return Some(number);
    }

    None
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::from_input(input);
    computer.run_until_end();
    Some(computer.out())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut computer = Computer::from_input(input);
    Some(solve_self_replication(&mut computer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
