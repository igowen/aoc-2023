use crate::Dir;
use aoc_runner_derive::{aoc, aoc_generator};
use array2d::Array2D;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
enum Mirror {
    /// '/'
    Forward,
    /// '\'
    Backward,
    /// '|'
    SplitVertical,
    /// '-'
    SplitHorizontal,
    /// '.'
    Empty,
}

#[derive(Clone, Debug)]
struct Input {
    map: Array2D<Mirror>,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct LightVector {
    origin: (i64, i64),
    dir: Dir,
}

#[aoc_generator(day16)]
fn generate(input: &str) -> Input {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let map = Array2D::from_iter_row_major(
        input
            .lines()
            .map(|line| {
                line.chars().map(|c| match c {
                    '/' => Mirror::Forward,
                    '\\' => Mirror::Backward,
                    '|' => Mirror::SplitVertical,
                    '-' => Mirror::SplitHorizontal,
                    '.' => Mirror::Empty,
                    _ => panic!("invalid char {c}"),
                })
            })
            .flatten(),
        height,
        width,
    )
    .unwrap();
    Input { map }
}

fn trace(input: &Input, start: LightVector) -> i64 {
    let mut energized = Array2D::<(bool, (bool, bool, bool, bool))>::filled_with(
        (false, (false, false, false, false)),
        input.map.num_rows(),
        input.map.num_columns(),
    );
    let mut current = vec![start];
    let mut splits = vec![];
    while !current.is_empty() {
        for v in current.iter_mut() {
            energized[(v.origin.0 as usize, v.origin.1 as usize)].0 = true;
            match input.map[(v.origin.0 as usize, v.origin.1 as usize)] {
                Mirror::Forward => match v.dir {
                    Dir::E | Dir::W => v.dir = v.dir.ccw(),
                    Dir::N | Dir::S => v.dir = v.dir.cw(),
                },
                Mirror::Backward => match v.dir {
                    Dir::E | Dir::W => v.dir = v.dir.cw(),
                    Dir::N | Dir::S => v.dir = v.dir.ccw(),
                },
                Mirror::SplitHorizontal => {
                    if v.dir == Dir::N || v.dir == Dir::S {
                        v.dir = Dir::E;
                        splits.push(LightVector {
                            origin: v.origin,
                            dir: Dir::W,
                        });
                    }
                }
                Mirror::SplitVertical => {
                    if v.dir == Dir::E || v.dir == Dir::W {
                        v.dir = Dir::N;
                        splits.push(LightVector {
                            origin: v.origin,
                            dir: Dir::S,
                        });
                    }
                }
                Mirror::Empty => {}
            }
        }
        current.extend(splits.drain(..));
        current.retain_mut(|v| {
            if let Some(next) = v.dir.neighbor(v.origin, &input.map) {
                let unext = (next.0 as usize, next.1 as usize);
                let visited = match v.dir {
                    Dir::N => &mut energized[unext].1 .0,
                    Dir::S => &mut energized[unext].1 .1,
                    Dir::E => &mut energized[unext].1 .2,
                    Dir::W => &mut energized[unext].1 .3,
                };
                if !*visited {
                    v.origin = next;
                    *visited = true;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        });
    }

    energized
        .elements_row_major_iter()
        .filter(|(b, _)| *b)
        .count() as i64
}

#[aoc(day16, part1)]
fn solve_part1(input: &Input) -> i64 {
    trace(
        input,
        LightVector {
            origin: (0, 0),
            dir: Dir::E,
        },
    )
}

#[aoc(day16, part2)]
fn solve_part2(input: &Input) -> i64 {
    let mut max = 0;
    for x in 0..input.map.num_columns() {
        max = max.max(trace(
            input,
            LightVector {
                origin: (0, x as i64),
                dir: Dir::S,
            },
        ));
        max = max.max(trace(
            input,
            LightVector {
                origin: (input.map.num_rows() as i64 - 1, x as i64),
                dir: Dir::N,
            },
        ));
    }
    for y in 0..input.map.num_rows() {
        max = max.max(trace(
            input,
            LightVector {
                origin: (y as i64, 0),
                dir: Dir::E,
            },
        ));
        max = max.max(trace(
            input,
            LightVector {
                origin: (y as i64, input.map.num_columns() as i64 - 1),
                dir: Dir::W,
            },
        ));
    }
    max
}
