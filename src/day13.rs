use std::fmt::Debug;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
enum Tile {
    Ash,
    Rock,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ash => write!(f, "."),
            Tile::Rock => write!(f, "#"),
        }
    }
}

#[derive(Clone, Debug)]
struct Map {
    rows: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

#[aoc_generator(day13)]
fn generate(input: &str) -> Vec<Map> {
    input
        .lines()
        .collect::<Vec<_>>()
        .split(|s| s.trim().is_empty())
        .map(|v| {
            let rows = v
                .iter()
                .map(|s| {
                    s.chars()
                        .map(|c| match c {
                            '.' => Tile::Ash,
                            '#' => Tile::Rock,
                            _ => panic!("invalid char {:?}", c),
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            Map {
                width: rows[0].len(),
                height: rows.len(),
                rows,
            }
        })
        .collect()
}

#[aoc(day13, part1)]
fn solve_part1(input: &Vec<Map>) -> i64 {
    input
        .iter()
        .enumerate()
        .map(|(n, m)| {
            let mut columnwise = vec![vec![]; m.width];
            for x in 0..m.width {
                for row in &m.rows {
                    columnwise[x].push(row[x]);
                }
            }
            for i in 1..m.height {
                if m.rows[0..i]
                    .iter()
                    .rev()
                    .zip(m.rows[i..].iter())
                    .filter(|(a, b)| a != b)
                    .count()
                    == 0
                {
                    return i as i64 * 100;
                }
            }
            for i in 1..m.width {
                if columnwise[0..i]
                    .iter()
                    .rev()
                    .zip(columnwise[i..].iter())
                    .filter(|(a, b)| a != b)
                    .count()
                    == 0
                {
                    return i as i64;
                }
            }
            panic!("{n}: no valid reflection");
        })
        .sum()
}

#[aoc(day13, part2)]
fn solve_part2(input: &Vec<Map>) -> i64 {
    input
        .iter()
        .enumerate()
        .map(|(n, m)| {
            let mut columnwise = vec![vec![]; m.width];
            for x in 0..m.width {
                for row in &m.rows {
                    columnwise[x].push(row[x]);
                }
            }
            for i in 1..m.height {
                if m.rows[0..i]
                    .iter()
                    .rev()
                    .zip(m.rows[i..].iter())
                    .map(|(a, b)| a.iter().zip(b.iter()).filter(|(i, j)| i != j).count())
                    .sum::<usize>()
                    == 1
                {
                    return i as i64 * 100;
                }
            }
            for i in 1..m.width {
                if columnwise[0..i]
                    .iter()
                    .rev()
                    .zip(columnwise[i..].iter())
                    .map(|(a, b)| a.iter().zip(b.iter()).filter(|(i, j)| i != j).count())
                    .sum::<usize>()
                    == 1
                {
                    return i as i64;
                }
            }
            panic!("{n}: no valid reflection");
        })
        .sum()
}
