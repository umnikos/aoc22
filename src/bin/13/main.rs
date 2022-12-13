use self::Item::{Num, Packet};
use chumsky::prelude::*;
use itertools::*;
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Item {
    Num(u64),
    Packet(Vec<Box<Item>>),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Num(a) => match other {
                Num(b) => a.cmp(b),
                Packet(_b) => {
                    let newleft = Packet(Vec::from([Box::new(self.clone())]));
                    newleft.cmp(other)
                }
            },
            Packet(a) => match other {
                Num(_b) => {
                    let newright = Packet(Vec::from([Box::new(other.clone())]));
                    self.cmp(&newright)
                }
                Packet(b) => {
                    let n = a.len().min(b.len());
                    for i in 0..n {
                        let r = a[i].cmp(&b[i]);
                        if r != Equal {
                            return r;
                        }
                    }
                    a.len().cmp(&b.len())
                }
            },
        }
    }
}

fn num() -> impl Parser<char, u64, Error = Simple<char>> {
    text::int::<char, Simple<char>>(10).from_str().unwrapped()
}

fn packet_parser() -> impl Parser<char, Item, Error = Simple<char>> {
    recursive(|item| {
        let list = item
            .separated_by(just(','))
            .delimited_by(just('['), just(']'))
            .map(|v| Item::Packet(v.into_iter().map(Box::new).collect()));
        num().map(Item::Num).or(list)
    })
}

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    let mut res = 0;
    let Ok(packets) = packet_parser().padded().repeated().parse(input) else {panic!()};
    let mut index = 1;
    for (a, b) in packets.into_iter().tuples() {
        if a < b {
            res += index;
        }
        index += 1;
    }
    println!("{res}");
}

fn part_two(input: &str) {
    let Ok(mut packets) = packet_parser().padded().repeated().parse(input) else {panic!()};
    let left_separator = packet_parser().parse("[[2]]").unwrap();
    let right_separator = packet_parser().parse("[[6]]").unwrap();
    packets.push(left_separator.clone());
    packets.push(right_separator.clone());
    packets.sort();
    let mut li = 0;
    let mut ri = 0;
    for (i, p) in packets.into_iter().enumerate() {
        if p == left_separator {
            li = i + 1;
        }
        if p == right_separator {
            ri = i + 1;
        }
    }
    println!("{}", li * ri);
}
