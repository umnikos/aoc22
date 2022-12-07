use rayon::prelude::*;
use scan_fmt::*;

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn parse(line: &str) -> (i32, i32, i32, i32) {
    scan_fmt!(line, "{}-{},{}-{}", i32, i32, i32, i32).unwrap()
}

fn part_one(input: &str) {
    let res = input
        .lines()
        .par_bridge()
        .filter(|line| {
            let (al, ar, bl, br) = parse(line);
            (bl <= al && ar <= br) || (al <= bl && br <= ar)
        })
        .count();
    println!("{res}");
}

fn part_two(input: &str) {
    let res = input
        .lines()
        .par_bridge()
        .filter(|line| {
            let (al, ar, bl, br) = parse(line);
            (bl <= al && al <= br) || (al <= bl && bl <= ar)
        })
        .count();
    println!("{res}");
}
