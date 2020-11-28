#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<Units>, Vec<Units>);

type Type = String;

#[derive(Clone)]
pub struct Units {
    count: usize,
    hit_points: usize,
    weaknesses: Vec<Type>,
    immunities: Vec<Type>,
    attack: usize,
    attack_type: Type,
    initiative: usize,
}

impl Units {
    fn attack(&self, other: &mut Self) {
        let units_lost = self.effective_power() * other.multiplier_from(self) / other.hit_points;
        other.count = other.count.saturating_sub(units_lost);
    }
    fn effective_power(&self) -> usize { self.count * self.attack }
    fn multiplier_from(&self, other: &Self) -> usize {
        if self.weaknesses.contains(&other.attack_type) { return 2; }
        if self.immunities.contains(&other.attack_type) { return 0; }
        1
    }
}

fn parse_units_list<'a>(regex: &Regex, input: impl Iterator<Item = &'a str>) -> Vec<Units> {
    input
        .map(|line| {
            let captures = regex.captures(line).expect(&format!("Invalid input {:?}", line));

            let weak_imm = captures.name("weak_imm").map(|m| m.as_str()).unwrap_or("");
            let mut weaknesses = Vec::new();
            let mut immunities = Vec::new();
            for group in weak_imm.split("; ") {
                if group.starts_with("weak to") {
                    weaknesses.extend(group[8..].split(", ").map(str::to_string));
                } else if group.starts_with("immune to") {
                    immunities.extend(group[10..].split(", ").map(str::to_string));
                } else if group != "" {
                    panic!("Invalid input")
                }
            }

            Units {
                count: captures["count"].parse().expect("Invalid input"),
                hit_points: captures["hit_points"].parse().expect("Invalid input"),
                weaknesses,
                immunities,
                attack: captures["attack"].parse().expect("Invalid input"),
                attack_type: captures["attack_type"].to_string(),
                initiative: captures["initiative"].parse().expect("Invalid input"),
            }
        })
        .collect()
}

pub fn input_generator(input: &str) -> Input {
    let mut split = input.splitn(2, "\n\n");

    let mut immune_system = split.next().expect("Invalid input").lines();
    assert_eq!(immune_system.next(), Some("Immune System:"));

    let mut infection = split.next().expect("Invalid input").lines();
    assert_eq!(infection.next(), Some("Infection:"));

    let regex = Regex::new(concat!(
        r"(?P<count>\d+) ",
        r"units each with ",
        r"(?P<hit_points>\d+) ",
        r"hit points ",
        r"(?:\((?P<weak_imm>[^\)]+)\) )?",
        r"with an attack that does ",
        r"(?P<attack>\d+) ",
        r"(?P<attack_type>\w+) ",
        r"damage at initiative ",
        r"(?P<initiative>\d+)",
    )).unwrap();

    (parse_units_list(&regex, immune_system), parse_units_list(&regex, infection))
}

fn get_targets(attackers: &[Units], defenders: &[Units]) -> Vec<Option<usize>> {
    let mut targets = vec![None; attackers.len()];
    for (idx, attacker) in attackers.iter().enumerate() {
        let target = defenders.iter()
            .enumerate()
            .filter(|(_, unit)| unit.multiplier_from(attacker) != 0)
            .filter(|&(idx, _)| !targets.contains(&Some(idx)))
            .max_by_key(|(_, unit)| (
                unit.multiplier_from(attacker),
                unit.effective_power(),
                unit.initiative,
            ))
            .map(|(idx, _)| idx);
        targets[idx] = target;
    }
    targets
}

fn next_turn(immune_system: &mut [Units], infection: &mut [Units]) -> bool {
    immune_system.sort_unstable_by_key(|unit| (Reverse(unit.effective_power()), unit.initiative));
    infection.sort_unstable_by_key(|unit| (Reverse(unit.effective_power()), unit.initiative));
    let immune_targets = get_targets(immune_system, infection);
    let infection_targets = get_targets(infection, immune_system);

    let mut attackers = (0.. immune_system.len() + infection.len()).collect::<Vec<_>>();
    attackers.sort_unstable_by_key(|&idx| {
        let unit = if idx < immune_system.len() {
            &immune_system[idx]
        } else {
            &infection[idx - immune_system.len()]
        };
        Reverse(unit.initiative)
    });
    let mut stuck = true;
    for idx in attackers {
        let (attacker, defender) = if idx < immune_system.len() {
            let defender_idx = if let Some(idx) = immune_targets[idx] { idx } else { continue; };
            (&immune_system[idx], &mut infection[defender_idx])
        } else {
            let idx = idx - immune_system.len();
            let defender_idx = if let Some(idx) = infection_targets[idx] { idx } else { continue; };
            (&infection[idx], &mut immune_system[defender_idx])
        };
        if attacker.count > 0 {
            attacker.attack(defender);
            stuck = false;
        }
    }
    stuck
}

pub fn part1(input: &Input) -> usize {
    let (mut immune_system, mut infection) = input.clone();
    while !immune_system.is_empty() && !infection.is_empty() {
        next_turn(&mut immune_system, &mut infection);
        immune_system.retain(|unit| unit.count != 0);
        infection.retain(|unit| unit.count != 0);
    }
    immune_system.iter()
        .chain(infection.iter())
        .map(|unit| unit.count)
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let mut boost = 0;
    let mut boost_step = 1000;

    loop {
        let mut stuck = false;
        let (mut immune_system, mut infection) = input.clone();
        immune_system.iter_mut().for_each(|unit| unit.attack += boost);
        while !stuck && !immune_system.is_empty() && !infection.is_empty() {
            stuck = next_turn(&mut immune_system, &mut infection);
            immune_system.retain(|unit| unit.count != 0);
            infection.retain(|unit| unit.count != 0);
        }

        if !infection.is_empty() {
            boost += boost_step;
        } else {
            if boost_step == 1 {
                return immune_system.iter().map(|unit| unit.count).sum();
            } else {
                boost -= boost_step;
                boost_step /= 10;
            }
        }
    }
}
