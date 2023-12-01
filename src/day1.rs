use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn generate<'a>(input: &str) -> Vec<String> {
    input.lines().map(|s| s.into()).collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &Vec<String>) -> u32 {
    let mut total = 0u32;
    for line in input {
        let mut val = 0u32;
        for c in line.chars() {
            if let Some(i) = c.to_digit(10) {
                val = i * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if let Some(i) = c.to_digit(10) {
                val += i;
                break;
            }
        }
        total += val;
    }
    total
}

fn parse_leading<S: AsRef<str>>(s: S) -> Option<u32> {
    let s = s.as_ref();
    if let Some(i) = s.chars().next()?.to_digit(10) {
        Some(i)
    } else if s.starts_with("one") {
        Some(1)
    } else if s.starts_with("two") {
        Some(2)
    } else if s.starts_with("three") {
        Some(3)
    } else if s.starts_with("four") {
        Some(4)
    } else if s.starts_with("five") {
        Some(5)
    } else if s.starts_with("six") {
        Some(6)
    } else if s.starts_with("seven") {
        Some(7)
    } else if s.starts_with("eight") {
        Some(8)
    } else if s.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

#[aoc(day1, part2)]
fn solve_part2(input: &Vec<String>) -> u32 {
    let mut total = 0u32;
    for line in input {
        let mut val = 0u32;
        for i in 0..line.len() {
            if let Some(i) = parse_leading(&line[i..]) {
                val = i * 10;
                break;
            }
        }
        for i in (0..line.len()).rev() {
            if let Some(i) = parse_leading(&line[i..]) {
                val += i;
                break;
            }
        }
        total += val;
    }
    total
}
