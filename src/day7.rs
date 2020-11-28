#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input = BTreeMap<char, Vec<char>>;

pub fn input_generator(input: &str) -> Input {
    let mut steps = BTreeMap::new();

    for line in input.lines() {
        let c1 = line.chars().nth(5).expect("Invalid input");
        let c2 = line.chars().nth(36).expect("Invalid input");

        steps.entry(c1).or_insert(vec![]);
        steps.entry(c2).or_insert(vec![]).push(c1);
    }

    steps
}

pub fn part1(steps: &Input) -> String {
    let mut steps = steps.clone();
    let mut output = String::new();

    while steps.len() != 0 {
        let first = *steps.iter().find(|(_, v)| v.len() == 0).expect("Cyclic dependencies!").0;
        output.push(first);
        steps.remove(&first);
        for deps in steps.values_mut() {
            if let Some(p) = deps.iter().position(|&r| r == first) {
                deps.swap_remove(p);
            }
        }
    }

    output
}

pub fn part2(steps: &Input) -> u32 {
    let mut steps = steps.clone();
    let mut workers: [Option<(char, u32)>; 5] = [None ; 5];
    let mut time = 0;

    loop {
        for option in workers.iter_mut() {
            if let Some((c, time_left)) = option {
                *time_left -= 1;
                if *time_left == 0 {
                    for (_, deps) in steps.iter_mut() {
                        if let Some(p) = deps.iter().position(|&r| r == *c) {
                            deps.swap_remove(p);
                        }
                    }
                    *option = None;
                }
            }
        }

        for option in workers.iter_mut() {
            if *option == None {
                if let Some((&c, _)) = steps.iter().find(|(_, v)| v.len() == 0) {
                    *option = Some((c, 60 + (c as u32 - 64)));
                    steps.remove(&c);
                }
            }
        }

        if workers.iter().all(Option::is_none) {
            break;
        }

        time += 1;
    }

    time
}
