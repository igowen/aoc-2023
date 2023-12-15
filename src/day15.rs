use aoc_runner_derive::{aoc, aoc_generator};

fn hash(s: &str) -> u8 {
    s.chars().fold(0, |acc, c| ((acc + c as i64) * 17) % 256) as u8
}

#[aoc(day15, part1)]
fn solve_part1(input: &str) -> i64 {
    input.split(",").map(|step| hash(step) as i64).sum()
}

#[aoc(day15, part2)]
fn solve_part2(input: &str) -> i64 {
    let mut boxes: Vec<Vec<(String, u8)>> = vec![vec![]; 256];
    for step in input.split(",") {
        if step.contains('=') {
            let v = step.split("=").collect::<Vec<_>>();
            let label = v[0];
            let box_num = hash(label);
            let power: u8 = v[1].parse().unwrap();
            let b = &mut boxes[box_num as usize];
            if let Some((_, existing)) = b.iter_mut().find(|(s, _)| s == label) {
                *existing = power;
            } else {
                b.push((label.to_string(), power));
            }
        } else {
            let label = step.strip_suffix("-").unwrap();
            let h = hash(label);
            if let Some((i, _)) = boxes[h as usize]
                .iter()
                .enumerate()
                .find(|(_, (existing, _))| existing == label)
            {
                boxes[h as usize].remove(i);
            }
        }
    }
    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, (_, power))| (i as i64 + 1) * (j as i64 + 1) * *power as i64)
                .sum::<i64>()
        })
        .sum()
}
