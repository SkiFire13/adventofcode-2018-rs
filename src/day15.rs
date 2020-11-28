#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input = Stage;

#[derive(Clone, Copy, PartialEq, Eq)]
enum StageTile { Block, Empty }

#[derive(Clone, Copy, PartialEq, Eq)]
enum SoldierType { Elf, Goblin }
use SoldierType::*;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Soldier {
    soldier_type: SoldierType,
    health: usize,
    attack_power: usize,
    pos: Point,
}

type Point = (usize, usize);

fn neighbours(point: Point) -> impl Iterator<Item=Point> {
    [(-1, 0), (0, -1), (0, 1), (1, 0)]
        .iter()
        .map(move |&(dy, dx)| ((point.0 as isize + dy) as usize, (point.1 as isize + dx) as usize))
}

#[derive(Clone)]
pub struct Stage {
    map: Arc<Grid<StageTile>>,
    soldiers: Vec<Soldier>,
    soldiers_idx: BTreeMap<Point, usize>,
    goblin_count: usize,
    elf_count: usize,
}

impl Stage {
    fn try_direct_attack(&mut self, soldier: Soldier) -> bool {
        let target = neighbours(soldier.pos)
            .filter_map(|pos| self.soldiers_idx.get(&pos))
            .map(|&idx| self.soldiers[idx])
            .filter(|target| soldier.soldier_type != target.soldier_type && target.health > 0)
            .min_by_key(|soldier| (soldier.health, soldier.pos));
        if let Some(target) = target {
            let target = &mut self.soldiers[self.soldiers_idx[&target.pos]];
            target.health = target.health.saturating_sub(soldier.attack_power);
            if target.health == 0 {
                self.soldiers_idx.remove(&target.pos);
                match target.soldier_type {
                    Goblin => self.goblin_count -= 1,
                    Elf => self.elf_count -= 1,
                }
            }
            return true;
        }
        false
    }

    fn play(&mut self) -> usize {
        let mut rounds_played = 0;
        let mut idxs = Vec::with_capacity(self.soldiers_idx.len());
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        'outer: loop {
            idxs.clear();
            idxs.extend(self.soldiers_idx.values().copied());

            'inner: for idx in idxs.drain(..) {
                let mut soldier = self.soldiers[idx];

                if soldier.health == 0 {
                    continue 'inner;
                }

                if self.try_direct_attack(soldier) {
                    if self.goblin_count == 0 || self.elf_count == 0 { break 'outer; }
                    continue 'inner;
                }

                seen.clear();
                queue.clear();
                queue.extend(
                    neighbours(soldier.pos)
                    .filter(|&pos| self.map[(pos.1, pos.0)] == StageTile::Empty)
                    .map(|pos| (pos, pos))
                );
                let mut move_to = None;
                'move_to: while let Some((next_pos, move_to_candidate)) = queue.pop_front() {
                    if seen.insert(next_pos) {
                        if let Some(&idx) = self.soldiers_idx.get(&next_pos) {
                            if soldier.soldier_type != self.soldiers[idx].soldier_type {
                                move_to = Some(move_to_candidate);
                                break 'move_to;
                            }
                        } else {
                            queue.extend(
                                neighbours(next_pos)
                                .filter(|&pos| self.map[(pos.1, pos.0)] == StageTile::Empty)
                                .map(|pos| (pos, move_to_candidate))
                            );
                        }
                    }
                }
                if let Some(move_to) = move_to {
                    self.soldiers_idx.remove(&soldier.pos);
                    self.soldiers_idx.insert(move_to, idx);
                    self.soldiers[idx].pos = move_to;
                    soldier.pos = move_to;
                } else {
                    continue 'inner;
                }

                if self.try_direct_attack(soldier) {
                    if self.goblin_count == 0 || self.elf_count == 0 { break 'outer; }
                    continue 'inner;
                }
            }

            rounds_played += 1;
        }

        rounds_played
    }
}

pub fn input_generator(input: &str) -> Input {
    let mut goblin_count = 0;
    let mut elf_count = 0;
    let mut soldiers = Vec::new();
    let mut soldiers_idx = BTreeMap::new();
    let mut grid_vec = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                'G' => {
                    goblin_count += 1;
                    soldiers_idx.insert((y, x), soldiers.len());
                    soldiers.push(Soldier { soldier_type: Goblin, health: 200, attack_power: 3, pos: (y, x)});
                    StageTile::Empty
                },
                'E' => {
                    elf_count += 1;
                    soldiers_idx.insert((y, x), soldiers.len());
                    soldiers.push(Soldier { soldier_type: Elf, health: 200, attack_power: 3, pos: (y, x) });
                    StageTile::Empty
                }
                '.' => StageTile::Empty,
                '#' => StageTile::Block,
                _ => panic!("Invalid input"),
            };

            grid_vec.push(tile);
        }
    }

    let map = Arc::new(Grid {
        vec: grid_vec,
        width: input.lines().next().expect("Invalid input").len()
    });

    Stage { map, soldiers, soldiers_idx, goblin_count, elf_count }
}

pub fn part1(input: &Input) -> usize {
    let mut stage = input.clone();
    let rounds_played = stage.play();
    rounds_played * stage.soldiers.iter().map(|soldier| soldier.health).sum::<usize>()
}

pub fn part2(input: &Input) -> usize {
    let mut elf_attack_power = 4;
    let starting_elf_count = input.elf_count;
    
    loop {
        let mut stage = input.clone();
        stage.soldiers.iter_mut()
            .filter(|soldier| soldier.soldier_type == Elf)
            .for_each(|soldier| soldier.attack_power = elf_attack_power);
        let rounds_played = stage.play();

        if stage.elf_count == starting_elf_count {
            return rounds_played * stage.soldiers.iter().map(|soldier| soldier.health).sum::<usize>();
        }
        elf_attack_power += 1;
    }
}
