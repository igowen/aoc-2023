use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand {
    cards: [u8; 5],
    bid: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Score {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

impl Score {
    fn rank(&self) -> u8 {
        match self {
            Score::FiveOfAKind => 7,
            Score::FourOfAKind => 6,
            Score::FullHouse => 5,
            Score::ThreeOfAKind => 4,
            Score::TwoPair => 3,
            Score::Pair => 2,
            Score::HighCard => 1,
        }
    }

    fn jokerfy(&self) -> Self {
        match self {
            Score::FiveOfAKind => Score::FiveOfAKind,
            Score::FourOfAKind => Score::FiveOfAKind,
            Score::FullHouse => Score::FourOfAKind,
            Score::ThreeOfAKind => Score::FourOfAKind,
            Score::TwoPair => Score::FullHouse,
            Score::Pair => Score::ThreeOfAKind,
            Score::HighCard => Score::Pair,
        }
    }
}

impl Hand {
    fn score(&self) -> Score {
        let mut cardmap = std::collections::HashMap::new();
        self.cards
            .iter()
            .for_each(|c| *cardmap.entry(c).or_insert(0) += 1);
        let mut counts: Vec<_> = cardmap.values().collect();
        counts.sort_by(|a, b| b.cmp(a));
        let result = match counts[0] {
            5 => Score::FiveOfAKind,
            4 => Score::FourOfAKind,
            3 => {
                if *counts[1] == 2 {
                    Score::FullHouse
                } else {
                    Score::ThreeOfAKind
                }
            }
            2 => {
                if *counts[1] == 2 {
                    Score::TwoPair
                } else {
                    Score::Pair
                }
            }
            1 => Score::HighCard,
            _ => panic!("wat"),
        };
        result
    }

    fn score_part2(&self) -> Score {
        let mut cardmap = std::collections::HashMap::new();
        self.cards
            .iter()
            .filter(|c| **c != 11)
            .for_each(|c| *cardmap.entry(c).or_insert(0) += 1);
        let jokers = self.cards.iter().filter(|c| **c == 11 as u8).count();
        if jokers == 5 {
            return Score::FiveOfAKind;
        }
        let mut counts: Vec<_> = cardmap.values().collect();
        counts.sort_by(|a, b| b.cmp(a));
        let result = {
            let mut base = match counts[0] {
                5 => Score::FiveOfAKind,
                4 => Score::FourOfAKind,
                3 => {
                    if counts.len() > 1 && *counts[1] == 2 {
                        Score::FullHouse
                    } else {
                        Score::ThreeOfAKind
                    }
                }
                2 => {
                    if counts.len() > 1 && *counts[1] == 2 {
                        Score::TwoPair
                    } else {
                        Score::Pair
                    }
                }
                1 => Score::HighCard,
                _ => panic!("wat"),
            };
            for _ in 0..jokers {
                base = base.jokerfy();
            }
            base
        };
        result
    }
}

#[aoc_generator(day7)]
fn generate(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let v = line.split(" ").collect::<Vec<_>>();
            let cs = v[0].chars().collect::<Vec<_>>();
            let mut cards = [0; 5];
            for i in 0..5 {
                cards[i] = match cs[i] {
                    '2'..='9' => 2 + cs[i] as u8 - '2' as u8,
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("invalid char: {:?}", cs[i]),
                };
            }
            let bid: u64 = v[1].parse().unwrap();
            Hand { cards, bid }
        })
        .collect()
}

#[aoc(day7, part1)]
fn solve_part1(input: &Vec<Hand>) -> u64 {
    let mut ranked_hands = input.clone();
    ranked_hands.sort_by(|a, b| {
        if a.score() == b.score() {
            a.cards.cmp(&b.cards)
        } else {
            a.score().rank().cmp(&b.score().rank())
        }
    });
    ranked_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u64 * hand.bid)
        .sum()
}

#[aoc(day7, part2)]
fn solve_part2(input: &Vec<Hand>) -> u64 {
    let mut ranked_hands = input.clone();
    ranked_hands.sort_by(|a, b| {
        if a.score_part2() == b.score_part2() {
            let mut ac = a.cards.clone();
            let mut bc = b.cards.clone();
            for i in 0..5 {
                if ac[i] == 11 {
                    ac[i] = 0;
                }
                if bc[i] == 11 {
                    bc[i] = 0;
                }
            }
            ac.cmp(&bc)
        } else {
            a.score_part2().rank().cmp(&b.score_part2().rank())
        }
    });
    ranked_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u64 * hand.bid)
        .sum()
}
