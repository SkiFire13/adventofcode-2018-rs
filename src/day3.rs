#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input = (Vec<Claim>, Grid<usize>);

#[derive(FromStr, Debug, Copy, Clone)]
#[from_str(regex = r"(?m)#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)")]
pub struct Claim {
    id: u16,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

pub fn input_generator(input: &str) -> Input {
    let claims: Vec<Claim> = input
        .lines()
        .map(|line| line.parse::<Claim>().expect("Invalid input"))
        .collect();

    let (tot_width, tot_height) = claims.iter().fold((0, 0), |(old_w, old_h), claim| {
        (
            std::cmp::max(old_w, claim.x + claim.width),
            std::cmp::max(old_h, claim.y + claim.height),
        )
    });

    let mut claim_count = vec![0; tot_height * tot_width];

    for claim in claims.iter() {
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                claim_count[x + y * tot_width] += 1;
            }
        }
    }

    let grid = Grid {
        vec: claim_count,
        width: tot_width,
    };

    (claims, grid)
}

pub fn part1((_, claim_count): &Input) -> usize {
    claim_count.vec.iter().filter(|&&count| count > 1).count()
}

pub fn part2((claims, claim_count): &Input) -> u16 {
    'claim: for claim in claims.iter() {
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                if claim_count.vec[x + y * claim_count.width] > 1 {
                    continue 'claim;
                }
            }
        }
        return claim.id;
    }

    panic!("There's no claim that doesn't overlap")
}
