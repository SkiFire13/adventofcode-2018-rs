#![allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<Sample>, Vec<Instruction>);

type Instruction = [usize; 4];

#[derive(Clone, Copy)]
pub struct Sample {
    before: [usize; 4],
    instruction: Instruction,
    after: [usize; 4],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum OpCode {
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori,
    Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr
}
use OpCode::*;
static OPCODES: [OpCode; 16] = [
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori,
    Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr
];

fn positives(sample: Sample, opcodes: &[OpCode]) -> impl Iterator<Item = OpCode> + '_ {
    opcodes
        .iter()
        .copied()
        .filter(move |&opcode| {
            let mut regs = sample.before;
            let [_, a, b, c] = sample.instruction;
            exec_opcode(opcode, a, b, c, &mut regs);
            regs == sample.after
        })
}

fn exec_opcode(opcode: OpCode, a: usize, b: usize, c: usize, regs: &mut [usize; 4]) {
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
    }
}

fn parse_4_usize<'a>(mut iter: impl Iterator<Item = &'a str>) -> [usize; 4] {
    [
        iter.next().expect("Invalid input").parse().expect("Invalid input"),
        iter.next().expect("Invalid input").parse().expect("Invalid input"),
        iter.next().expect("Invalid input").parse().expect("Invalid input"),
        iter.next().expect("Invalid input").parse().expect("Invalid input"),
    ]
}

pub fn input_generator(input: &str) -> Input {
    let mut split = input.splitn(2, "\n\n\n\n");
    let input_1 = split.next().expect("Invalid input");
    let input_2 = split.next().expect("Invalid input");

    let samples = input_1.split("\n\n")
        .map(|sample| {
            let mut lines = sample.lines();

            let before = lines.next().expect("Invalid input");
            let before = parse_4_usize(before[9..before.len()-1].split(", "));

            let instruction = lines.next().expect("Invalid input");
            let instruction = parse_4_usize(instruction.split(" "));

            let after = lines.next().expect("Invalid input");
            let after = parse_4_usize(after[9..after.len()-1].split(", "));

            Sample { before, instruction, after }
        })
        .collect();

    let instructions = input_2.lines()
        .map(|line| parse_4_usize(line.split(" ")))
        .collect();
    
    (samples, instructions)
}

pub fn part1(input: &Input) -> usize {
    let (samples, _) = input;
    samples
        .iter()
        .filter(|&&sample| positives(sample, &OPCODES).count() >= 3)
        .count()
}

pub fn part2(input: &Input) -> usize {
    let (samples, instructions) = input;
    let mut candidates = (0..16).map(|_| OPCODES.to_vec()).collect::<Vec<_>>();
    for &sample in samples {
        let n = sample.instruction[0];
        candidates[n] = positives(sample, &candidates[n]).collect();
    }

    let mut opcode_map = HashMap::new();
    let mut opcodes_found = Vec::new();
    while opcode_map.len() != 16 {
        for (n, opcodes) in candidates.iter_mut().enumerate() {
            opcodes.retain(|opcode| !opcodes_found.contains(opcode));
            if opcodes.len() == 1 {
                let opcode = opcodes[0];
                opcodes_found.push(opcode);
                opcode_map.insert(n, opcode);
            }
        }
    }

    let mut registers = [0; 4];
    for &[opcode, a, b, c] in instructions {
        let opcode = opcode_map[&opcode];
        exec_opcode(opcode, a, b, c, &mut registers);
    }
    registers[0]
}
