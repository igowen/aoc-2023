use std::{collections::BTreeSet, fmt::Display};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Tile {
    Empty,
    Galaxy,
}

#[derive(Clone, Debug)]
struct Input {
    map: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    empty_rows: BTreeSet<usize>,
    empty_columns: BTreeSet<usize>,
    galaxies: Vec<(usize, usize)>,
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "size = ({},{})", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                match self.map[y][x] {
                    Tile::Empty => write!(f, ".")?,
                    Tile::Galaxy => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc_generator(day11)]
fn generate(input: &str) -> Input {
    let mut galaxies = vec![];
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Tile::Empty,
                    '#' => {
                        galaxies.push((x, y));
                        Tile::Galaxy
                    }
                    _ => panic!("invalid char {:?}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = map[0].len();
    let height = map.len();
    let mut empty_columns = (0..width).collect::<BTreeSet<_>>();
    let mut empty_rows = (0..height).collect::<BTreeSet<_>>();
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == Tile::Galaxy {
                empty_columns.remove(&x);
                empty_rows.remove(&y);
            }
        }
    }

    Input {
        width,
        height,
        map,
        empty_rows,
        empty_columns,
        galaxies,
    }
}

#[aoc(day11, part1)]
fn solve_part1(input: &Input) -> i64 {
    input
        .galaxies
        .iter()
        .map(|(x0, y0)| {
            let (x0, y0) = (*x0 as isize, *y0 as isize);
            input
                .galaxies
                .iter()
                .map(|(x1, y1)| {
                    let (x1, y1) = (*x1 as isize, *y1 as isize);
                    let mut distance = (x1 - x0).abs() + (y1 - y0).abs();
                    distance += input
                        .empty_columns
                        .iter()
                        .filter(|x| **x as isize > x0.min(x1) && (**x as isize) < x0.max(x1))
                        .count() as isize;
                    distance += input
                        .empty_rows
                        .iter()
                        .filter(|y| **y as isize > y0.min(y1) && (**y as isize) < y0.max(y1))
                        .count() as isize;
                    distance
                })
                .sum::<isize>()
        })
        .sum::<isize>() as i64
        / 2
}

#[aoc(day11, part2)]
fn solve_part2(input: &Input) -> i64 {
    input
        .galaxies
        .iter()
        .map(|(x0, y0)| {
            let (x0, y0) = (*x0 as isize, *y0 as isize);
            input
                .galaxies
                .iter()
                .map(|(x1, y1)| {
                    let (x1, y1) = (*x1 as isize, *y1 as isize);
                    let mut distance = (x1 - x0).abs() + (y1 - y0).abs();
                    distance += input
                        .empty_columns
                        .iter()
                        .filter(|x| **x as isize > x0.min(x1) && (**x as isize) < x0.max(x1))
                        .count() as isize
                        * 999999;
                    distance += input
                        .empty_rows
                        .iter()
                        .filter(|y| **y as isize > y0.min(y1) && (**y as isize) < y0.max(y1))
                        .count() as isize
                        * 999999;
                    distance
                })
                .sum::<isize>()
        })
        .sum::<isize>() as i64
        / 2
}
