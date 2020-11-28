#![allow(unused_imports)]
use super::prelude::*;
type Input = Grid<Cell>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell { Flowing, Still, Empty, Wall }

fn is_stagn(grid: &Grid<Cell>, (x, y): (usize, usize)) -> bool {
    let mut tx = x + 1;
    loop {
        let cell = grid.get((tx, y)).copied();
        if cell == Some(Cell::Wall) { break; }
        if cell == None { return false; }
        let under = grid[(tx, y+1)];
        if under == Cell::Empty || under == Cell::Flowing { return false; }
        tx += 1;
    }
    let mut tx = x - 1;
    loop {
        let cell = grid.get((tx, y)).copied();
        if cell == Some(Cell::Wall) { break; }
        if cell == None { return false; }
        let under = grid[(tx, y+1)];
        if under == Cell::Empty || under == Cell::Flowing { return false; }
        tx -= 1;
    }
    true
}

fn fill_side(grid: &mut Grid<Cell>, fill: Cell, x: usize, y: usize, next: fn(usize) -> usize) -> usize {
    let mut x = next(x);
    while grid[(x, y)] != Cell::Wall {
        grid[(x, y)] = fill;
        let under = grid[(x, y+1)];
        if under == Cell::Empty || under == Cell::Flowing {
            grid[(x, y+1)] = Cell::Flowing;
            break;
        }
        x = next(x);
    }
    x
}

fn flow(grid: &mut Grid<Cell>) {
    let mut x = 0;
    let mut y = 0;

    'outer: while let Some(&cell) = grid.get((x, y)) {
        if cell == Cell::Flowing {
            match grid.get((x, y+1)) {
                Some(Cell::Still) | Some(Cell::Wall) => {
                    let fill = if is_stagn(&grid, (x, y)) { Cell::Still } else { Cell::Flowing };
                    grid[(x, y)] = fill;
                    let leftmost = fill_side(grid, fill, x, y, |x| x-1);
                    let rightmost = fill_side(grid, fill, x, y, |x| x+1);
                    x = if fill == Cell::Still { leftmost } else { rightmost };
                    if fill == Cell::Still { y -= 1; }
                    continue 'outer;
                },
                Some(Cell::Empty) => grid[(x, y+1)] = Cell::Flowing,
                _ => {},
            }
        }
        x += 1;
        y += x / grid.width;
        x %= grid.width;
    }
}

pub fn input_generator(input: &str) -> Input {
    #[derive(FromStr)]
    #[from_str(regex = "(?P<direction>[xy])=(?P<fixed>[0-9]+), [xy]=(?P<start>[0-9]+)..(?P<end>[0-9]+)")]
    struct Rectangle {
        direction: char,
        fixed: usize,
        start: usize,
        end: usize,
    }

    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut min_y = usize::MAX;
    let mut max_y = 0;
    let rectangles = input.lines()
        .map(|line| line.parse::<Rectangle>().expect("Invalid input"))
        .map(|r| match r.direction {
            'x' => (r.fixed, r.fixed, r.start, r.end),
            'y' => (r.start, r.end, r.fixed, r.fixed),
            _ => unreachable!(),
        })
        .inspect(|r| {
            min_x = std::cmp::min(min_x, r.0);
            max_x = std::cmp::max(max_x, r.1);
            min_y = std::cmp::min(min_y, r.2);
            max_y = std::cmp::max(max_y, r.3);
        })
        .collect::<Vec<_>>();

    min_x -= 1;
    max_x += 1;

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut grid = Grid { vec: vec![Cell::Empty; width * height], width };

    rectangles.into_iter()
        .flat_map(|(xs, xe, ys, ye)| (xs..=xe).cartesian_product(ys..=ye))
        .for_each(|(x, y)| grid[(x - min_x, y - min_y)] = Cell::Wall);

    assert_eq!(grid[(500 - min_x, 0)], Cell::Empty);
    grid[(500 - min_x, 0)] = Cell::Flowing;

    grid
}

pub fn part1(input: &Input) -> usize {
    let mut grid = input.clone();
    flow(&mut grid);
    grid.vec.iter()
        .filter(|&&cell| cell == Cell::Still || cell == Cell::Flowing)
        .count()
}

pub fn part2(input: &Input) -> usize {
    let mut grid = input.clone();
    flow(&mut grid);
    grid.vec.iter()
        .filter(|&&cell| cell == Cell::Still)
        .count()
}
