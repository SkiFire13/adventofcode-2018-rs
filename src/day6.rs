#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input = Vec<Point>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, FromStr)]
#[from_str(regex = r"(?P<x>\d+), (?P<y>\d+)")]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn dist(&self, x: usize, y: usize) -> usize {
        return ((self.x as i16 - x as i16).abs() + (self.y as i16 - y as i16).abs()) as usize;
    }

    fn closest(points: &[Point], x: usize, y: usize) -> Option<Point> {
        let mut min_d = points[0].dist(x, y);
        let mut min_p = Some(points[0]);

        for p in points.iter().skip(1).copied() {
            let d = p.dist(x, y);
            if d == min_d {
                min_p = None;
            } else if d < min_d {
                min_d = d;
                min_p = Some(p);
            }
        }
        min_p
    }
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.parse().expect("Invalid input"))
        .collect()
}

pub fn part1(points: &Input) -> usize {
    let width = points.iter().max_by_key(|p| p.x).expect("Input is empty").x;
    let height = points.iter().max_by_key(|p| p.y).expect("Input is empty").y;

    let mut infinite_points = HashSet::with_capacity(points.len() / 4);
    let mut point_counts = HashMap::with_capacity(width * height);
    for (x, y) in (0..height).flat_map(|y| (0..width).map(move |x| (x, y))) {
        if let Some(point) = Point::closest(points, x, y) {
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                infinite_points.insert(point);
            }
            *point_counts.entry(point).or_insert(0) += 1;
        }
    }

    point_counts
        .into_iter()
        .filter(|(point, _)| !infinite_points.contains(point))
        .map(|(_, count)| count)
        .max()
        .expect("There's no point with finite area")
}

pub fn part2(points: &Input) -> usize {
    let width = points.iter().max_by_key(|p| p.x).expect("Input is empty").x;
    let height = points.iter().max_by_key(|p| p.y).expect("Input is empty").y;

    (0..width)
        .flat_map(|x| (0..height).map(move |y| (x, y)))
        .filter(|&(x, y)| points.iter().map(|p| p.dist(x, y)).sum::<usize>() < 10000)
        .count()
}
