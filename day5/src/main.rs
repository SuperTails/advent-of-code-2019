use std::time::Instant;
use std::convert::TryFrom;
use num_traits::FromPrimitive;
use std::io::{self, prelude::*};
use std::path::Path;

extern crate num_traits;
#[macro_use]
extern crate num_derive;

fn parse_input(path: &std::path::Path) -> Vec<i32> {
    std::fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|n| n.trim().parse::<i32>().unwrap_or_else(|_| panic!("{}", n)))
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
pub enum ParamMode {
    Position = 0,
    Immediate = 1,
}

#[derive(Debug, Copy, Clone, FromPrimitive, PartialEq)]
pub enum OpcodeRoot {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpTrue = 5,
    JumpFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}

impl OpcodeRoot {
    pub fn arg_count(&self) -> usize {
        match self {
            OpcodeRoot::Multiply => 3,
            OpcodeRoot::Add => 3,
            OpcodeRoot::Input => 1,
            OpcodeRoot::Output => 1,
            OpcodeRoot::JumpTrue => 2,
            OpcodeRoot::JumpFalse => 2,
            OpcodeRoot::LessThan => 3,
            OpcodeRoot::Equals => 3,
            OpcodeRoot::Halt => 0,
        }
    }
}

#[derive(Debug)]
pub struct Opcode {
    pub root: OpcodeRoot,
    pub params: Vec<(ParamMode, i32)>,
}

// MSD to LSD
fn get_digits(mut value: u32) -> Vec<u32> {
    let mut result = Vec::new();

    while value != 0 {
        result.push(value % 10);
        value /= 10;
    }

    result
}

impl Opcode {
    pub fn parse(data: &[i32]) -> Opcode {
        let mut code_digits = get_digits(u32::try_from(data[0]).unwrap());
        if code_digits.len() < 2 {
            code_digits.resize(2, 0);
        }

        let root = {
            let id = code_digits[0] + 10 * code_digits[1];
            OpcodeRoot::from_u32(id).unwrap_or_else(|| panic!("Invalid root {}", id))
        };

        let mut params = Vec::new();

        let code_digits = code_digits[2..].iter().chain(std::iter::once(&0).cycle()).map(|d| ParamMode::from_u32(*d).unwrap());

        for (arg, arg_type) in data[1..][0..root.arg_count()].iter().zip(code_digits) {
            params.push((arg_type, *arg));
        }

        Opcode {
            root,
            params,
        }
    }

    fn read(arg_type: ParamMode, arg: i32, program: &[i32]) -> i32 {
        match arg_type {
            ParamMode::Position => {
                program[arg as usize]
            }
            ParamMode::Immediate => {
                arg
            }
        }
    }

    fn write(value: i32, arg_type: ParamMode, arg: i32, program: &mut [i32]) {
        match arg_type {
            ParamMode::Position => {
                program[arg as usize] = value;
            }
            ParamMode::Immediate => {
                panic!("Cannot write to immediate");
            }
        }
    }

    pub fn execute(&self, program: &mut [i32]) -> Option<usize> {
        match self.root {
            OpcodeRoot::Add => {
                let lhs = Opcode::read(self.params[0].0, self.params[0].1, program);
                let rhs = Opcode::read(self.params[1].0, self.params[1].1, program);
                let result = lhs + rhs;
                Opcode::write(result, self.params[2].0, self.params[2].1, program);
                None
            }
            OpcodeRoot::Multiply => {
                let lhs = Opcode::read(self.params[0].0, self.params[0].1, program);
                let rhs = Opcode::read(self.params[1].0, self.params[1].1, program);
                let result = lhs * rhs;
                Opcode::write(result, self.params[2].0, self.params[2].1, program);
                None
            }
            OpcodeRoot::Halt => {
                println!("Halted with value at position 0: {}", program[0]);
                None
            }
            OpcodeRoot::Input => {
                print!("Input: ");
                io::stdout().flush().unwrap();
                let input = {
                    let mut s = String::new();
                    io::stdin().read_line(&mut s).unwrap();
                    s.trim().parse::<i32>().unwrap()
                };

                Opcode::write(input, self.params[0].0, self.params[0].1, program);
                None
            }
            OpcodeRoot::Output => {
                println!("Output: {}", Opcode::read(self.params[0].0, self.params[0].1, program));
                None
            }
            OpcodeRoot::LessThan => {
                let lhs = Opcode::read(self.params[0].0, self.params[0].1, program);
                let rhs = Opcode::read(self.params[1].0, self.params[1].1, program);

                let result = lhs < rhs;
                Opcode::write(result as i32, self.params[2].0, self.params[2].1, program);
                None
            }
            OpcodeRoot::Equals => {
                let lhs = Opcode::read(self.params[0].0, self.params[0].1, program);
                let rhs = Opcode::read(self.params[1].0, self.params[1].1, program);

                let result = lhs == rhs;
                Opcode::write(result as i32, self.params[2].0, self.params[2].1, program);
                None
            }
            OpcodeRoot::JumpTrue => {
                let cond = Opcode::read(self.params[0].0, self.params[0].1, program);

                if cond != 0 {
                    Some(Opcode::read(self.params[1].0, self.params[1].1, program) as usize)
                } else {
                    None
                }
            }
            OpcodeRoot::JumpFalse => {
                let cond = Opcode::read(self.params[0].0, self.params[0].1, program);

                if cond == 0 {
                    Some(Opcode::read(self.params[1].0, self.params[1].1, program) as usize)
                } else {
                    None
                }
            }
        }
    }
}

fn run_prog(mut program: Vec<i32>) -> i32 { let mut pc = 0;

    loop {
        let opcode = Opcode::parse(&program[pc..]);
        println!("Running {:?}", opcode);
        if let Some(new_pc) = opcode.execute(&mut program) {
            pc = new_pc;
        } else {
            pc += 1 + opcode.root.arg_count();
        }

        if opcode.root == OpcodeRoot::Halt {
            return program[0];
        }
    }
}

fn run_with_args(noun: i32, verb: i32, mut p: Vec<i32>) -> i32 {
    p[1] = noun;
    p[2] = verb;

    run_prog(p)
}

fn find_end_value(program: Vec<i32>) -> (i32, i32) {
    for noun in 0..100 {
        for verb in 0..100 {
            let result = run_with_args(noun, verb, program.clone());

            if result == 19690720 {
                return (noun, verb);
            }
        }
    }

    unreachable!()
}

fn main() {
    assert_eq!(run_prog(parse_input(Path::new("data2.txt"))), 3500);

    let program = parse_input(Path::new("./data3.txt"));
    println!("{:?}", program);
    
    run_prog(program);
}
