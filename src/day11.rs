#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input = Vec<i32>;

#[derive(Clone, Copy, Display, Debug)]
#[display("{0},{1}")]
pub struct Tuple2(isize, isize);
#[derive(Clone, Copy, Display, Debug)]
#[display("{0},{1},{2}")]
pub struct Tuple3(isize, isize, isize);

fn get2d(levels: &[i32], x: isize, y: isize) -> i32 {
    if x < 0 || x >= 300 || y < 0 || y >= 300 {
        return 0;
    }
    levels[(x + 300 * y) as usize]
}

fn get_level(levels: &[i32], x: isize, y: isize, size: isize) -> i32 {
    get2d(levels, x - 1, y - 1) + get2d(levels, x - 1 + size, y - 1 + size)
        - get2d(levels, x - 1 + size, y - 1)
        - get2d(levels, x - 1, y - 1 + size)
}

pub fn input_generator(input: &str) -> Input {
    let serial = input.parse::<isize>().expect("Invalid input");

    let mut levels = vec![0; 300 * 300];
    for y in 0..300 {
        for x in 0..300 {
            let base = (((x + 10) * y + serial) * (x + 10) / 100 % 10 - 5) as i32;
            levels[(x + 300 * y) as usize] = base - get_level(&levels, x, y, 1);
        }
    }

    levels
}

pub fn part1(levels: &Input) -> Tuple2 {
    (0..298)
        .flat_map(|x| (0..298).map(move |y| Tuple2(x, y)))
        .max_by_key(|&Tuple2(x, y)| get_level(levels, x, y, 3))
        .expect("The input was empty")
}

pub fn part2(levels: &Input) -> Tuple3 {
    (0..298)
        .flat_map(|x| (0..298).map(move |y| (x, y)))
        .flat_map(|(x, y)| (1..min(300 - x, 300 - y)).map(move |size| Tuple3(x, y, size)))
        .max_by_key(|&Tuple3(x, y, size)| get_level(levels, x, y, size))
        .expect("The input was empty")
}
