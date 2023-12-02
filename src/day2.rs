use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Default, Debug)]
struct Draw {
    r: u32,
    g: u32,
    b: u32,
}

#[derive(Default, Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

#[aoc_generator(day2)]
fn generate(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let mut game = Game::default();
            let line = line.split(": ").collect::<Vec<_>>();
            game.id = line[0].split(" ").last().unwrap().parse().unwrap();
            for draw in line[1].split("; ") {
                let mut d = Draw::default();
                for c in draw.split(", ") {
                    let cube = c.split(" ").collect::<Vec<_>>();
                    let count: u32 = cube[0].parse().unwrap();
                    match cube[1] {
                        "red" => d.r = count,
                        "green" => d.g = count,
                        "blue" => d.b = count,
                        _ => (),
                    }
                }
                game.draws.push(d);
            }
            game
        })
        .collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &Vec<Game>) -> u32 {
    input
        .iter()
        .filter_map(|game| {
            if game
                .draws
                .iter()
                .filter(|draw| draw.r > 12 || draw.g > 13 || draw.b > 14)
                .count()
                == 0
            {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day2, part2)]
fn solve_part2(input: &Vec<Game>) -> u32 {
    input
        .iter()
        .map(|game| {
            let max_r = game.draws.iter().map(|d| d.r).max().unwrap();
            let max_g = game.draws.iter().map(|d| d.g).max().unwrap();
            let max_b = game.draws.iter().map(|d| d.b).max().unwrap();
            max_r * max_g * max_b
        })
        .sum()
}
