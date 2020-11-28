#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input = String;

pub fn input_generator(input: &str) -> Input { input.to_string() }

fn collapse(input: impl Iterator<Item = char>) -> usize {
    fn are_opposite(c1: char, c2: char) -> bool {
        return c1 != c2 && c1.to_ascii_lowercase() == c2.to_ascii_lowercase();
    }

    let mut stack = Vec::new();

    for c in input {
        match stack.last() {
            Some(&last) if are_opposite(c, last) => {
                stack.pop();
            }
            _ => {
                stack.push(c);
            }
        }
    }

    stack.len()
}

pub fn part1(input: &Input) -> usize {
    collapse(input.chars())
}

pub fn part2(input: &Input) -> usize {
    input
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .unique()
        .map(|c_lower| {
            let c_upper = c_lower.to_ascii_uppercase();
            collapse(input.chars().filter(|&x| x != c_lower && x != c_upper))
        })
        .min()
        .unwrap()
}
