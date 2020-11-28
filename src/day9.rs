#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input = (u32, u32);

pub fn input_generator(input: &str) -> Input {
    let mut split = input.split_whitespace();
    let players = split
        .next()
        .expect("Invalid input")
        .parse()
        .expect("Invalid input");
    let last_worth = split
        .skip(5)
        .next()
        .expect("Invalid input")
        .parse()
        .expect("Invalid input");

    (players, last_worth)
}

fn winning_score(players: u32, last_worth: u32) -> u32 {
    let mut scores = vec![0u32; players as usize];
    let mut table = VecDeque::new();
    table.push_back(0);

    for i in 1..(last_worth + 1) {
        if i % 23 == 0 {
            table.rotate_right(7);
            scores[(i % players) as usize] += i + table.pop_back().unwrap();
            table.rotate_left(1);
        } else {
            table.rotate_left(1);
            table.push_back(i);
        }
    }

    scores.iter().copied().max().unwrap()
}

pub fn part1(&(players, last_worth): &Input) -> u32 {
    winning_score(players, last_worth)
}

pub fn part2(&(players, last_worth): &Input) -> u32 {
    winning_score(players, last_worth * 100)
}
