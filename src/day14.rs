#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input = usize;

const CHUNK_SIZE: usize = 20_000_000;

// Local helper for the scoreboard
struct RecipeScoreboard {
    elf1: usize,
    elf2: usize,
    recipes: Vec<u8>
}
impl RecipeScoreboard {
    fn new(max: usize) -> Self {
        let mut recipes = Vec::with_capacity(max);
        recipes.push(3);
        recipes.push(7);
        Self { elf1: 0, elf2: 1, recipes }
    }

    fn next(&mut self) {
        let sum: u8 = self.recipes[self.elf1] + self.recipes[self.elf2];
        if sum >= 10 {
            self.recipes.push(1);
            self.recipes.push(sum % 10);
        }
        else {
            self.recipes.push(sum);
        }
        self.elf1 = ( self.elf1 + 1 + self.recipes[self.elf1] as usize ) % self.recipes.len();
        self.elf2 = ( self.elf2 + 1 + self.recipes[self.elf2] as usize ) % self.recipes.len();
    }
}

pub fn input_generator(input: &str) -> Input  {
    input.parse().expect("Invalid input")
}

pub fn part1(&input: &Input) -> String {
    let mut scoreboard = RecipeScoreboard::new(input + 10 + CHUNK_SIZE + 2);
    let num_recipes = input + 10;
    while scoreboard.recipes.len() < num_recipes {
        scoreboard.next();
    }
    scoreboard.recipes.iter().skip(input).take(10).map(|i| i.to_string()).collect()
}

pub fn part2(&input: &Input) -> usize {
    let mut scoreboard = RecipeScoreboard::new(input + 10 + CHUNK_SIZE + 2);
    let to_find: Vec<u8> = input.to_string().chars().map(|d| d.to_digit(10).unwrap() as u8).collect();
    loop {
        let target_length = scoreboard.recipes.len() + CHUNK_SIZE;
        while scoreboard.recipes.len() < target_length {
            scoreboard.next();
        }
        if let Some(index) = scoreboard.recipes.windows(to_find.len()).position(|window| to_find == window) {
            return index;
        }
        scoreboard.recipes.reserve(CHUNK_SIZE + 1);
    }
}
