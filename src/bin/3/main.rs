use itertools::*;

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn priority(item: char) -> i32 {
    match item {
        'a'..='z' => item as i32 - 'a' as i32 + 1,
        'A'..='Z' => item as i32 - 'A' as i32 + 27,
        _ => unreachable!(),
    }
}

fn part_one(input: &str) {
    let total = input
        .lines()
        .map(|backpack| {
            let (front, back) = backpack.split_at(backpack.len() / 2);
            for item in front.chars() {
                if back.find(item).is_some() {
                    return priority(item);
                }
            }
            unreachable!();
        })
        .sum::<i32>();
    println!("{total}");
}

fn part_two(input: &str) {
    let total = input
        .lines()
        .tuples()
        .into_iter()
        .map(|(first, second, third)| {
            for item in first.chars() {
                if second.find(item).is_some() && third.find(item).is_some() {
                    return priority(item);
                }
            }
            unreachable!();
        })
        .sum::<i32>();
    println!("{total}");
}
