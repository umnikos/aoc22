use itertools::*;
use scan_fmt::*;

fn main() {
    let input = include_str!("input.txt");
    let (stacks_str, commands) = input.split("\n\n").collect_tuple().unwrap();
    let stacks: Vec<Vec<char>> = stacks_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    part_one(stacks.clone(), commands);
    part_two(stacks.clone(), commands);
}

fn stacks_move(stacks: &mut [Vec<char>], count: i32, from: usize, to: usize) {
    let mut intermediate: Vec<char> = Vec::new();
    for _ in 0..count {
        intermediate.push(stacks[from - 1].pop().unwrap());
    }
    for _ in 0..count {
        stacks[to - 1].push(intermediate.pop().unwrap());
    }
}

fn part_one(stacks: Vec<Vec<char>>, commands: &str) {
    let mut stacks = stacks;
    for command in commands.lines() {
        let (count, from, to) =
            scan_fmt!(command, "move {} from {} to {}", i32, usize, usize).unwrap();
        for _ in 0..count {
            stacks_move(&mut stacks, 1, from, to);
        }
    }
    let res: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    println!("{res}");
}

fn part_two(stacks: Vec<Vec<char>>, commands: &str) {
    let mut stacks = stacks;
    for command in commands.lines() {
        let (count, from, to) =
            scan_fmt!(command, "move {} from {} to {}", i32, usize, usize).unwrap();
        stacks_move(&mut stacks, count, from, to);
    }
    let res: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    println!("{res}");
}
