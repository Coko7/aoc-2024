use core::panic;

advent_of_code::solution!(17);

#[derive(Debug)]
struct Computer {
    // A,B,C registers
    a_reg: u32,
    b_reg: u32,
    c_reg: u32,
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

    fn parse_reg_def(input: &str, prefix: &str) -> u32 {
        input
            .strip_prefix(prefix)
            .unwrap()
            .trim()
            .parse::<u32>()
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

    fn process_operand(&self, operand: u8, is_combo: bool) -> u32 {
        assert!(operand < 8, "Operand is not 3-bit: {}", operand);

        match is_combo {
            false => operand as u32,
            true => match operand {
                0..=3 => operand as u32,
                4 => self.a_reg,
                5 => self.b_reg,
                6 => self.c_reg,
                7 => panic!("7 is reserved and should never appear!"),
                val => panic!("{} is not a valid 3-bit number", val),
            },
        }
    }

    fn out(&self) -> String {
        let out_nums: Vec<String> = self.std_out.iter().map(|n| n.to_string()).collect();
        out_nums.join(",")
    }

    fn run(&mut self) {
        while self.pc < self.std_in.len() {
            let cur_pc = self.pc;
            self.pc += 2;

            let op_code = self.std_in[cur_pc];
            let instruction = Instr::from_opcode(op_code);
            let operand = self.std_in[cur_pc + 1];

            self.execute_instruction(instruction, operand);
        }
    }

    fn execute_instruction(&mut self, instruction: Instr, operand: u8) {
        match instruction {
            Instr::ADV => self.a_reg /= 2_u32.pow(self.process_operand(operand, true)),
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
            Instr::BDV => self.b_reg = self.a_reg / 2_u32.pow(self.process_operand(operand, true)),
            Instr::CDV => self.c_reg = self.a_reg / 2_u32.pow(self.process_operand(operand, true)),
        }
    }
}

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

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::from_input(input);
    computer.run();
    Some(computer.out())
}

pub fn part_two(input: &str) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}