use itertools::*;

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn totals(input: &str) -> Vec<i32> {
    input
        .lines()
        .group_by(|&line| line != "")
        .into_iter()
        .step_by(2)
        .map(|(_, group)| group.map(|l| l.parse::<i32>().unwrap()).sum())
        .collect()
}

fn part_one(input: &str) {
    println!("{}", max(totals(input)).unwrap());
}

fn part_two(input: &str) {
    let mut rankings = totals(input);
    rankings.sort_by_key(|n| -n);
    println!("{}", rankings.iter().take(3).sum::<i32>());
}
