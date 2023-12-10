use std::fmt::Display;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Tile {
    Start,
    Empty,
    NS,
    NE,
    NW,
    SW,
    SE,
    EW,
}

impl From<(Dir, Dir)> for Tile {
    fn from(v: (Dir, Dir)) -> Self {
        match v {
            (Dir::N, Dir::S) | (Dir::S, Dir::N) => Tile::NS,
            (Dir::N, Dir::E) | (Dir::E, Dir::N) => Tile::NE,
            (Dir::N, Dir::W) | (Dir::W, Dir::N) => Tile::NW,
            (Dir::S, Dir::E) | (Dir::E, Dir::S) => Tile::SE,
            (Dir::S, Dir::W) | (Dir::W, Dir::S) => Tile::SW,
            (Dir::E, Dir::W) | (Dir::W, Dir::E) => Tile::EW,
            _ => Tile::Empty,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Start => "?",
                Tile::Empty => " ",
                Tile::NS => "│",
                Tile::EW => "─",
                Tile::NW => "┘",
                Tile::NE => "└",
                Tile::SW => "┐",
                Tile::SE => "┌",
            }
        )
    }
}

impl Tile {
    /// Returns true iff this tile has an exit in the specified direction.
    fn exits_toward(&self, dir: Dir) -> bool {
        match dir {
            Dir::N => *self == Tile::NS || *self == Tile::NE || *self == Tile::NW,
            Dir::S => *self == Tile::NS || *self == Tile::SE || *self == Tile::SW,
            Dir::E => *self == Tile::NE || *self == Tile::SE || *self == Tile::EW,
            Dir::W => *self == Tile::NW || *self == Tile::SW || *self == Tile::EW,
        }
    }

    /// Assuming we entered through `dir`, returns the other exit for this tile.
    fn thru(self, dir: Dir) -> Option<Dir> {
        match dir {
            Dir::N => match self {
                Tile::NS => Some(Dir::S),
                Tile::NW => Some(Dir::W),
                Tile::NE => Some(Dir::E),
                _ => None,
            },
            Dir::S => match self {
                Tile::NS => Some(Dir::N),
                Tile::SW => Some(Dir::W),
                Tile::SE => Some(Dir::E),
                _ => None,
            },
            Dir::E => match self {
                Tile::EW => Some(Dir::W),
                Tile::SE => Some(Dir::S),
                Tile::NE => Some(Dir::N),
                _ => None,
            },
            Dir::W => match self {
                Tile::EW => Some(Dir::E),
                Tile::SW => Some(Dir::S),
                Tile::NW => Some(Dir::N),
                _ => None,
            },
        }
    }

    /// Returns true iff this tile is a corner.
    fn is_corner(self) -> bool {
        self == Tile::NW || self == Tile::NE || self == Tile::SE || self == Tile::SW
    }

