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
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct Bounds {
    origin: (i64, i64),
    width: usize,
    height: usize,
}

pub trait Plane {
    fn origin(&self) -> (i64, i64);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn extract(&self) -> Bounds {
        Bounds {
            origin: self.origin(),
            width: self.width(),
            height: self.height(),
        }
    }
}

impl<T> Plane for array2d::Array2D<T> {
    fn origin(&self) -> (i64, i64) {
        (0, 0)
    }
    fn width(&self) -> usize {
        self.num_columns()
    }
    fn height(&self) -> usize {
        self.num_rows()
    }
}

impl Plane for Bounds {
    fn origin(&self) -> (i64, i64) {
        self.origin
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}

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

    pub fn delta(self) -> (i64, i64) {
        match self {
            Dir::N => (-1, 0),
            Dir::S => (1, 0),
            Dir::E => (0, 1),
            Dir::W => (0, -1),
        }
    }

    pub fn neighbor<P: Plane>(self, (y, x): (i64, i64), p: &P) -> Option<(i64, i64)> {
        let (min_x, min_y) = p.origin();
        let max_x = min_x + p.width() as i64;
        let max_y = min_y + p.height() as i64;
        match self {
            Dir::N => {
                if y == min_y {
                    None
                } else {
                    Some((y - 1, x))
                }
            }
            Dir::S => {
                if y >= max_y - 1 {
                    None
                } else {
                    Some((y + 1, x))
                }
            }
            Dir::W => {
                if x == min_x {
                    None
                } else {
                    Some((y, x - 1))
                }
            }
            Dir::E => {
                if x >= max_x - 1 {
                    None
                } else {
                    Some((y, x + 1))
                }
            }
        }
    }

    pub fn wrapping_neighbor<P: Plane>(self, (y, x): (i64, i64), p: &P) -> (i64, i64) {
        let (min_x, min_y) = p.origin();
        let max_x = min_x + p.width() as i64;
        let max_y = min_y + p.height() as i64;
        match self {
            Dir::N => {
                if y == min_y {
                    (max_y - 1, x)
                } else {
                    (y - 1, x)
                }
            }
            Dir::S => {
                if y >= max_y - 1 {
                    (min_y, x)
                } else {
                    (y + 1, x)
                }
            }
            Dir::W => {
                if x == min_x {
                    (y, max_x - 1)
                } else {
                    (y, x - 1)
                }
            }
            Dir::E => {
                if x >= max_x - 1 {
                    (y, min_x)
                } else {
                    (y, x + 1)
                }
            }
        }
    }
    pub fn neighbors<P: Plane>((y, x): (i64, i64), p: &P) -> impl Iterator<Item = (i64, i64)> {
        let bounds = p.extract();
        [Dir::N, Dir::S, Dir::E, Dir::W]
            .into_iter()
            .filter_map(move |dir| dir.neighbor((y, x), &bounds))
    }
}

fn gcd<T: std::ops::Rem<Output = T> + Eq + Default + Copy>(a: T, b: T) -> T {
    if b == T::default() {
        a
    } else {
        gcd(b, a % b)
    }
}

aoc_lib! { year = 2023 }
