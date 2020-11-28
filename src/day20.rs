#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input = Vec<Instruction>;

type Point = (isize, isize);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    North,
    South,
    West,
    East,
    GroupStart,
    GroupEnd,
    Branch,
}

pub fn input_generator(input: &str) -> Input {
    let mut chars = input.chars();
    assert_eq!(chars.next(), Some('^'));
    assert_eq!(chars.next_back(), Some('$'));
    chars
        .map(|c| match c {
            'N' => Instruction::North,
            'S' => Instruction::South,
            'W' => Instruction::West,
            'E' => Instruction::East,
            '(' => Instruction::GroupStart,
            ')' => Instruction::GroupEnd,
            '|' => Instruction::Branch,
            _ => panic!("Invalid input"),
        })
        .collect()
}

fn create_map(instructions: &[Instruction]) -> HashMap<Point, ArrayVec<[Point; 4]>> {
    let mut current = (0, 0);
    let mut stack = vec![(0, 0)];
    let mut map = HashMap::new();
    for &instruction in instructions {
        let next = match instruction {
            Instruction::North => (current.0, current.1 + 1),
            Instruction::South => (current.0, current.1 - 1),
            Instruction::West => (current.0 - 1, current.1),
            Instruction::East => (current.0 + 1, current.1),
            Instruction::GroupStart => {
                stack.push(current);
                continue;
            },
            Instruction::GroupEnd => {
                stack.pop();
                continue;
            },
            Instruction::Branch => {
                current = stack.last().copied().expect("Invalid number of group instructions");
                continue;
            }
        };
        let current_neighbours = map.entry(current).or_insert_with(ArrayVec::new);
        if !current_neighbours.contains(&next) {
            current_neighbours.push(next);
        }
        let next_neighbours = map.entry(next).or_insert_with(ArrayVec::new);
        if !next_neighbours.contains(&current) {
            next_neighbours.push(current);
        }
        current = next;
    }
    map
}

fn distances(map: &HashMap<Point, ArrayVec<[Point; 4]>>) -> impl Iterator<Item = (Point, usize)> + '_ {
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    queue.push(Reverse((0, (0, 0))));
    iter::from_fn(move || loop {
        let Reverse((distance, point)) = queue.pop()?;
        if seen.insert(point) {
            queue.extend(map[&point].iter().copied().map(|next| Reverse((distance + 1, next))));
            return Some((point, distance));
        }
    })
}

pub fn part1(input: &Input) -> usize {
    let instructions = input;
    let map = create_map(instructions);
    let distances = distances(&map);
    distances.last().unwrap().1
}

pub fn part2(input: &Input) -> usize {
    let instructions = input;
    let map = create_map(instructions);
    let distances = distances(&map);
    distances.filter(|&(_, dist)| dist >= 1000).count()
}
