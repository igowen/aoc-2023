use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Card {
    numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

#[aoc_generator(day4)]
fn generate(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let line = line
                .split(": ")
                .last()
                .unwrap()
                .split("|")
                .collect::<Vec<_>>();
            Card {
                winning_numbers: line[0]
                    .split(" ")
                    .filter(|s| s.len() > 0)
                    .map(|s| s.parse())
                    .collect::<Result<_, _>>()
                    .unwrap(),
                numbers: line[1]
                    .split(" ")
                    .filter(|s| s.len() > 0)
                    .map(|s| s.parse())
                    .collect::<Result<_, _>>()
                    .unwrap(),
            }
        })
        .collect()
}

use std::collections::BTreeSet;

#[aoc(day4, part1)]
fn solve_part1(input: &Vec<Card>) -> u32 {
    input
        .iter()
        .map(|card| {
            let numbers: BTreeSet<u32> = card.numbers.iter().cloned().collect();
            let winning_numbers: BTreeSet<u32> = card.winning_numbers.iter().cloned().collect();
            let hits = numbers.intersection(&winning_numbers).count();
            if hits > 0 {
                2u32.pow(hits as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
fn solve_part2(input: &Vec<Card>) -> u32 {
    let hits = input
        .iter()
        .map(|card| {
            let numbers: BTreeSet<u32> = card.numbers.iter().cloned().collect();
            let winning_numbers: BTreeSet<u32> = card.winning_numbers.iter().cloned().collect();
            let hits = numbers.intersection(&winning_numbers).count();
            hits
        })
        .collect::<Vec<_>>();
    let mut instances = vec![1; hits.len()];
    for i in 0..hits.len() {
        for j in 0..hits[i] {
            if i + j + 1 >= hits.len() {
                break;
            }
            instances[i + j + 1] += instances[i];
        }
    }
    instances.into_iter().map(|i| i as u32).sum()
}
