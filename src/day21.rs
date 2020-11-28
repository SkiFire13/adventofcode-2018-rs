#[allow(unused_imports)]
use super::prelude::*;
type Input = (usize, Vec<Instruction>);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, FromStr)]
#[display(style = "lowercase")]
pub enum OpCode {
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori,
    Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Instruction {
    opcode: OpCode,
    a: usize,
    b: usize,
    c: usize,
}

pub fn run_next(ip: usize, instructions: &[Instruction], regs: &mut [usize; 6]) {
    if let Some(&instruction) = instructions.get(regs[ip]) {
        let Instruction { opcode, a, b, c} = instruction;
        regs[c] = match opcode {
            OpCode::Addr => regs[a] + regs[b],
            OpCode::Addi => regs[a] + b,
            OpCode::Mulr => regs[a] * regs[b],
            OpCode::Muli => regs[a] * b,
            OpCode::Banr => regs[a] & regs[b],
            OpCode::Bani => regs[a] & b,
            OpCode::Borr => regs[a] | regs[b],
            OpCode::Bori => regs[a] | b,
            OpCode::Setr => regs[a],
            OpCode::Seti => a,
            OpCode::Gtir => (a > regs[b]) as usize,
            OpCode::Gtri => (regs[a] > b) as usize,
            OpCode::Gtrr => (regs[a] > regs[b]) as usize,
            OpCode::Eqir => (a == regs[b]) as usize,
            OpCode::Eqri => (regs[a] == b) as usize,
            OpCode::Eqrr => (regs[a] == regs[b]) as usize,
        };
        regs[ip] += 1;
    }
}

pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let ip = lines.next().expect("Invalid input");
    let ip = ip[4..].parse().expect("Invalid input");

    let instructions = lines.map(|line| {
        let mut split = line.split_ascii_whitespace();
        let opcode = split.next().expect("Invalid input").parse().expect("Invalid input");
        let a = split.next().expect("Invalid input").parse().expect("Invalid input");
        let b = split.next().expect("Invalid input").parse().expect("Invalid input");
        let c = split.next().expect("Invalid input").parse().expect("Invalid input");
        Instruction { opcode, a, b, c }
    })
    .collect();

    (ip, instructions)
}

pub fn part1(input: &Input) -> usize {
    let &(ip, ref instructions) = input;
    let mut registers = [0; 6];
    while registers[ip] != 28 {
        run_next(ip, instructions, &mut registers);
    }
    registers[3]
}

pub fn part2(input: &Input) -> usize {
    let &(ip, ref instructions) = input;
    let mut registers = [0; 6];
    let mut seen = HashSet::new();
    let mut last_seen = None;
    loop {
        if registers[ip] == (instructions.len()-1) - 2 {
            if seen.insert(registers[3]) {
                last_seen = Some(registers[3]);
            } else {
                break;
            }
        }
        run_next(ip, instructions, &mut registers);
    }
    last_seen.unwrap()
}
