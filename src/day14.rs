use crate::Dir;
use std::{collections::HashMap, fmt::Display};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
enum Rock {
    Round,
    Square,
    None,
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rock::Round => write!(f, "O"),
            Rock::Square => write!(f, "#"),
            Rock::None => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map {
    rocks: Vec<Vec<Rock>>,
    width: usize,
    height: usize,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rocks {
            for c in row {
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn tilt(&mut self, dir: Dir) {
        match dir {
            Dir::N => {
                for x in 0..self.width {
                    let mut current = 0;
                    for y in (0..self.height).rev() {
                        match self.rocks[y][x] {
                            Rock::Round => {
                                current += 1;
                                self.rocks[y][x] = Rock::None;
                            }
                            Rock::Square => {
                                for i in 0..current {
                                    self.rocks[y + i + 1][x] = Rock::Round;
                                }
                                current = 0;
                            }
                            Rock::None => {}
                        }
                    }
                    for i in 0..current {
                        self.rocks[i][x] = Rock::Round;
                    }
                }
            }
            Dir::S => {
                for x in 0..self.width {
                    let mut current = 0;
                    for y in 0..self.height {
                        match self.rocks[y][x] {
                            Rock::Round => {
                                current += 1;
                                self.rocks[y][x] = Rock::None;
                            }
                            Rock::Square => {
                                for i in 0..current {
                                    self.rocks[y - (i + 1)][x] = Rock::Round;
                                }
                                current = 0;
                            }
                            Rock::None => {}
                        }
                    }
                    for i in 0..current {
                        self.rocks[self.height - 1 - i][x] = Rock::Round;
                    }
                }
            }
            Dir::W => {
                for y in 0..self.height {
                    let mut current = 0;
                    for x in (0..self.width).rev() {
                        match self.rocks[y][x] {
                            Rock::Round => {
                                current += 1;
                                self.rocks[y][x] = Rock::None;
                            }
                            Rock::Square => {
                                for i in 0..current {
                                    self.rocks[y][x + i + 1] = Rock::Round;
                                }
                                current = 0;
                            }
                            Rock::None => {}
                        }
                    }
                    for i in 0..current {
                        self.rocks[y][i] = Rock::Round;
                    }
                }
            }
            Dir::E => {
                for y in 0..self.height {
                    let mut current = 0;
                    for x in 0..self.width {
                        match self.rocks[y][x] {
                            Rock::Round => {
                                current += 1;
                                self.rocks[y][x] = Rock::None;
                            }
                            Rock::Square => {
                                for i in 0..current {
                                    self.rocks[y][x - (i + 1)] = Rock::Round;
                                }
                                current = 0;
                            }
                            Rock::None => {}
                        }
                    }
                    for i in 0..current {
                        self.rocks[y][self.width - 1 - i] = Rock::Round;
                    }
                }
            }
        }
    }

    fn load(&self) -> i64 {
        self.rocks
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| (row.iter().filter(|r| **r == Rock::Round).count() * (i + 1)) as i64)
            .sum()
    }
}

#[aoc_generator(day14)]
fn generate(input: &str) -> Map {
    let rocks = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Rock::Round,
                    '#' => Rock::Square,
                    '.' => Rock::None,
                    _ => panic!("invalid char {c}"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Map {
        width: rocks[0].len(),
        height: rocks.len(),
        rocks,
    }
}

#[aoc(day14, part1)]
fn solve_part1(input: &Map) -> i64 {
    let mut map = input.clone();
    map.tilt(Dir::N);
    map.load()
}

#[aoc(day14, part2)]
fn solve_part2(input: &Map) -> i64 {
    let mut map = input.clone();
    let mut seen = HashMap::new();
    seen.insert(map.clone(), 0);
    let n = loop {
        map.tilt(Dir::N);
        map.tilt(Dir::W);
        map.tilt(Dir::S);
        map.tilt(Dir::E);

        if let Some(n) = seen.get(&map) {
            break n;
        }
        seen.insert(map.clone(), seen.len());
    };

    let cycle_length = seen.len() - n;
    let steps = (1000000000 - seen.len()) % cycle_length;

    for _ in 0..steps {
        map.tilt(Dir::N);
        map.tilt(Dir::W);
        map.tilt(Dir::S);
        map.tilt(Dir::E);
    }

    map.load()
}
