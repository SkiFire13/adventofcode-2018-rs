#[allow(unused_imports)]
use super::prelude::*;
type Input = (usize, (usize, usize));

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Tool { Torch, ClimbingGear, Nothing }

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct RegionCoordinate {
    x: usize,
    y: usize,
    tool: Tool
}

fn dist(rc1: RegionCoordinate, rc2: RegionCoordinate) -> usize {
    max(rc1.x, rc2.x) - min(rc1.x, rc2.x) +
    max(rc1.y, rc2.y) - min(rc1.y, rc2.y) +
    (rc1.tool != rc2.tool) as usize * 7
}

struct Cave {
    er_levels: Vec<Vec<usize>>,
    depth: usize,
    target: (usize, usize)
}

fn er_level_at(cave: &mut Cave, x: usize, y: usize) -> usize {
    if let Some(&er_level) = cave.er_levels.get(x).and_then(|col| col.get(y)) {
        return er_level;
    }

    if x != 0 { er_level_at(cave, x - 1, y); }
    if y != 0 { er_level_at(cave, x, y - 1); }

    let gindex = match (x, y) {
        (0, 0) => 0,
        (x, 0) => x * 16807,
        (0, y) => y * 48271,
        (x, y) if (x, y) == cave.target => 0,
        (x, y) => er_level_at(cave, x - 1, y) * er_level_at(cave, x, y - 1)
    };
    let er_level = (gindex + cave.depth) % 20183;

    if cave.er_levels.len() <= x {
        cave.er_levels.resize_with(x+1, Vec::new);
    }
    cave.er_levels[x].push(er_level);

    er_level
}

pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();

    let line1 = lines.next().expect("Invalid input");
    let depth = line1[7..].parse().expect("Invalid input");

    let line2 = lines.next().expect("Invalid input");
    let mut split = line2[8..].splitn(2, ',');
    let target_x = split.next().expect("Invalid input").parse().expect("Invalid input");
    let target_y = split.next().expect("Invalid input").parse().expect("Invalid input");

    (depth, (target_x, target_y))
}

pub fn part1(input: &Input) -> usize {
    let &(depth, target) = input;
    let mut cave = Cave { er_levels: Vec::new(), depth, target };
    er_level_at(&mut cave, target.0, target.1);
    cave.er_levels.into_iter().flatten().map(|r| r % 3).sum()
}

pub fn part2(input: &Input) -> usize {
    let &(depth, target) = input;
    let mut cave = Cave { er_levels: Vec::new(), depth, target };

    let mut seen = HashSet::with_capacity(target.0 * target.1);
    let mut queue = BinaryHeap::with_capacity(target.0 * target.1);

    let start = RegionCoordinate { x: 0, y: 0, tool: Tool::Torch };
    let target = RegionCoordinate { x: target.0, y: target.1, tool: Tool::Torch };

    queue.push(Reverse((0, 0, start)));

    while let Some(Reverse((_, time, point))) = queue.pop() {
        if seen.insert(point) {
            if point == target { return time; }
            let neighbours = [(1, 0), (-1, 0), (0, 1), (0, -1)].iter()
                .copied()
                .map(|(dx, dy)| (point.x as isize + dx, point.y as isize + dy))
                .filter(|&(x, y)| x >= 0 && y >= 0)
                .map(|(x, y)| (x as usize, y as usize))
                .map(|(x, y)| RegionCoordinate { x, y, ..point })
                .map(|point| (time + 1, point));
            let changetool = [Tool::Torch, Tool::ClimbingGear, Tool::Nothing].iter()
                .copied()
                .filter(|&tool| tool != point.tool)
                .map(|tool| RegionCoordinate { tool, ..point })
                .map(|point| (time + 7, point));
            let joined = neighbours.chain(changetool)
                .filter(|(_, point)| match (point.tool, er_level_at(&mut cave, point.x, point.y) % 3) {
                    (Tool::Nothing, 0) | (Tool::Torch, 1) | (Tool::ClimbingGear, 2) => false,
                    _ => true,
                })
                .map(|(time, point)| Reverse((time + dist(point, target), time, point)));
            queue.extend(joined);
        }
    }

    unreachable!();
}
