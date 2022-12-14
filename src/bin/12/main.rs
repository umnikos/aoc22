use aoc22::*;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn parse(input: &str) -> Array2<char> {
    Array2::from_shape_vec(
        (41, 64),
        input.lines().collect::<Vec<_>>().concat().chars().collect(),
    )
    .unwrap()
}

fn elevation(c: char) -> i32 {
    match c {
        'S' => 1,
        'E' => 26,
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        _ => unreachable!("invalid character in match: {c}"),
    }
}

fn pathfind(arr: &Array2<char>, unexplored: VecDeque<(usize, usize)>) -> Option<i32> {
    let mut unexplored = unexplored;
    let mut explored: HashSet<(usize, usize)> = HashSet::new();
    for step in 0.. {
        let mut found: VecDeque<(usize, usize)> = VecDeque::new();
        if unexplored.is_empty() {
            // no route found.
            return None;
        }
        // println!("exploring {:?}", unexplored);
        for &location in unexplored.iter() {
            if arr[location] == 'E' {
                // found the end!
                return Some(step);
            }
            explored.insert(location);
            'offsetting: for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let Some(newspot) = offset_coords(location,dir) else {continue 'offsetting;};
                if explored.contains(&newspot) {
                    continue 'offsetting;
                }
                if found.contains(&newspot) {
                    continue 'offsetting;
                }
                let Some(&newelevation) = arr.get(newspot) else {continue 'offsetting;};
                if elevation(newelevation) - elevation(arr[location]) > 1 {
                    continue 'offsetting;
                }
                found.push_back(newspot);
            }
        }

        unexplored = found;
    }
    unreachable!();
}

fn part_one(input: &str) {
    let arr = parse(input);
    for y in 0..41 {
        for x in 0..64 {
            if arr[(y, x)] == 'S' {
                let start = (y, x);
                let res = pathfind(&arr, VecDeque::from([start]));
                println!("{res:?}");
                return;
            }
        }
    }
}

fn part_two(input: &str) {
    let arr = parse(input);
    let mut starts = VecDeque::new();
    for y in 0..41 {
        for x in 0..64 {
            if elevation(arr[(y, x)]) == 1 {
                let start = (y, x);
                starts.push_back(start);
            }
        }
    }
    let min = pathfind(&arr, starts);
    println!("{min:?}");
}
