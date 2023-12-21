use crate::Dir;
use aoc_runner_derive::{aoc, aoc_generator};
use array2d::Array2D;

#[derive(Clone, Debug)]
struct Input {
    map: Array2D<u8>,
}

#[aoc_generator(day17)]
fn generate(input: &str) -> Input {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let map = Array2D::from_iter_row_major(
        input
            .lines()
            .map(|line| line.chars().map(|c| c.to_string().parse().unwrap()))
            .flatten(),
        height,
        width,
    )
    .unwrap();
    Input { map }
}

#[derive(Clone, Debug)]
struct Cart {
    pos: (usize, usize),
    dir: Dir,
    dir_remaining: u8,
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
struct DirMap<T> {
    dirs: [T; 4],
}

impl<T: Clone> DirMap<T> {
    fn filled_with(v: T) -> Self {
        DirMap {
            dirs: [v.clone(), v.clone(), v.clone(), v],
        }
    }
}

impl<T> std::ops::Index<Dir> for DirMap<T> {
    type Output = T;
    fn index(&self, index: Dir) -> &Self::Output {
        &self.dirs[match index {
            Dir::N => 0,
            Dir::S => 1,
            Dir::E => 2,
            Dir::W => 3,
        }]
    }
}

impl<T> std::ops::IndexMut<Dir> for DirMap<T> {
    fn index_mut(&mut self, index: Dir) -> &mut Self::Output {
        &mut self.dirs[match index {
            Dir::N => 0,
            Dir::S => 1,
            Dir::E => 2,
            Dir::W => 3,
        }]
    }
}

#[aoc(day17, part1)]
fn solve_part1(input: &Input) -> i64 {
    // Each element represents the smallest cost traveling through the current position in each
    // different direction, broken down by number of remaining moves in the current direction.
    // sps[(y, x)][Dir::N][0] -> traveling north through (y, x) with 0 straight moves left (i.e.,
    // the next move must be a turn).
    let mut sps = Array2D::<DirMap<[Option<i64>; 3]>>::filled_with(
        Default::default(),
        input.map.num_rows(),
        input.map.num_columns(),
    );

    let mut current = vec![];

    for dir in [Dir::S, Dir::E].into_iter() {
        current.push(Cart {
            pos: (0, 0),
            dir,
            dir_remaining: 2,
        });
    }

    sps[(0, 0)] = DirMap::filled_with([
        None,
        None,
        // For some reason, the start location does not contribute to heat loss.
        Some(0),
    ]);

    let mut next = vec![];
    while !current.is_empty() {
        for cart in current.drain(..) {
            let partial = sps[cart.pos][cart.dir][cart.dir_remaining as usize].unwrap();
            for dir in [cart.dir.cw(), cart.dir.ccw()].into_iter() {
                if let Some(pos) = dir.neighbor((cart.pos.0 as i64, cart.pos.1 as i64), &input.map)
                {
                    let pos = (pos.0 as usize, pos.1 as usize);
                    let next_cart = Cart {
                        pos,
                        dir,
                        dir_remaining: 2,
                    };
                    let sp = &mut sps[pos][dir][2];
                    match sp {
                        None => {
                            *sp = Some(partial + input.map[pos] as i64);
                            next.push(next_cart);
                        }
                        Some(existing_cost) => {
                            if (partial + input.map[pos] as i64) < *existing_cost {
                                *existing_cost = partial + input.map[pos] as i64;
                                next.push(next_cart);
                            }
                        }
                    }
                }
            }
            if cart.dir_remaining > 0 {
                if let Some(pos) = cart
                    .dir
                    .neighbor((cart.pos.0 as i64, cart.pos.1 as i64), &input.map)
                {
                    let pos = (pos.0 as usize, pos.1 as usize);
                    let next_cart = Cart {
                        pos,
                        dir: cart.dir,
                        dir_remaining: cart.dir_remaining - 1,
                    };
                    let sp = &mut sps[pos][cart.dir][cart.dir_remaining as usize - 1];
                    match sp {
                        None => {
                            *sp = Some(partial + input.map[pos] as i64);
                            next.push(next_cart);
                        }
                        Some(existing_cost) => {
                            if (partial + input.map[pos] as i64) < *existing_cost {
                                *existing_cost = partial + input.map[pos] as i64;
                                next.push(next_cart);
                            }
                        }
                    }
                }
            }
        }
        current.extend(next.drain(..));
    }

    let target = (input.map.num_rows() - 1, input.map.num_columns() - 1);

    let min = [Dir::S, Dir::E]
        .into_iter()
        .map(|dir| sps[target][dir].iter())
        .flatten()
        .filter_map(|sp| *sp)
        .min()
        .unwrap();

    min
}

#[aoc(day17, part2)]
fn solve_part2(input: &Input) -> i64 {
    // This is the same as part one but we need to track up to 10 straight moves remaining.
    let mut sps = Array2D::<DirMap<[Option<i64>; 10]>>::filled_with(
        Default::default(),
        input.map.num_rows(),
        input.map.num_columns(),
    );

    let mut current = vec![];

    for dir in [Dir::S, Dir::E].into_iter() {
        current.push(Cart {
            pos: (0, 0),
            dir,
            dir_remaining: 9,
        });
    }

    sps[(0, 0)] = DirMap::filled_with([
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(0),
    ]);

    let mut next = vec![];
    while !current.is_empty() {
        for cart in current.drain(..) {
            // This will have been filled by the previous iteration of the outer loop.
            let partial = sps[cart.pos][cart.dir][cart.dir_remaining as usize].unwrap();
            if cart.dir_remaining < 7 {
                for dir in [cart.dir.cw(), cart.dir.ccw()].into_iter() {
                    if let Some(pos) =
                        dir.neighbor((cart.pos.0 as i64, cart.pos.1 as i64), &input.map)
                    {
                        let pos = (pos.0 as usize, pos.1 as usize);
                        let next_cart = Cart {
                            pos,
                            dir,
                            dir_remaining: 9,
                        };
                        let sp = &mut sps[pos][dir][9];
                        match sp {
                            None => {
                                *sp = Some(partial + input.map[pos] as i64);
                                next.push(next_cart);
                            }
                            Some(existing_cost) => {
                                if (partial + input.map[pos] as i64) < *existing_cost {
                                    *existing_cost = partial + input.map[pos] as i64;
                                    next.push(next_cart);
                                }
                            }
                        }
                    }
                }
            }
            if cart.dir_remaining > 0 {
                if let Some(pos) = cart
                    .dir
                    .neighbor((cart.pos.0 as i64, cart.pos.1 as i64), &input.map)
                {
                    let pos = (pos.0 as usize, pos.1 as usize);
                    let next_cart = Cart {
                        pos,
                        dir: cart.dir,
                        dir_remaining: cart.dir_remaining - 1,
                    };
                    let sp = &mut sps[pos][cart.dir][cart.dir_remaining as usize - 1];
                    match sp {
                        None => {
                            *sp = Some(partial + input.map[pos] as i64);
                            next.push(next_cart);
                        }
                        Some(existing_cost) => {
                            if (partial + input.map[pos] as i64) < *existing_cost {
                                *existing_cost = partial + input.map[pos] as i64;
                                next.push(next_cart);
                            }
                        }
                    }
                }
            }
        }
        current.extend(next.drain(..));
    }

    let target = (input.map.num_rows() - 1, input.map.num_columns() - 1);
    let min = [Dir::S, Dir::E]
        .into_iter()
        // It's impossible to end with a dir_remaining of > 7 since we must end after moving at
        // least 4 squares.
        .map(|dir| sps[target][dir].iter().take(7))
        .flatten()
        .filter_map(|sp| *sp)
        .min()
        .unwrap();

    min
}
