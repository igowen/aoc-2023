use crate::Dir;
use aoc_runner_derive::{aoc, aoc_generator};
use array2d::Array2D;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Location {
    GardenPlot,
    Rock,
}

#[derive(Clone, Debug)]
struct Input {
    map: Array2D<Location>,
    start_location: (i64, i64),
}

#[aoc_generator(day21)]
fn generate(input: &str) -> Input {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let start_location = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                'S' => Some((y as i64, x as i64)),
                _ => None,
            })
        })
        .flatten()
        .next()
        .unwrap();
    let map = Array2D::from_iter_row_major(
        input
            .lines()
            .map(|line| {
                line.chars().map(|c| match c {
                    '.' | 'S' => Location::GardenPlot,
                    '#' => Location::Rock,
                    other => panic!("invalid char {other}"),
                })
            })
            .flatten(),
        height,
        width,
    )
    .unwrap();
    Input {
        map,
        start_location,
    }
}

#[aoc(day21, part1)]
fn solve_part1(input: &Input) -> i64 {
    let mut current = HashSet::new();
    let mut next = HashSet::new();
    current.insert(input.start_location);
    for _ in 0..64 {
        for pos in current.drain() {
            for (y, x) in Dir::neighbors(pos, &input.map) {
                if input.map[(y as usize, x as usize)] == Location::GardenPlot {
                    next.insert((y, x));
                }
            }
        }
        std::mem::swap(&mut current, &mut next);
    }
    current.len() as i64
}
