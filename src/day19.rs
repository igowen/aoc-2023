use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Bound {
    Gt(u32),
    Lt(u32),
}

impl Bound {
    fn satisfies(&self, val: u32) -> bool {
        match self {
            Bound::Gt(b) => val > *b,
            Bound::Lt(b) => val < *b,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Attribute {
    X,
    M,
    A,
    S,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Action {
    Goto(String),
    Accept,
    Reject,
}

impl Action {
    fn goto(&self) -> Option<&str> {
        match self {
            Action::Goto(s) => Some(&s),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Rule {
    attribute: Attribute,
    bound: Bound,
    action: Action,
}

impl Rule {
    fn applies(&self, p: &Part) -> bool {
        let result = self.bound.satisfies(p.attribute(self.attribute));
        result
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn score(&self) -> u64 {
        self.x as u64 + self.m as u64 + self.a as u64 + self.s as u64
    }
    fn attribute(&self, a: Attribute) -> u32 {
        match a {
            Attribute::X => self.x,
            Attribute::M => self.m,
            Attribute::A => self.a,
            Attribute::S => self.s,
        }
    }
}

#[derive(Clone, Debug)]
struct RuleSet {
    rules: Vec<Rule>,
    default_action: Action,
}

#[derive(Default)]
struct Input {
    rules: HashMap<String, RuleSet>,
    parts: Vec<Part>,
}

#[aoc_generator(day19)]
fn generate(input: &str) -> Input {
    let ruleset_re = Regex::new(r"^(\w+)\{((?:[xmas][<>][0-9]+:\w+,)+)(\w+)\}").unwrap();
    let rule_re = Regex::new(r"([xmas])([<>])([0-9]+):(\w+)").unwrap();
    let rules = input
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            let caps = ruleset_re.captures(line).unwrap();
            let name = caps[1].to_string();
            let rules = rule_re
                .captures_iter(&caps[2])
                .map(|c| {
                    let attribute = match &c[1] {
                        "x" => Attribute::X,
                        "m" => Attribute::M,
                        "a" => Attribute::A,
                        "s" => Attribute::S,
                        other => panic!("invalid attribute {other}"),
                    };
                    let bound = match &c[2] {
                        "<" => Bound::Lt(c[3].parse().unwrap()),
                        ">" => Bound::Gt(c[3].parse().unwrap()),
                        _ => panic!(),
                    };
                    let action = match &c[4] {
                        "A" => Action::Accept,
                        "R" => Action::Reject,
                        other => Action::Goto(other.to_string()),
                    };
                    Rule {
                        attribute,
                        bound,
                        action,
                    }
                })
                .collect::<Vec<_>>();
            let default_action = match &caps[3] {
                "A" => Action::Accept,
                "R" => Action::Reject,
                other => Action::Goto(other.to_string()),
            };
            (
                name,
                RuleSet {
                    rules,
                    default_action,
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let part_re = Regex::new(r"^\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}").unwrap();
    let parts = input
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .skip(1)
        .map(|line| {
            let caps = part_re.captures(line).unwrap();
            Part {
                x: caps[1].parse().unwrap(),
                m: caps[2].parse().unwrap(),
                a: caps[3].parse().unwrap(),
                s: caps[4].parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    Input { rules, parts }
}

#[aoc(day19, part1)]
fn solve_part1(input: &Input) -> u64 {
    input
        .parts
        .iter()
        .map(|part| {
            let mut action = Action::Goto("in".to_string());
            'outer: while let Some(rule_name) = action.goto() {
                let ruleset = &input.rules[rule_name];
                for rule in &ruleset.rules {
                    if rule.applies(part) {
                        action = rule.action.clone();
                        continue 'outer;
                    }
                }

                action = ruleset.default_action.clone();
            }
            match action {
                Action::Accept => {
                    return part.score();
                }
                Action::Reject => {
                    return 0;
                }
                _ => unreachable!(),
            }
        })
        .sum::<u64>()
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct PossibleParts {
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
}

impl PossibleParts {
    fn new() -> Self {
        PossibleParts {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn include(&self, rule: &Rule) -> Option<Self> {
        let mut reduced = self.clone();
        let ta = match rule.attribute {
            Attribute::X => &mut reduced.x,
            Attribute::M => &mut reduced.m,
            Attribute::A => &mut reduced.a,
            Attribute::S => &mut reduced.s,
        };
        match rule.bound {
            Bound::Gt(min) => {
                if ta.1 < min {
                    return None;
                } else {
                    ta.0 = min + 1;
                }
            }
            Bound::Lt(max) => {
                if ta.0 > max {
                    return None;
                } else {
                    ta.1 = max - 1;
                }
            }
        }
        Some(reduced)
    }

    fn exclude(&self, rule: &Rule) -> Option<Self> {
        let mut reduced = self.clone();
        let ta = match rule.attribute {
            Attribute::X => &mut reduced.x,
            Attribute::M => &mut reduced.m,
            Attribute::A => &mut reduced.a,
            Attribute::S => &mut reduced.s,
        };
        match rule.bound {
            Bound::Gt(min) => {
                if ta.0 >= min {
                    return None;
                } else {
                    ta.1 = min;
                }
            }
            Bound::Lt(max) => {
                if ta.1 <= max {
                    return None;
                } else {
                    ta.0 = max;
                }
            }
        }
        Some(reduced)
    }

    fn count(&self) -> u64 {
        (self.x.1 - self.x.0 + 1) as u64
            * (self.m.1 - self.m.0 + 1) as u64
            * (self.a.1 - self.a.0 + 1) as u64
            * (self.s.1 - self.s.0 + 1) as u64
    }
}

#[aoc(day19, part2)]
fn solve_part2(input: &Input) -> u64 {
    let mut current = vec![("in".to_string(), PossibleParts::new())];
    let mut finals = vec![];
    'outer: while let Some((rule_name, mut pp)) = current.pop() {
        let ruleset = &input.rules[&rule_name];
        for rule in &ruleset.rules {
            if let Some(reduced) = pp.include(rule) {
                match &rule.action {
                    Action::Accept => finals.push(reduced),
                    Action::Goto(new_rule) => current.push((new_rule.clone(), reduced)),
                    Action::Reject => {}
                }
            }
            if let Some(reduced) = pp.exclude(rule) {
                pp = reduced;
            } else {
                continue 'outer;
            }
        }
        match &ruleset.default_action {
            Action::Accept => finals.push(pp),
            Action::Goto(new_rule) => current.push((new_rule.clone(), pp)),
            Action::Reject => {}
        }
    }
    finals.iter().map(|pp| pp.count()).sum::<u64>()
}
