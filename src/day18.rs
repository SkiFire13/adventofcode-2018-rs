#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input = Grid<Acre>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Acre { Open, Tree, Lumberyard }

fn neighbours(grid: &Grid<Acre>, x: usize, y: usize) -> impl Iterator<Item = Acre> + '_ {
    (-1..=1)
        .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
        .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
        .filter_map(move |(dx, dy)| Some(((x as isize + dx).try_into().ok()?, (y as isize + dy).try_into().ok()?)))
        .filter_map(move |(x, y)| grid.get((x, y)))
        .copied()
}

fn next_tick(grid: &Grid<Acre>, buffer: &mut Grid<Acre>) {
    for x in 0..grid.width {
        for y in 0..grid.height() {
            let mut open_count = 0;
            let mut tree_count = 0;
            let mut lumberyard_count = 0;
            for neighbour in neighbours(grid, x, y) {
                match neighbour {
                    Acre::Open => open_count += 1,
                    Acre::Tree => tree_count += 1,
                    Acre::Lumberyard => lumberyard_count += 1,
                }
            }
            buffer[(x, y)] = match grid[(x, y)] {
                Acre::Open if tree_count >= 3 => Acre::Tree,
                Acre::Open => Acre::Open,
                Acre::Tree if lumberyard_count >= 3 => Acre::Lumberyard,
                Acre::Tree => Acre::Tree,
                Acre::Lumberyard if lumberyard_count >= 1 && tree_count >= 1 => Acre::Lumberyard,
                Acre::Lumberyard => Acre::Open,
            }
        }
    }
}

fn resource_value(grid: &Grid<Acre>) -> usize {
    let mut tree_count = 0;
    let mut lumberyard_count = 0;
    for acre in grid.vec.iter() {
        match acre {
            Acre::Tree => tree_count += 1,
            Acre::Lumberyard => lumberyard_count += 1,
            _ => {}
        }
    }
    tree_count * lumberyard_count
}

pub fn input_generator(input: &str) -> Input {
    Grid {
        vec: input.lines().flat_map(|line| {
            line.chars().map(|c| match c {
                '.' => Acre::Open,
                '|' => Acre::Tree,
                '#' => Acre::Lumberyard,
                _ => panic!("Invalid input")
            })
        })
        .collect(),
        width: input.lines().next().expect("Invalid input").len()
    }
}

pub fn part1(input: &Input) -> usize {
    let mut grid = input.clone();
    let mut buffer = input.clone();
    for _ in 0..10 {
        next_tick(&grid, &mut buffer);
        swap(&mut grid, &mut buffer);
    }
    resource_value(&grid)
}

pub fn part2(input: &Input) -> usize {
    let mut grid = input.clone();
    let mut buffer = input.clone();
    let mut cache = HashMap::<Grid<Acre>, usize>::new();

    for cycle in 1..=1_000_000_000 {
        next_tick(&grid, &mut buffer);
        swap(&mut grid, &mut buffer);
        if let Some(&prev_cycle) = cache.get(&grid) {
            let final_idx = prev_cycle + (1_000_000_000 - cycle) % (cycle - prev_cycle);
            return resource_value(
                cache.iter()
                    .find(|&(_, &idx)| idx == final_idx)
                    .unwrap()
                    .0
            );
        }
        cache.insert(grid.clone(), cycle);
    }
    resource_value(&grid)
}
