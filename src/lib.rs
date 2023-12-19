use aoc_runner_derive::aoc_lib;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    pub fn cw(self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }
    pub fn ccw(self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::W => Dir::S,
            Dir::S => Dir::E,
            Dir::E => Dir::N,
        }
    }
    pub fn inverse(self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }
    pub fn neighbor<T>(
        self,
        (y, x): (usize, usize),
        a: &array2d::Array2D<T>,
    ) -> Option<(usize, usize)> {
        match self {
            Dir::N => {
                if y == 0 {
                    None
                } else {
                    Some((y - 1, x))
                }
            }
            Dir::S => {
                if y >= a.num_rows() - 1 {
                    None
                } else {
                    Some((y + 1, x))
                }
            }
            Dir::W => {
                if x == 0 {
                    None
                } else {
                    Some((y, x - 1))
                }
            }
            Dir::E => {
                if x >= a.num_columns() - 1 {
                    None
                } else {
                    Some((y, x + 1))
                }
            }
        }
    }
}

aoc_lib! { year = 2023 }
