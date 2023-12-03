use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct NumberSpan {
    x: usize,
    y: usize,
    len: usize,
}

impl NumberSpan {
    fn parse(&self, grid: &Box<[Box<[char]>]>) -> Option<u32> {
        grid[self.y][self.x..(self.x + self.len)]
            .iter()
            .collect::<String>()
            .parse()
            .ok()
    }

    fn neighbors(&self, (width, height): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let mut v = vec![];
        if self.y < height - 1 {
            for i in 0..self.len {
                v.push((self.x + i, self.y + 1))
            }
        }
        if self.y > 0 {
            for i in 0..self.len {
                v.push((self.x + i, self.y - 1))
            }
        }
        if self.x > 0 {
            v.push((self.x - 1, self.y));
            if self.y > 0 {
                v.push((self.x - 1, self.y - 1))
            }
            if self.y < height - 1 {
                v.push((self.x - 1, self.y + 1))
            }
        }
        if self.x + self.len < width {
            v.push((self.x + self.len, self.y));
            if self.y > 0 {
                v.push((self.x + self.len, self.y - 1))
            }
            if self.y < height - 1 {
                v.push((self.x + self.len, self.y + 1))
            }
        }
        v.into_iter()
    }
}

enum State {
    Start,
    Number(usize),
}

impl State {
    fn new() -> Self {
        State::Start
    }
}

#[derive(Debug)]
struct SymbolLocation {
    x: usize,
    y: usize,
}

impl SymbolLocation {
    fn neighbors(&self, (width, height): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let mut v = vec![];
        if self.x > 0 {
            v.push((self.x - 1, self.y));
            if self.y > 0 {
                v.push((self.x - 1, self.y - 1));
            }
            if self.y + 1 < height {
                v.push((self.x - 1, self.y + 1));
            }
        }
        if self.x + 1 < width {
            v.push((self.x + 1, self.y));
            if self.y > 0 {
                v.push((self.x + 1, self.y - 1));
            }
            if self.y + 1 < height {
                v.push((self.x + 1, self.y + 1));
            }
        }
        if self.y > 0 {
            v.push((self.x, self.y - 1));
        }
        if self.y + 1 < height {
            v.push((self.x, self.y + 1));
        }

        v.into_iter()
    }
}

struct StateMachine {
    state: State,
    numbers: Vec<NumberSpan>,
    symbols: Vec<SymbolLocation>,
    x: usize,
    y: usize,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: State::new(),
            numbers: vec![],
            symbols: vec![],
            x: 0,
            y: 0,
        }
    }
    fn process(&mut self, c: char) {
        self.state = match self.state {
            State::Start => match c {
                '.' => State::Start,
                '0'..='9' => State::Number(self.x),
                '\0' => State::Start,
                _ => {
                    self.symbols.push(SymbolLocation {
                        x: self.x,
                        y: self.y,
                    });
                    State::Start
                }
            },
            State::Number(v) => match c {
                '0'..='9' => State::Number(v),
                other => {
                    self.numbers.push(NumberSpan {
                        x: v,
                        y: self.y,
                        len: self.x - v,
                    });
                    match other {
                        '.' | '\0' => State::Start,
                        _ => {
                            self.symbols.push(SymbolLocation {
                                x: self.x,
                                y: self.y,
                            });
                            State::Start
                        }
                    }
                }
            },
        };
        if c == '\0' {
            self.x = 0;
            self.y += 1;
        } else {
            self.x += 1;
        }
    }
}

struct PuzzleInput {
    numbers: Vec<NumberSpan>,
    symbols: Vec<SymbolLocation>,
    grid: Box<[Box<[char]>]>,
    width: usize,
    height: usize,
}

#[aoc_generator(day3)]
fn generate<'a>(input: &str) -> PuzzleInput {
    let mut machine = StateMachine::new();
    for line in input.lines() {
        for c in line.chars().chain(['\0'].into_iter()) {
            machine.process(c);
        }
    }

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Box<[_]>>())
        .collect::<Box<[_]>>();

    PuzzleInput {
        numbers: machine.numbers,
        symbols: machine.symbols,
        width: grid[0].len(),
        height: grid.len(),
        grid,
    }
}

#[aoc(day3, part1)]
fn solve_part1(input: &PuzzleInput) -> u32 {
    let mut sum = 0;
    for span in &input.numbers {
        for (x, y) in span.neighbors((input.width, input.height)) {
            if match input.grid[y][x] {
                '.' | '0'..='9' => false,
                _ => true,
            } {
                sum += span.parse(&input.grid).unwrap();
                break;
            }
        }
    }
    sum
}

use std::collections::BTreeSet;

#[aoc(day3, part2)]
fn solve_part2(input: &PuzzleInput) -> u32 {
    let mut sum = 0;
    let mut num_grid: Vec<Vec<Option<usize>>> = vec![vec![None; input.width]; input.height];
    for (span_id, span) in input.numbers.iter().enumerate() {
        for i in 0..span.len {
            num_grid[span.y][span.x + i] = Some(span_id);
        }
    }

    for sym in input
        .symbols
        .iter()
        .filter(|sym| input.grid[sym.y][sym.x] == '*')
    {
        let neighboring_numbers = sym
            .neighbors((input.width, input.height))
            .filter_map(|(x, y)| num_grid[y][x])
            .collect::<BTreeSet<usize>>();
        if neighboring_numbers.len() == 2 {
            sum += neighboring_numbers
                .into_iter()
                .map(|i| input.numbers[i].parse(&input.grid).unwrap())
                .product::<u32>();
        }
    }
    sum
}
