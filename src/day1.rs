#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input = Vec<i32>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.parse().expect("Invalid input line")).collect()
}

pub fn part1(input: &Input) -> i32 {
    input.iter().sum()
}

pub fn part2(input: &Input) -> i32 {
    let mut sum = 0;
    let mut seen = HashSet::new();
    seen.insert(0);

    for &n in input.iter().cycle() {
        sum += n;
        if !seen.insert(sum) {
            break;
        }
    }

    sum
}
