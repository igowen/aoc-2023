use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
struct Race {
    time: u64,
    distance: u64,
}

#[aoc_generator(day6)]
fn generate(input: &str) -> (Vec<Race>, Race) {
    let lines = input.lines().collect::<Vec<_>>();
    let times = lines[0]
        .split(" ")
        .filter(|s| s.len() > 0)
        .skip(1)
        .map(|s| s.parse().unwrap());
    let distances = lines[1]
        .split(" ")
        .filter(|s| s.len() > 0)
        .skip(1)
        .map(|s| s.parse().unwrap());

    (
        times
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect(),
        Race {
            time: lines[0]
                .split(":")
                .skip(1)
                .map(|s| {
                    s.chars()
                        .filter(|c| *c >= '0' && *c <= '9')
                        .collect::<String>()
                })
                .collect::<String>()
                .parse()
                .unwrap(),
            distance: lines[1]
                .split(":")
                .skip(1)
                .map(|s| {
                    s.chars()
                        .filter(|c| *c >= '0' && *c <= '9')
                        .collect::<String>()
                })
                .collect::<String>()
                .parse()
                .unwrap(),
        },
    )
}

/// ```
/// # use aoc_2023::day6::distance;
/// assert!(distance(4, 7) == 12);
/// ```
pub fn distance(hold: u64, total: u64) -> u64 {
    hold * (total - hold)
}

#[aoc(day6, part1)]
fn solve_part1((input, _): &(Vec<Race>, Race)) -> i64 {
    input
        .iter()
        .map(|race| {
            (0..race.time)
                .filter(|i| distance(*i, race.time) > race.distance)
                .count() as i64
        })
        .product()
}

#[aoc(day6, part2)]
fn solve_part2((_, race): &(Vec<Race>, Race)) -> i64 {
    (0..race.time)
        .filter(|i| distance(*i, race.time) > race.distance)
        .count() as i64
}
