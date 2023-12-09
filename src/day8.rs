use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Dir {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

struct Input {
    directions: Vec<Dir>,
    map: std::collections::HashMap<String, Node>,
}

#[aoc_generator(day8)]
fn generate(input: &str) -> Input {
    let node_regex =
        regex::Regex::new(r"^([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)$").unwrap();
    let mut it = input.lines();
    let directions = it
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!("invalid direction"),
        })
        .collect();
    let map = it
        .skip(1)
        .map(|line| {
            let caps = node_regex.captures(line).unwrap();
            Node {
                id: caps[1].into(),
                left: caps[2].into(),
                right: caps[3].into(),
            }
        })
        .map(|node| (node.id.clone(), node))
        .collect();
    Input { map, directions }
}

#[aoc(day8, part1)]
fn solve_part1(input: &Input) -> i64 {
    let mut current = "AAA".to_string();
    let mut count = 0;
    for dir in input.directions.iter().cycle() {
        if current == "ZZZ" {
            break;
        }
        let node = &input.map[&current];
        current = match dir {
            Dir::Left => node.left.clone(),
            Dir::Right => node.right.clone(),
        };
        count += 1;
    }
    count
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[aoc(day8, part2)]
fn solve_part2(input: &Input) -> i64 {
    let starts = input.map.keys().filter(|id| id.ends_with("A"));
    let counts = starts
        .map(|start| {
            let mut current = start.clone();
            let mut count = 0i64;
            for dir in input.directions.iter().cycle() {
                if current.ends_with("Z") {
                    break;
                }
                let node = &input.map[&current];
                current = match dir {
                    Dir::Left => node.left.clone(),
                    Dir::Right => node.right.clone(),
                };
                count += 1;
            }
            count
        })
        .collect::<Vec<_>>();
    assert!(counts
        .iter()
        .all(|n| n % input.directions.len() as i64 == 0));
    counts.iter().fold(1, |acc, n| (acc * n) / gcd(acc, *n))
}
