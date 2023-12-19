use crate::Dir;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Instruction {
    dir: Dir,
    distance: u32,
}

#[derive(Clone, Debug)]
struct Input {
    part1_instructions: Vec<Instruction>,
    part2_instructions: Vec<Instruction>,
}

#[aoc_generator(day18)]
fn generate(input: &str) -> Input {
    let re = Regex::new(r"^([UDLR]) (\d+) \(#([0-9a-f]{5})([0-9a-f])\)$").unwrap();
    let (part1_instructions, part2_instructions): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let dir = match &caps[1] {
                "U" => Dir::N,
                "D" => Dir::S,
                "L" => Dir::W,
                "R" => Dir::E,
                other => panic!("invalid direction {}", other),
            };
            let distance = caps[2].parse::<u32>().unwrap();
            let part2_distance = u32::from_str_radix(&caps[3], 16).unwrap();
            let part2_dir = match &caps[4] {
                "0" => Dir::E,
                "1" => Dir::S,
                "2" => Dir::W,
                "3" => Dir::N,
                other => panic!("invalid hex direction {}", other),
            };
            (
                Instruction { dir, distance },
                Instruction {
                    dir: part2_dir,
                    distance: part2_distance,
                },
            )
        })
        .unzip();

    Input {
        part1_instructions,
        part2_instructions,
    }
}

fn solve(input: &Vec<Instruction>) -> i64 {
    let mut y = 0;
    let mut total = 0;
    for i in input {
        match i.dir {
            Dir::N => {
                y -= i.distance as i64;
            }
            Dir::S => {
                y += i.distance as i64;
            }
            Dir::E => {
                total += y * i.distance as i64;
            }
            Dir::W => {
                total -= y * i.distance as i64;
            }
        }
    }
    total.abs() + input.iter().map(|v| v.distance as i64).sum::<i64>() / 2 + 1
}

#[aoc(day18, part1)]
fn solve_part1(input: &Input) -> i64 {
    solve(&input.part1_instructions)
}

#[aoc(day18, part2)]
fn solve_part2(input: &Input) -> i64 {
    solve(&input.part2_instructions)
}