    /// Returns true iff `self` and `other` are opposite corners from one another (e.g., NW and
    /// SE).
    fn is_opposite_corner(self, other: Self) -> bool {
        (self == Tile::NW && other == Tile::SE)
            || (self == Tile::SE && other == Tile::NW)
            || (self == Tile::NE && other == Tile::SW)
            || (self == Tile::SW && other == Tile::NE)
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn inverse(self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    start: (usize, usize),
}

impl Map {
    /// Returns true iff the tile in the given directon from `(x, y)` has an exit leading to `(x,
    /// y)`.
    fn is_connected(&self, (x, y): (usize, usize), dir: Dir) -> bool {
        self.neighbor((x, y), dir)
            .is_some_and(|(xp, yp)| self.tiles[yp][xp].exits_toward(dir.inverse()))
    }
    /// Returns the coordinates of the tile in the given direction from the tile at `(x, y)`, if it
    /// exists.
    fn neighbor(&self, (x, y): (usize, usize), dir: Dir) -> Option<(usize, usize)> {
        match dir {
            Dir::N => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Dir::S => {
                if y >= self.tiles.len() {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Dir::W => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            Dir::E => {
                if x >= self.tiles[y].len() {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
        }
    }

    /// Follow the maze.
    fn travel(&self, v: MazeVector) -> Result<MazeVector, String> {
        let tile = self.tiles[v.pos.1][v.pos.0];
        let exit_dir = tile
            .thru(v.dir)
            .ok_or::<String>("Invalid direction of travel".into())?;
        let exit_pos = self
            .neighbor(v.pos, exit_dir)
            .ok_or::<String>("Traveled off the map!".into())?;
        Ok(MazeVector {
            dir: exit_dir.inverse(),
            pos: exit_pos,
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.tiles {
            for tile in line {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct MazeVector {
    /// Incoming direction of travel.
    dir: Dir,
    /// Current location.
    pos: (usize, usize),
}

#[aoc_generator(day10)]
fn generate(input: &str) -> Map {
    let mut start = None;
    let tiles = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = Some((x, y));
                        Tile::Start
                    }
                    '|' => Tile::NS,
                    '-' => Tile::EW,
                    'L' => Tile::NE,
                    'J' => Tile::NW,
                    '7' => Tile::SW,
                    'F' => Tile::SE,
                    '.' => Tile::Empty,
                    _ => panic!("invalid char {:?}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Map {
        height: tiles.len(),
        width: tiles[0].len(),
        tiles,
        start: start.unwrap(),
    }
}

#[aoc(day10, part1)]
fn solve_part1(input: &Map) -> i64 {
    // Find the valid directions leading from the start location.
    let start_dirs = [Dir::N, Dir::S, Dir::E, Dir::W]
        .into_iter()
        .filter(|dir| input.is_connected(input.start, *dir))
        .collect::<Vec<_>>();
    // There should be exactly two exits.
    assert_eq!(start_dirs.len(), 2);

    // Then, walk both directions until they meet or cross over.
    let mut left = MazeVector {
        dir: start_dirs[0].inverse(),
        pos: input.neighbor(input.start, start_dirs[0]).unwrap(),
    };

    let mut right = MazeVector {
        dir: start_dirs[1].inverse(),
        pos: input.neighbor(input.start, start_dirs[1]).unwrap(),
    };

    let mut len = 1;

    let mut prev_left = input.start;
    let mut prev_right = input.start;

    loop {
        if prev_left == right.pos || prev_right == left.pos || right.pos == left.pos {
            break;
        }
        prev_left = left.pos;
        prev_right = right.pos;
        left = input.travel(left).unwrap();
        right = input.travel(right).unwrap();
        len += 1;
    }
    len
}

#[aoc(day10, part2)]
fn solve_part2(input: &Map) -> i64 {
    // Find the valid directions leading from the start location.
    let start_dirs = [Dir::N, Dir::S, Dir::E, Dir::W]
        .into_iter()
        .filter(|dir| input.is_connected(input.start, *dir))
        .collect::<Vec<_>>();
    // There should be exactly two exits.
    assert_eq!(start_dirs.len(), 2);

    // First, we walk the full loop from the start point, storing each tile we visit in a fresh grid.
    let mut current = MazeVector {
        dir: start_dirs[0].inverse(),
        pos: input.neighbor(input.start, start_dirs[0]).unwrap(),
    };

    let mut tiles = vec![vec![Tile::Empty; input.width]; input.height];
    loop {
        tiles[current.pos.1][current.pos.0] = input.tiles[current.pos.1][current.pos.0];
        current = input.travel(current).unwrap();
        if current.pos == input.start {
            break;
        }
    }

    // Fill in the start tile with what it actually is.
    tiles[input.start.1][input.start.0] = (start_dirs[0], start_dirs[1]).into();

    // Now, for each row of the grid, we use the even-odd rule to determine which x values are
    // enclosed by the path.
    let mut area = 0;
    for y in 0..input.height {
        let mut inside = false;
        let mut last_corner = Tile::Empty;
        for x in 0..input.width {
            if tiles[y][x] == Tile::NS {
                inside = !inside;
            } else if tiles[y][x].is_corner() && last_corner == Tile::Empty {
                inside = !inside;
                last_corner = tiles[y][x];
            } else if tiles[y][x].is_corner() {
                if !last_corner.is_opposite_corner(tiles[y][x]) {
                    inside = !inside;
                }
                last_corner = Tile::Empty;
            } else if tiles[y][x] == Tile::Empty {
                if inside {
                    area += 1;
                }
            }
        }
    }

    area as i64
}
