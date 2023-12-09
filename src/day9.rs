use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn generate(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn extrapolate(sequence: Vec<i64>) -> i64 {
    let mut subseqs: Vec<Vec<i64>> = vec![sequence.clone()];
    loop {
        subseqs.push(
            subseqs
                .last()
                .unwrap()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect(),
        );
        if subseqs.last().unwrap().iter().all(|n| *n == 0) {
            break;
        }
    }
    for i in (0..(subseqs.len() - 1)).rev() {
        let e = subseqs[i].last().unwrap() + subseqs[i + 1].last().unwrap();
        subseqs[i].push(e);
    }
    *subseqs[0].last().unwrap()
}

#[aoc(day9, part1)]
fn solve_part1(input: &Vec<Vec<i64>>) -> i64 {
    input.iter().map(|seq| extrapolate(seq.clone())).sum()
}

#[aoc(day9, part2)]
fn solve_part2(input: &Vec<Vec<i64>>) -> i64 {
    input
        .iter()
        .map(|seq| extrapolate(seq.iter().cloned().rev().collect()))
        .sum()
}
