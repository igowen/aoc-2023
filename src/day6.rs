use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
struct Race {
    time: i64,
    distance: i64,
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

pub fn roots(time: i64, record: i64) -> (i64, i64) {
    let s = ((time * time - 4 * record) as f64).sqrt() / 2.0;
    let halftime = time as f64 / 2.0;
    ((halftime - s).ceil() as i64, (halftime + s).floor() as i64)
}

#[aoc(day6, part1)]
fn solve_part1((input, _): &(Vec<Race>, Race)) -> i64 {
    input
        .iter()
        .map(|race| {
            let (min, max) = roots(race.time, race.distance);
            max - min + 1
        })
        .product()
}

#[aoc(day6, part2)]
fn solve_part2((_, race): &(Vec<Race>, Race)) -> i64 {
    let (min, max) = roots(race.time, race.distance);
    max - min + 1
}
