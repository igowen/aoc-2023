use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
struct MapRange {
    source: i64,
    dest: i64,
    len: i64,
}

#[derive(Clone, Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Vec<MapRange>>,
}

#[aoc_generator(day5)]
fn generate(input: &str) -> Almanac {
    let lines = input.lines().filter(|s| s.len() > 0).collect::<Vec<_>>();
    let seeds = lines[0]["seeds: ".len()..]
        .split(" ")
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let maps = lines[1..]
        .split(|line| line.contains("map:"))
        .skip(1)
        .map(|chunk| {
            chunk
                .iter()
                .map(|line| {
                    let mut it = line.split(" ").map(|s| s.parse().unwrap());
                    MapRange {
                        dest: it.next().unwrap(),
                        source: it.next().unwrap(),
                        len: it.next().unwrap(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Almanac { seeds, maps }
}

#[aoc(day5, part1)]
fn solve_part1(input: &Almanac) -> i64 {
    let Almanac { seeds, maps } = input;
    seeds
        .iter()
        .map(|seed| {
            let mut cur = *seed;
            for map in maps {
                for range in map {
                    if cur >= range.source && cur < range.source + range.len {
                        cur = range.dest + (cur - range.source);
                        break;
                    }
                }
            }
            cur
        })
        .min()
        .unwrap()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
struct Offset {
    start: i64,
    end: i64,
    diff: i64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
struct Range {
    start: i64,
    end: i64,
}

#[aoc(day5, part2)]
fn solve_part2(input: &Almanac) -> i64 {
    let Almanac { seeds, maps } = input;

    let mut seeds = seeds
        .chunks(2)
        .map(|v| Range {
            start: v[0],
            end: v[0] + v[1],
        })
        .collect::<Vec<_>>();

    let mut next = vec![];

    let maps = maps
        .into_iter()
        .map(|map| {
            map.into_iter()
                .map(|m| Offset {
                    start: m.source,
                    end: m.source + m.len,
                    diff: m.dest - m.source,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for map in maps {
        let mut map = map.clone();
        seeds.sort_by_key(|s| s.start);
        map.sort_by_key(|m| m.start);
        let mut map_it = map.into_iter().peekable();
        for seed in &seeds {
            let mut seed = seed.clone();
            loop {
                match map_it.peek_mut() {
                    Some(m) => {
                        if seed.start < m.start && seed.end < m.start {
                            // No map for this range -- pass through unchanged.
                            next.push(seed);
                            break;
                        } else if seed.start < m.start {
                            // We need to split this into an unmapped range and a mapped range.
                            next.push(Range {
                                start: seed.start,
                                end: m.start,
                            });
                            // Fast forward so we match the start of the map range -- the second
                            // part of the split will be handled by the branch immediately follwing
                            // this one (on the next iteration).
                            seed.start = m.start;
                        } else if seed.start == m.start {
                            if seed.end <= m.end {
                                // The full input range is mapped through this map, so we're done.
                                next.push(Range {
                                    start: seed.start + m.diff,
                                    end: seed.end + m.diff,
                                });
                                break;
                            } else {
                                // Otherwise, we need to split the range.
                                next.push(Range {
                                    start: seed.start + m.diff,
                                    end: m.end + m.diff,
                                });
                                // Again, the second part of the split will be handled by the next
                                // iteration of the loop.
                                seed.start = m.end;
                            }
                        } else if seed.start < m.end {
                            // We have an unused chunk of the map, so we can drop the unused
                            // portion and hite the `seed.start == m.start` case on the next
                            // iteration of the loop.
                            *m = Offset {
                                start: seed.start,
                                end: m.end,
                                diff: m.diff,
                            };
                        } else {
                            // Nothing maps through this one -- skip.
                            map_it.next();
                        }
                    }
                    None => {
                        // If we're out of maps to look at, pass the input through unchanged.
                        next.push(seed);
                        break;
                    }
                }
            }
        }
        std::mem::swap(&mut seeds, &mut next);
        next.clear();
    }

    seeds.iter().map(|s| s.start).min().unwrap()
}
