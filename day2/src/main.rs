use std::time::Instant;

fn parse_input() -> Vec<i32> {
    std::fs::read_to_string("./data.txt")
        .unwrap()
        .split(',')
        .map(|n| n.trim().parse::<i32>().unwrap_or_else(|_| panic!("{}", n)))
        .collect()
}

fn run_prog(mut program: Vec<i32>) -> i32 {
    let mut pc = 0;

    loop {
        match program[pc] {
            1 => {
                let value = program[program[pc + 1] as usize] + program[program[pc + 2] as usize];
                let dest = program[pc + 3] as usize;
                program[dest] = value;
                pc += 4;
            }
            2 => {
                let value = program[program[pc + 1] as usize] * program[program[pc + 2] as usize];
                let dest = program[pc + 3] as usize;
                program[dest] = value;
                pc += 4;
            }
            99 => {
                return program[0];
            }
            a => panic!("Invalid opcode {}", a)
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
    println!("{:?}", parse_input());
    let program = parse_input();

    let start = Instant::now();
    let (noun, verb) = find_end_value(program);
    let end = Instant::now();
    println!("Found noun {} and verb {} in {:?}", noun, verb, end - start);
}
