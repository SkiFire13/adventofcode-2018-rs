#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<NanoBot>;

#[derive(Clone, Copy, FromStr)]
#[display("pos=<{x},{y},{z}>, r={r}")]
pub struct NanoBot {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

#[ord_by_key(|p| (
    p.count,
    p.size,
    Reverse(p.distance_origin()),
))]
#[derive(Clone, Copy)]
struct Region {
    count: usize,
    x: i64,
    y: i64,
    z: i64,
    size: i64,
}

fn clamp(a: i64, min: i64, max: i64) -> i64 { if a < min { min } else if a > max { max } else { a } }

impl Region {
    fn distance_origin(&self) -> i64 {
        let &Region { x, y, z, size, ..} = self;
        min(i64::abs(x), i64::abs(x + size)) +
        min(i64::abs(y), i64::abs(y + size)) +
        min(i64::abs(z), i64::abs(z + size))
    }
    fn new(nanobots: &[NanoBot], size: i64, x: i64, y: i64, z: i64) -> Self {
        let count = nanobots.par_iter()
            .filter(|nb| {
                let nx = clamp(nb.x, x, x + size - 1);
                let ny = clamp(nb.y, y, y + size - 1);
                let nz = clamp(nb.z, z, z + size - 1);
                i64::abs(nx - nb.x) + i64::abs(ny - nb.y) + i64::abs(nz - nb.z) <= nb.r
            })
            .count();
        Self { count, x, y, z, size }
    }
    fn subdivide(self, nanobots: &[NanoBot]) -> [Self; 8] {
        let Self { x, y, z, size, ..} = self;
        let size = size >> 1;
        [
            Self::new(nanobots, size, x,        y,        z       ),
            Self::new(nanobots, size, x + size, y,        z       ),
            Self::new(nanobots, size, x,        y + size, z       ),
            Self::new(nanobots, size, x,        y,        z + size),
            Self::new(nanobots, size, x + size, y + size, z       ),
            Self::new(nanobots, size, x,        y + size, z + size),
            Self::new(nanobots, size, x + size, y,        z + size),
            Self::new(nanobots, size, x + size, y + size, z + size),
        ]
    }
}

pub fn input_generator(input: &str) -> Input {
    input.lines()
        .map(|line| line.parse().expect("Invalid input"))
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let nanobots = input;
    let target = nanobots.iter().max_by_key(|nanobot| nanobot.r).unwrap();
    nanobots.iter()
        .filter(|nanobot| {
            (nanobot.x - target.x).abs()
              + (nanobot.y - target.y).abs()
              + (nanobot.z - target.z).abs() 
            <= target.r
        })
        .count()
}

pub fn part2(input: &Input) -> i64 {
    let nanobots = input;

    let mut size = i64::MIN;
    for nanobot in nanobots.iter() {
        size = max(size, nanobot.x.abs());
        size = max(size, nanobot.y.abs());
        size = max(size, nanobot.z.abs());
    }
    let size = 1 << (64 - size.leading_zeros());

    let mut queue = BinaryHeap::new();
    queue.push(Region::new(nanobots, size * 2, -size, -size, -size));
    let mut best_leaf = Region::new(&[], 0, 0, 0, 0);

    while let Some(region) = queue.pop() {
        if best_leaf.count > region.count {
            continue;
        }

        if region.size == 1 {
            best_leaf = max(best_leaf,region);
        } else {
            let children = region.subdivide(nanobots);
            for &child in children.iter() {
                queue.push(child);
            }
        }
    }

    let Region { x, y, z, ..} = best_leaf;
    x.abs() + y.abs() + z.abs()
}
