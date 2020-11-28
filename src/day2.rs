#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input<'a> = &'a str;

pub fn input_generator(input: &str) -> Input { input }

pub fn part1(input: &Input) -> usize {
    let mut count_two = 0;
    let mut count_three = 0;

    for line in input.lines() {
        let mut counter = HashMap::new();
        for c in line.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
        if counter.values().any(|c| *c == 2) {
            count_two += 1;
        }
        if counter.values().any(|c| *c == 3) {
            count_three += 1;
        }
    }

    count_two * count_three
}

pub fn part2(input: &Input) -> String {
    for (i, line) in input.lines().enumerate() {
        for other in input.lines().skip(i + 1) {
            let similar_chars = line
                .chars()
                .zip(other.chars())
                .filter(|(a, b)| a == b)
                .map(|(c, _)| c);
            if similar_chars.clone().count() == line.len() - 1 {
                return similar_chars.collect();
            }
        }
    }

    panic!("No lines are similar");
}
