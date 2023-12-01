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
    return total;
}

fn parse_leading<S: AsRef<str>>(s: S) -> Option<u32> {
    if let Some(i) = s.as_ref().chars().next()?.to_digit(10) {
        return Some(i);
    }
    if s.as_ref().starts_with("one") {
        return Some(1);
    }
    if s.as_ref().starts_with("two") {
        return Some(2);
    }
    if s.as_ref().starts_with("three") {
        return Some(3);
    }
    if s.as_ref().starts_with("four") {
        return Some(4);
    }
    if s.as_ref().starts_with("five") {
        return Some(5);
    }
    if s.as_ref().starts_with("six") {
        return Some(6);
    }
    if s.as_ref().starts_with("seven") {
        return Some(7);
    }
    if s.as_ref().starts_with("eight") {
        return Some(8);
    }
    if s.as_ref().starts_with("nine") {
        return Some(9);
    }
    None
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
    return total;
}
