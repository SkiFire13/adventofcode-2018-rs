#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input = (Vec<Pot>, Vec<Rule>);

#[derive(PartialEq, Eq, Clone, Copy, Debug, Display, FromStr)]
pub enum Pot {
    #[display("#")]
    Plant,
    #[display(".")]
    Nothing
}

#[derive(Clone, Copy)]
pub struct Rule {
    start: [Pot ; 5],
    end: Pot
}

#[derive(Clone)]
pub struct PotsState {
    state: VecDeque<Pot>,
    offset: isize,
}

impl PotsState {
    fn normalize(&mut self) {
        while let Some(Pot::Nothing) = self.state.front() {
            self.state.pop_front();
            self.offset -= 1;
        }
        while let Some(Pot::Nothing) = self.state.back() {
            self.state.pop_back();
        }
        for _ in 0..5 {
            self.state.push_front(Pot::Nothing);
            self.state.push_back(Pot::Nothing);
            self.offset += 1;
        }
    }

    fn next(&mut self, rules: &[Rule]) {
        for _ in 0..self.state.len()-4 {
            let next = rules.iter()
                .find(|rule| {
                    rule.start.iter().zip(self.state.iter().take(5)).all(|(&p1, &p2)| p1 == p2)
                })
                .map(|rule| rule.end)
                .unwrap_or(Pot::Nothing);

            self.state.pop_front();
            self.state.push_back(next);
        }
    
        self.state.pop_front();
        self.state.pop_front();
        self.state.pop_front();
        self.state.pop_front();
        self.offset -= 2;

        self.normalize();
    }

    fn sum(&self) -> isize {
        self.state.iter()
            .enumerate()
            .filter(|&(_, &p)| p == Pot::Plant)
            .map(|(i, _)| i as isize - self.offset)
            .sum()
    }
}

pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();

    let pots = lines
        .next()
        .expect("Invalid input")["initial state: ".len()..]
        .matches(|_| true)
        .map(|c| c.parse().expect("Invalid input"))
        .collect();

    let rules = lines
        .skip(1)
        .map(|line| {
            let start = line[..5]
                .matches(|_| true)
                .map(|c| c.parse().expect("Invalid input"))
                .collect::<ArrayVec<_>>()
                .into_inner()
                .expect("Invalid input");
            let end = line[9..].parse().expect("Invalid input");
            Rule { start, end }
        }).collect();

    (pots, rules)
}

pub fn part1((pots, rules): &Input) -> isize {
    let mut pots_state = PotsState {
        state: pots.iter().copied().collect(),
        offset: 0,
    };
    pots_state.normalize();

    for _ in 0..20 {
        pots_state.next(rules)
    }

    pots_state.sum()
}

pub fn part2((pots, rules): &Input) -> isize {
    let mut pots_state = PotsState {
        state: pots.iter().copied().collect(),
        offset: 0,
    };
    pots_state.normalize();
    let mut old_state = pots_state.clone();

    for i in 0..50_000_000_000isize {
        pots_state.next(rules);
        if pots_state.state == old_state.state {
            pots_state.offset += (pots_state.offset - old_state.offset) * (50_000_000_000 - (i + 1));
            return pots_state.sum();
        }
        old_state.offset = pots_state.offset;
        old_state.state.clear();
        old_state.state.extend(pots_state.state.iter());
        old_state.clone_from(&pots_state);
    }

    pots_state.sum()
}
