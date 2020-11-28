#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input = (Grid<TrackDirection>, Vec<Cart>);

#[derive(Clone, Copy, Display, Debug)]
#[display("{0},{1}")]
pub struct Tuple2(usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TrackDirection {
    XAxis,
    YAxis,
    Diag13,
    Diag24,
    Crossroad,
    Empty,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Direction(isize, isize);

#[derive(Copy, Clone)]
pub struct Cart {
    id: usize,
    x: usize,
    y: usize,
    direction: Direction,
    crossroad_count: u32,
    crashed: bool
}

impl Cart {
    fn new(id: usize, x: usize, y: usize, direction: Direction) -> Self {
        Self { id, x, y, direction, crossroad_count: 0, crashed: false }
    }
    fn move_next(&mut self, grid: &Grid<TrackDirection>) {
        self.x = (self.x as isize + self.direction.0) as usize;
        self.y = (self.y as isize + self.direction.1) as usize;
        
        let dir = self.direction;
        self.direction = match grid[(self.x, self.y)] {
            TrackDirection::XAxis | TrackDirection::YAxis => dir,
            TrackDirection::Diag13 => Direction(-dir.1, -dir.0),
            TrackDirection::Diag24 => Direction(dir.1, dir.0),
            TrackDirection::Crossroad => {
                self.crossroad_count += 1;
                match self.crossroad_count % 3 {
                    0 => Direction(-dir.1, dir.0),
                    1 => Direction(dir.1, -dir.0),
                    _ => dir
                }
            }
            TrackDirection::Empty => panic!("A cart is off the track!"),
        };
    }
}

pub fn input_generator(input: &str) -> Input {
    let grid = Grid {
        vec: input.lines().flat_map(|line| {
            line.chars().map(|c| match c {
                '-' | '<' | '>' => TrackDirection::XAxis,
                '|' | 'v' | '^' => TrackDirection::YAxis,
                '/' => TrackDirection::Diag13,
                '\\' => TrackDirection::Diag24,
                '+' => TrackDirection::Crossroad,
                ' ' => TrackDirection::Empty,
                _ => panic!("Invalid input"),
            })
        }).collect(),
        width: input.lines().nth(0).expect("Invalid input").len(),
    };

    let width = grid.width;
    let carts = input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter_map(move |(x, c)| match c {
            'v' => Some(Cart::new(x + width * y, x, y, Direction(0, 1))),
            '^' => Some(Cart::new(x + width * y, x, y, Direction(0, -1))),
            '<' => Some(Cart::new(x + width * y, x, y, Direction(-1, 0))),
            '>' => Some(Cart::new(x + width * y, x, y, Direction(1, 0))),
            _ => None
        })
    })
    .collect();

    (grid, carts)
}

fn next_tick(grid: &Grid<TrackDirection>, carts: &mut Vec<Cart>) {
    for i in 0..carts.len() {
        let cart = &mut carts[i];

        if !cart.crashed {
            cart.move_next(grid);
        }

        let (id, x, y) = (cart.id, cart.x, cart.y);
        let mut crashed = false;
        for c in carts.iter_mut().filter(|c| c.x == x && c.y == y && c.id != id) {
            c.crashed = true;
            crashed = true;
        }
        carts[i].crashed = crashed;
    }
}

pub fn part1((grid, carts): &Input) -> Tuple2 {
    let mut carts = carts.clone();

    while carts.iter().all(|c| !c.crashed) {
        next_tick(grid, &mut carts);
    }
    
    carts.iter().filter(|c| c.crashed).map(|c| Tuple2(c.x, c.y)).next().unwrap()
}

pub fn part2((grid, carts): &Input) -> Tuple2 {
    let mut carts = carts.clone();

    while carts.len() > 1 {
        next_tick(grid, &mut carts);
        carts.retain(|c| !c.crashed);
        carts.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
    }

    carts.iter().filter(|c| !c.crashed).map(|c| Tuple2(c.x, c.y)).next().unwrap()
}
