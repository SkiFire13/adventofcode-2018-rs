#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input = Vec<MovingPoint>;

#[derive(Clone, Copy, FromStr)]
#[from_str(regex = r"position=<\s*(?P<x>-?\d+),\s*(?P<y>-?\d+)> velocity=<\s*(?P<dx>-?\d+),\s*(?P<dy>-?\d+)>")]
pub struct MovingPoint {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.parse().expect("Invalid input"))
        .collect()
}

fn find_word(mut points: Vec<MovingPoint>) -> (String, usize) {
    let (u, v) = points
        .iter()
        .minmax_by_key(|p| p.dx + p.dy)
        .into_option()
        .unwrap();
    let (udx, udy, vdx, vdy) = (v.x - u.x, v.y - u.y, v.dx - u.dx, v.dy - u.dy);

    let mut i = ((-vdx * udx - vdy * udy) / (vdx * vdx + vdy * vdy)) as usize;

    for point in points.iter_mut() {
        point.x += point.dx * i as i32;
        point.y += point.dy * i as i32;
    }

    loop {
        let maxx = points.iter().map(|p| p.x).max().expect("Can't find maxx");
        let minx = points.iter().map(|p| p.x).min().expect("Can't find minx");
        let maxy = points.iter().map(|p| p.y).max().expect("Can't find maxy");
        let miny = points.iter().map(|p| p.y).min().expect("Can't find miny");

        let width = (maxx - minx + 1) as usize;
        let height = (maxy - miny + 1) as usize;

        if height == 10 {
            const CHARMAP: [u64; 26] = [
                0b001111111101000100001000010000100001000001000100000011111111, // A
                0b111111111110001000011000100001100010000110001000010111011110, // B
                0b011111111010000000011000000001100000000110000000010100000010, // C
                0,                                                              // D
                0b111111111110001000011000100001100010000110001000011000000001, // E
                0b111111111110001000001000100000100010000010001000001000000000, // F
                0b011111111010000000011000000001100001000110000100100100011111, // G
                0b111111111100001000000000100000000010000000001000001111111111, // H
                0,                                                              // I
                0b000000011000000000010000000001100000000111111111101000000000, // J
                0b111111111100001100000001001000001000010001000000101000000001, // K
                0b111111111100000000010000000001000000000100000000010000000001, // L
                0,                                                              // M
                0b111111111101100000000001100000000001100000000001101111111111, // N
                0,                                                              // O
                0b111111111110001000001000100000100010000010001000000111000000, // P
                0,                                                              // Q
                0b111111111110001000001000100000100011000010001011000111000011, // R
                0,                                                              // S
                0,                                                              // T
                0,                                                              // U
                0,                                                              // V
                0,                                                              // W
                0b110000001100110011000000110000000011000000110011001100000011, // X
                0,                                                              // Y
                0b100000011110000010011000010001100010000110010000011110000001, // Z
            ];

            let mut grid = vec![false; width * height];
            for point in points.iter() {
                grid[(point.y - miny + (point.x - minx) * height as i32) as usize] = true;
            }

            return (
                (0..width * height)
                    .step_by(10 * 6 + 10 * 2)
                    .map(|start| {
                        let encoded = grid[start..start + 60]
                            .iter()
                            .fold(0, |acc, &b| (acc << 1) + b as u64);
                        let pos = CHARMAP
                            .iter()
                            .position(|&e| encoded == e)
                            .expect("Unknown char");
                        (pos as u8 + b'A') as char
                    })
                    .collect(),
                i,
            );
        }

        for point in points.iter_mut() {
            point.x += point.dx;
            point.y += point.dy;
        }
        i += 1;
    }
}

pub fn part1(points: &Input) -> String {
    find_word(points.to_vec()).0
}

pub fn part2(points: &Input) -> usize {
    find_word(points.to_vec()).1
}
