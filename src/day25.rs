#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Point4D>;

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, FromStr)]
#[display("{x},{y},{z},{w}")]
pub struct Point4D {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

fn dist(p1: Point4D, p2: Point4D) -> i32 {
    i32::abs(p1.x - p2.x) + 
    i32::abs(p1.y - p2.y) + 
    i32::abs(p1.z - p2.z) + 
    i32::abs(p1.w - p2.w)
}

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.parse().expect("Invalid input")).collect()
}

pub fn part1(input: &Input) -> usize {
    let mut input = input.clone();
    input.sort_unstable();

    let mut groups: Vec<Vec<Point4D>> = Vec::new();
    for point in input {
        let mut acc = Vec::new();
        let mut group_idx = 0;
        while group_idx < groups.len() {
            if groups[group_idx].iter().any(|&gpoint| dist(gpoint, point) <= 3) {
                let mut group = groups.swap_remove(group_idx);
                if acc.len() < group.len() { swap(&mut acc, &mut group); }
                acc.extend(group.into_iter());
            } else {
                group_idx += 1;
            }
        }
        acc.push(point);
        groups.push(acc);
    }

    groups.len()
}
