use std::time::Instant;
use std::convert::TryFrom;
use num_traits::FromPrimitive;
use std::io::{self, prelude::*};
use std::path::Path;
use std::slice::Iter as SliceIter;

extern crate num_traits;
#[macro_use]
extern crate num_derive;

fn parse_input(path: &std::path::Path) -> Vec<i64> {
    std::fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|n| n.trim().parse::<i64>().unwrap_or_else(|_| panic!("{}", n)))
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
pub enum ParamMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
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
    SetBase = 9,
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
            OpcodeRoot::SetBase => 1,
            OpcodeRoot::Halt => 0,
        }
    }
}

#[derive(Debug)]
pub struct Opcode {
    pub root: OpcodeRoot,
    pub params: Vec<(ParamMode, i64)>,
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
    pub fn parse(data: &[i64]) -> Opcode {
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

    fn read(arg_type: ParamMode, arg: i64, program: &[i64], relative_base: i64) -> i64 {
        match arg_type {
            ParamMode::Position => {
                program[arg as usize]
            }
            ParamMode::Immediate => {
                arg
            }
            ParamMode::Relative => {
                program[(arg + relative_base) as usize]
            }
        }
    }

    fn write(value: i64, arg_type: ParamMode, arg: i64, program: &mut [i64], relative_base: i64) {
        match arg_type {
            ParamMode::Position => {
                program[arg as usize] = value;
            }
            ParamMode::Immediate => {
                panic!("Cannot write to immediate");
            }
            ParamMode::Relative => {
                program[(arg + relative_base) as usize] = value;
            }
        }
    }

    pub fn execute(&self, program: &mut [i64], input: &mut SliceIter<i64>, relative_base: &mut i64) -> (Option<usize>, Option<i64>) {
        match self.root {
            OpcodeRoot::Add => {
                let lhs = Opcode::read(self.params[0].0, self.params[0].1, program, *relative_base);
                let rhs = Opcode::read(self.params[1].0, self.params[1].1, program, *relative_base);
                let result = lhs + rhs;
                Opcode::write(result, self.params[2].0, self.params[2].1, program, *relative_base);
                (None, None)
            }
            OpcodeRoot::Multiply => {
                let lhs = Opcode::read(self.params[0].0, self.params[0].1, program, *relative_base);
                let rhs = Opcode::read(self.params[1].0, self.params[1].1, program, *relative_base);
                let result = lhs * rhs;
                Opcode::write(result, self.params[2].0, self.params[2].1, program, *relative_base);
                (None, None)
            }
            OpcodeRoot::Halt => {
                (None, None)
            }
            OpcodeRoot::Input => {
                Opcode::write(*input.next().unwrap(), self.params[0].0, self.params[0].1, program, *relative_base);
                (None, None)
            }
            OpcodeRoot::Output => {
                (None, Some(Opcode::read(self.params[0].0, self.params[0].1, program, *relative_base)))
            }
            OpcodeRoot::LessThan => {
                let lhs = Opcode::read(self.params[0].0, self.params[0].1, program, *relative_base);
                let rhs = Opcode::read(self.params[1].0, self.params[1].1, program, *relative_base);

                let result = lhs < rhs;
                Opcode::write(result as i64, self.params[2].0, self.params[2].1, program, *relative_base);
                (None, None)
            }
            OpcodeRoot::Equals => {
                let lhs = Opcode::read(self.params[0].0, self.params[0].1, program, *relative_base);
                let rhs = Opcode::read(self.params[1].0, self.params[1].1, program, *relative_base);

                let result = lhs == rhs;
                Opcode::write(result as i64, self.params[2].0, self.params[2].1, program, *relative_base);
                (None, None)
            }
            OpcodeRoot::JumpTrue => {
                let cond = Opcode::read(self.params[0].0, self.params[0].1, program, *relative_base);

                (if cond != 0 {
                    Some(Opcode::read(self.params[1].0, self.params[1].1, program, *relative_base) as usize)
                } else {
                    None
                }, None)
            }
            OpcodeRoot::JumpFalse => {
                let cond = Opcode::read(self.params[0].0, self.params[0].1, program, *relative_base);

                (if cond == 0 {
                    Some(Opcode::read(self.params[1].0, self.params[1].1, program, *relative_base) as usize)
                } else {
                    None
                }, None)
            }
            OpcodeRoot::SetBase => {
                *relative_base += Opcode::read(self.params[0].0, self.params[0].1, program, *relative_base);
                (None, None)
            }
        }
    }
}

struct ProgramState {
    pub program: Vec<i64>,
    pub input: Vec<i64>,
    pub input_idx: usize,
    pub pc: usize,
    pub relative_base: i64,
}

fn step_prog(state: &mut ProgramState) -> (Option<i64>, bool) {
    let ProgramState { program, input, input_idx, pc, relative_base } = state;

    let opcode = Opcode::parse(&program[*pc..]);
    let (new_pc, out) = opcode.execute(program, &mut input[*input_idx..].iter(), relative_base);
    if let Some(new_pc) = new_pc {
        *pc = new_pc;
    } else {
        *pc += 1 + opcode.root.arg_count();
    }

    if opcode.root == OpcodeRoot::Input {
        *input_idx += 1;
    }

    (out, opcode.root == OpcodeRoot::Halt)
}

fn run_prog(mut program: Vec<i64>, input: &[i64]) -> Vec<i64> {
    if program.len() < 10000 {
            program.resize(10000, 0);
    }

    let mut output = Vec::new();

    let mut state = ProgramState { pc: 0, input: input.iter().copied().collect::<Vec<_>>(), input_idx: 0, program, relative_base: 0 };

    loop {
        let (out, halted) = step_prog(&mut state);

        if let Some(out) = out {
            output.push(out);
        }

        if halted {
            return output;
        }
    }
}

fn try_sequence_feedback(program: Vec<i64>, sequence: &[i64]) -> i64 {
    let mut programs = sequence.iter().map(|n| {
        ProgramState {
            program: program.clone(),
            input: vec![*n],
            input_idx: 0,
            pc: 0,
            relative_base: 0,
        }
    }).collect::<Vec<_>>();

    let mut halted = false;

    let mut next_value = 0;

    let run_until_one = |prog: &mut ProgramState| -> (Option<i64>, bool) {
        loop {
            match step_prog(prog) {
                (None, false) => { /* Keep looping */ },
                r => break r,
            }
        }
    };

    'overall: loop {
        for i in 0..5 {
            programs[i].input.push(next_value);
            let (new_out, h) = run_until_one(&mut programs[i]);
            if new_out == None { break 'overall; }
            println!("{} Output is now {}", i, next_value);
            next_value = new_out.unwrap();
        }
    }

    next_value
}

fn try_sequence(program: Vec<i64>, sequence: &[i64]) -> i64 {
    assert_eq!(sequence.len(), 5);

    let mut output = 0;

    for s in sequence.iter() {
        let new_output = run_prog(program.clone(), &[*s, output]);
        println!("Output is now {}", output);
        assert_eq!(new_output.len(), 1);
        output = new_output[0];
    }

    println!("");

    output
}

fn try_sequence_state(program: Vec<i64>, sequence: &[i64]) -> i64 {
    assert_eq!(sequence.len(), 5);

    let mut output = 0;

    for s in sequence.iter() {
        let new_output = run_prog(program.clone(), &[*s, output]);
        assert_eq!(new_output.len(), 1);
        output = new_output[0];
    }

    output
}


fn find_highest(program: Vec<i64>) -> i64 {
    let mut max_val = 0;

    for a in 0..5 {
        for b in (0..5).filter(|&n| n != a) {
            for c in (0..5).filter(|&n| n != a && n != b) {
                for d in (0..5).filter(|&n| n != a && n != b && n != c) {
                    let e = (0..5).find(|&n| n != a && n != b && n != c && n != d).unwrap();
                    let result = try_sequence_feedback(program.clone(), &[a+5, b+5, c+5, d+5, e+5]);
                    if max_val < result {
                        max_val = result;
                    }
                }
            }
        }
    }

    max_val
}

fn main() {
    assert_eq!(run_prog(parse_input(Path::new("./day5test.txt")), &[1]), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 16348437]);

    let prog = parse_input(Path::new("./quine.txt"));

    let output = run_prog(prog, &[]);

    println!("Output: {:?}", output);

    let boost = parse_input(Path::new("./input.txt"));

    println!("Boost: {:?}", run_prog(boost, &[2]));
}
