#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input = HashMap<u16, [u16; 60]>;
#[derive(Clone, Copy)]
enum LogEvent {
    GuardChange(u16),
    GuardWakesUp(u16),
    GuardSleeps(u16),
}

pub fn input_generator(input: &str) -> Input {
    let logs = input
        .lines()
        .sorted()
        .map(|line| {
            if line.contains("Guard") {
                return LogEvent::GuardChange(
                    line[26..line.find(" b").expect("Invalid input")]
                        .parse()
                        .expect("Invalid input"),
                );
            }
            let minute: u16 = line[15..17].parse().expect("Invalid input");
            if line.contains("falls") {
                return LogEvent::GuardSleeps(minute);
            }
            if line.contains("wakes") {
                return LogEvent::GuardWakesUp(minute);
            }
            panic!("Invalid input")
        })
        .collect::<Vec<_>>();

    let mut current_id = match logs[0] {
        LogEvent::GuardChange(id) => id,
        _ => panic!("First event is not GuardChange"),
    };
    let mut asleep_minute = match logs[1] {
        LogEvent::GuardSleeps(minute) => minute,
        _ => panic!("Second event is not GuardSleeps"),
    };

    let mut guard_map = HashMap::new();
    for &log in &logs[2..] {
        match log {
            LogEvent::GuardChange(id) => current_id = id,
            LogEvent::GuardSleeps(minute) => asleep_minute = minute,
            LogEvent::GuardWakesUp(minute) => {
                for i in asleep_minute..minute {
                    guard_map.entry(current_id).or_insert([0u16; 60])[i as usize] += 1;
                }
            }
        }
    }

    guard_map
}

pub fn part1(guard_map: &Input) -> u32 {
    let (&id, _) = guard_map
        .iter()
        .max_by_key(|(_, h)| h.iter().sum::<u16>())
        .expect("No max guard found");
    let (minute, _) = guard_map[&id]
        .iter()
        .enumerate()
        .max_by_key(|&(_, &m)| m)
        .expect("No max hour found");

    minute as u32 * id as u32
}

pub fn part2(guard_map: &Input) -> u32 {
    let (id, (minute, _)) = guard_map
        .iter()
        .map(|(&id, minutes)| {
            (
                id,
                minutes.iter().enumerate().max_by_key(|(_, &m)| m).unwrap(),
            )
        })
        .max_by_key(|(_, (_, &m))| m)
        .expect("No max hour found");
    id as u32 * minute as u32
}
