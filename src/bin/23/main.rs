use aoc22::*;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

type Board = HashSet<(isize, isize)>;

fn parse_input(input: &str) -> Board {
    HashSet::from_iter(
        input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars().enumerate().map(move |(x, c)| {
                    if c == '#' {
                        Some((y as isize, x as isize))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .flatten(),
    )
}

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn add_tuple(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 + b.0, a.1 + b.1)
}

// I might've mixed these up but it doesn't actually matter
fn rotate_left((x, y): (isize, isize)) -> (isize, isize) {
    (-y, x)
}
fn rotate_right((x, y): (isize, isize)) -> (isize, isize) {
    (y, -x)
}

fn simulate_rounds(mut board: Board, rounds: u32) -> (Board, u32) {
    let mut dirs = VecDeque::from([(-1, 0), (1, 0), (0, -1), (0, 1)]);
    for round in 0..rounds {
        let mut proposed: HashMap<(isize, isize), u32> = HashMap::new();
        let mut anyone_moved: bool = false;
        'proposing: for &elf in board.iter() {
            let (a, b, c, d) = dirs
                .iter()
                .map(|&d| {
                    let s = add_tuple(elf, d);
                    let l = add_tuple(s, rotate_left(d));
                    let r = add_tuple(s, rotate_right(d));
                    !(board.contains(&s) || board.contains(&l) || board.contains(&r))
                })
                .collect_tuple()
                .unwrap();
            if a && b && c && d {
                // yay we're alone
                continue 'proposing;
            }
            for (i, &x) in [a, b, c, d].iter().enumerate() {
                if x {
                    let entry = proposed.entry(add_tuple(elf, dirs[i])).or_insert(0);
                    *entry += 1;
                    continue 'proposing;
                }
            }
            // too crowded to propose a thing
        }

        let mut new_board: Board = HashSet::new();
        'moving: for &elf in board.iter() {
            let (a, b, c, d) = dirs
                .iter()
                .map(|&d| {
                    let s = add_tuple(elf, d);
                    let l = add_tuple(s, rotate_left(d));
                    let r = add_tuple(s, rotate_right(d));
                    !(board.contains(&s) || board.contains(&l) || board.contains(&r))
                })
                .collect_tuple()
                .unwrap();
            if a && b && c && d {
                // yay we're alone
                new_board.insert(elf);
                continue 'moving;
            }
            for (i, &x) in [a, b, c, d].iter().enumerate() {
                if x {
                    let entry = proposed.entry(add_tuple(elf, dirs[i])).or_insert(0);
                    assert_ne!(*entry, 0);
                    if *entry == 1 {
                        new_board.insert(add_tuple(elf, dirs[i]));
                        anyone_moved = true;
                    } else {
                        new_board.insert(elf);
                    }
                    // we stop trying to move even if this was a dud because this was our proposed direction
                    continue 'moving;
                }
            }
            // we're crowded but we don't cease to exist
            new_board.insert(elf);
        }

        board = new_board;

        if !anyone_moved {
            return (board, round);
        }

        let to_be_last = dirs.pop_front().unwrap();
        dirs.push_back(to_be_last);
    }

    (board, rounds)
}

fn part_one(input: &str) {
    let mut board = parse_input(input);
    (board, _) = simulate_rounds(board, 10);
    let minx = board.iter().map(|(_y, x)| x).min().unwrap();
    let miny = board.iter().map(|(y, _x)| y).min().unwrap();
    let maxx = board.iter().map(|(_y, x)| x).max().unwrap();
    let maxy = board.iter().map(|(y, _x)| y).max().unwrap();
    let area = (maxx - minx + 1) * (maxy - miny + 1);
    let res = area - board.len() as isize;
    println!("part one: {res}");
}

fn part_two(input: &str) {
    let board = parse_input(input);
    let (_, rounds) = simulate_rounds(board, u32::MAX);
    println!("part two: {}", rounds + 1);
}
