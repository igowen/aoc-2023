use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
enum Condition {
    Ok,
    Damaged,
    Unknown,
}

#[derive(Clone, Debug)]
struct Row {
    conditions: Box<[Condition]>,
    segments: Box<[usize]>,
}

impl Row {
    fn unfold(&self) -> Self {
        let new_condition_length = (self.conditions.len() + 1) * 5 - 1;
        let new_segment_length = self.segments.len() * 5;
        Self {
            conditions: self
                .conditions
                .iter()
                .copied()
                .chain(std::iter::once(Condition::Unknown))
                .cycle()
                .take(new_condition_length)
                .collect(),
            segments: self
                .segments
                .iter()
                .copied()
                .cycle()
                .take(new_segment_length)
                .collect(),
        }
    }
}

#[aoc_generator(day12)]
fn generate(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| {
            let mut s = line.split(" ");
            let conditions = s.next().unwrap();
            let segments = s.next().unwrap();
            Row {
                conditions: conditions
                    .chars()
                    .map(|c| match c {
                        '.' => Condition::Ok,
                        '#' => Condition::Damaged,
                        '?' => Condition::Unknown,
                        _ => panic!("invalid char {:?}", c),
                    })
                    .collect(),
                segments: segments.split(",").map(|s| s.parse().unwrap()).collect(),
            }
        })
        .collect()
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Remainder<'a> {
    conditions: &'a [Condition],
    segments: &'a [usize],
    current_segment: Option<usize>,
}

impl<'a> From<&'a Row> for Remainder<'a> {
    fn from(row: &'a Row) -> Self {
        Self {
            conditions: &row.conditions[..],
            segments: &row.segments[..],
            current_segment: None,
        }
    }
}

type Memo<'a> = HashMap<Remainder<'a>, i64>;

impl<'a> Remainder<'a> {
    fn handle_ok(&self, memo: &mut Memo<'a>) -> i64 {
        match self.current_segment {
            None => {
                // Fine.
                let next = Remainder {
                    conditions: &self.conditions[1..],
                    segments: self.segments,
                    current_segment: None,
                };
                next.count(memo)
            }
            Some(n) => {
                if n == 0 {
                    // Fine. clear current segment and proceed.
                    let next = Remainder {
                        conditions: &self.conditions[1..],
                        segments: self.segments,
                        current_segment: None,
                    };
                    next.count(memo)
                } else {
                    // Invalid. bail out
                    0
                }
            }
        }
    }

    fn handle_damaged(&self, memo: &mut Memo<'a>) -> i64 {
        match self.current_segment {
            None => {
                if self.segments.is_empty() {
                    0
                } else {
                    // Fine. pop the next segment and proceed.
                    let next_segment = self.segments[0];
                    let next = Remainder {
                        conditions: &self.conditions[1..],
                        segments: &self.segments[1..],
                        current_segment: Some(next_segment - 1),
                    };
                    next.count(memo)
                }
            }
            Some(n) => {
                if n == 0 {
                    // Invalid. bail out.
                    0
                } else {
                    // Fine. subtract one from the current segment and proceed.
                    let next = Remainder {
                        conditions: &self.conditions[1..],
                        segments: self.segments,
                        current_segment: Some(n - 1),
                    };
                    next.count(memo)
                }
            }
        }
    }
    fn count(&self, memo: &mut Memo<'a>) -> i64 {
        if !memo.contains_key(self) {
            if self.segments.is_empty()
                && self.conditions.is_empty()
                && self.current_segment.unwrap_or(0) == 0
            {
                return 1;
            }

            if self.conditions.is_empty() {
                return 0;
            }

            let count = match self.conditions[0] {
                Condition::Ok => self.handle_ok(memo),
                Condition::Damaged => self.handle_damaged(memo),
                Condition::Unknown => {
                    // Try both.
                    self.handle_ok(memo) + self.handle_damaged(memo)
                }
            };
            memo.insert(*self, count);
            return count;
        }
        *memo.get(self).unwrap()
    }
}

#[aoc(day12, part1)]
fn solve_part1(input: &Vec<Row>) -> i64 {
    let mut map = HashMap::new();
    input
        .iter()
        .map(|row| {
            let r: Remainder = row.into();
            r.count(&mut map)
        })
        .sum()
}

#[aoc(day12, part2)]
fn solve_part2(input: &Vec<Row>) -> i64 {
    let unfolded = input.iter().map(|row| row.unfold()).collect::<Vec<_>>();
    let mut map = HashMap::new();
    let n = unfolded
        .iter()
        .map(|row| {
            let r: Remainder = row.into();
            r.count(&mut map)
        })
        .sum();
    n
}
