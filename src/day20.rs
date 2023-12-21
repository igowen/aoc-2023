use crate::gcd;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Signal {
    High,
    Low,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum State {
    On,
    Off,
}

impl Default for State {
    fn default() -> Self {
        State::Off
    }
}

impl State {
    fn flip(&mut self) {
        *self = match self {
            State::On => State::Off,
            State::Off => State::On,
        }
    }

    fn signal(&self) -> Signal {
        match self {
            State::On => Signal::High,
            State::Off => Signal::Low,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Kind {
    Broadcaster,
    FlipFlop(State),
    Conjunction(HashMap<String, Signal>),
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Module {
    name: String,
    kind: Kind,
    outputs: Vec<String>,
}

#[aoc_generator(day20)]
fn generate(input: &str) -> HashMap<String, Module> {
    let mut result = input
        .lines()
        .map(|line| {
            let mut splits = line.split(" -> ");
            let name_kind = splits.next().unwrap();
            let connections = splits.next().unwrap();
            let kind = match name_kind.chars().next().unwrap() {
                '&' => Kind::Conjunction(Default::default()),
                '%' => Kind::FlipFlop(Default::default()),
                'b' => Kind::Broadcaster,
                other => panic!("unexpected module type: {other}"),
            };

            let name = if kind == Kind::Broadcaster {
                "broadcaster".to_string()
            } else {
                name_kind.chars().skip(1).collect()
            };
            let outputs = connections
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            (
                name.clone(),
                Module {
                    name,
                    kind,
                    outputs,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let mut conjunction_inputs = result
        .values()
        .filter_map(|m| {
            if let Kind::Conjunction(_) = m.kind {
                Some((m.name.clone(), vec![]))
            } else {
                None
            }
        })
        .collect::<HashMap<String, Vec<String>>>();

    for (name, module) in &mut result {
        for output in &module.outputs {
            if let Some(v) = conjunction_inputs.get_mut(output) {
                v.push(name.clone());
            }
        }
    }

    for (name, inputs) in conjunction_inputs {
        if let Some(Module {
            kind: Kind::Conjunction(h),
            ..
        }) = result.get_mut(&name)
        {
            h.extend(inputs.iter().map(|s| (s.clone(), Signal::Low)));
        }
    }

    result
}

// Run one button press and return the number of low and high signals that were observed, respectively.
fn run(system: &mut HashMap<String, Module>, to_track: &mut [(String, bool)]) -> (usize, usize) {
    let mut to_process = VecDeque::new();
    let mut highs = 0;
    let mut lows = 1;
    let bcast_outputs = &system["broadcaster"].outputs;
    lows += bcast_outputs.len();
    to_process.extend(
        bcast_outputs
            .iter()
            .map(|o| (o.clone(), "broadcaster".to_string(), Signal::Low)),
    );
    while let Some((name, from, signal)) = to_process.pop_front() {
        if signal == Signal::Low {
            for (tracked_name, seen) in to_track.iter_mut() {
                if name == *tracked_name {
                    *seen = true;
                }
            }
        }
        if let Some(module) = system.get_mut(&name) {
            match &mut module.kind {
                Kind::FlipFlop(ref mut state) => {
                    if signal == Signal::Low {
                        state.flip();
                        to_process.extend(
                            module
                                .outputs
                                .iter()
                                .map(|o| (o.clone(), name.clone(), state.signal())),
                        );
                        if state.signal() == Signal::High {
                            highs += module.outputs.len();
                        } else {
                            lows += module.outputs.len();
                        }
                    }
                }
                Kind::Conjunction(h) => {
                    *h.get_mut(&from).unwrap() = signal;
                    if h.values().any(|s| s == &Signal::Low) {
                        to_process.extend(
                            module
                                .outputs
                                .iter()
                                .map(|o| (o.clone(), name.clone(), Signal::High)),
                        );
                        highs += module.outputs.len();
                    } else {
                        to_process.extend(
                            module
                                .outputs
                                .iter()
                                .map(|o| (o.clone(), name.clone(), Signal::Low)),
                        );
                        lows += module.outputs.len();
                    }
                }
                _ => panic!(),
            }
        }
    }
    (lows, highs)
}

#[aoc(day20, part1)]
fn solve_part1(input: &HashMap<String, Module>) -> usize {
    let mut system = input.clone();

    let mut lows = 0;
    let mut highs = 0;
    for _ in 0..1000 {
        let (dl, dh) = run(&mut system, &mut []);
        lows += dl;
        highs += dh;
    }
    highs * lows
}

#[aoc(day20, part2)]
fn solve_part2(input: &HashMap<String, Module>) -> usize {
    let mut system = input.clone();

    let collector = input
        .values()
        .find_map(|m| {
            if m.outputs.contains(&"rx".to_string()) {
                Some(m.name.clone())
            } else {
                None
            }
        })
        .unwrap();
    let mut feeders = input
        .values()
        .filter_map(|m| {
            if m.outputs.contains(&collector) {
                Some((m.name.clone(), false))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let len = feeders.len();

    let mut loop_lengths = HashMap::<String, usize>::new();

    let mut i = 0;
    while loop_lengths.len() < len {
        run(&mut system, &mut feeders[..]);

        i += 1;
        for (name, seen) in &feeders {
            if *seen {
                loop_lengths.entry(name.clone()).or_insert(i);
            }
        }
    }

    let product = loop_lengths.values().product::<usize>();
    product / loop_lengths.values().fold(product, |acc, n| gcd(acc, *n))
}
