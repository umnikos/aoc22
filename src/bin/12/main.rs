use ndarray::prelude::*;
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

fn offset_coords(coords: (usize, usize), offset: (isize, isize)) -> Option<(usize, usize)> {
    let a = coords.0 as isize + offset.0;
    let b = coords.1 as isize + offset.1;
    if a < 0 || b < 0 {
        None
    } else {
        Some((a as usize, b as usize))
    }
}

fn elevation(c: char) -> i32 {
    match c {
        'S' => 1,
        'E' => 26,
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        _ => unreachable!("invalid character in match: {c}"),
    }
}

fn pathfind(arr: &Array2<char>, start: (usize, usize)) -> Option<i32> {
    let mut unexplored: VecDeque<(usize, usize)> = VecDeque::new();
    unexplored.push_back(start);
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
                let res = pathfind(&arr, start);
                println!("{res:?}");
                return;
            }
        }
    }
}

fn part_two(input: &str) {
    let arr = parse(input);
    let mut min = 99999;
    for y in 0..41 {
        for x in 0..64 {
            if elevation(arr[(y, x)]) == 1 {
                let Some(res) = pathfind(&arr, (y,x)) else {continue;};
                min = min.min(res);
            }
        }
    }
    println!("{min}");
}
