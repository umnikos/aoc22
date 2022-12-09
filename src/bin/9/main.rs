use scan_fmt::*;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn parse(input: &str) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    for line in input.lines() {
        let (dir, count) = scan_fmt!(line, "{} {}", char, u32).unwrap();
        let dir = match dir {
            'U' => (0, 1),
            'D' => (0, -1),
            'R' => (1, 0),
            'L' => (-1, 0),
            _ => unreachable!("unrecognized direction: {dir}"),
        };
        for _ in 0..count {
            res.push(dir);
        }
    }
    res
}

fn simulate(dirs: Vec<(i32, i32)>, knots: u32) -> HashSet<(i32, i32)> {
    let mut visited = HashSet::new();
    let mut moves: Vec<(i32, i32)> = Vec::new();
    let mut head: (i32, i32) = (0, 0);
    let mut tail = head;
    visited.insert(tail);
    for dir in dirs {
        head.0 += dir.0;
        head.1 += dir.1;
        if (head.0 - tail.0).abs() == 2 || (head.1 - tail.1).abs() == 2 {
            let previous_tail = tail;
            if head.0 - previous_tail.0 > 0 {
                tail.0 += 1;
            }
            if head.1 - previous_tail.1 > 0 {
                tail.1 += 1;
            }
            if head.0 - previous_tail.0 < 0 {
                tail.0 -= 1;
            }
            if head.1 - previous_tail.1 < 0 {
                tail.1 -= 1;
            }
            let new_move = (tail.0 - previous_tail.0, tail.1 - previous_tail.1);
            moves.push(new_move);
        }
        visited.insert(tail);
    }
    if knots == 2 {
        visited
    } else {
        simulate(moves, knots - 1)
    }
}

fn part_one(input: &str) {
    let set = simulate(parse(input), 2);
    println!("{}", set.len());
}

fn part_two(input: &str) {
    let set = simulate(parse(input), 10);
    println!("{}", set.len());
}
