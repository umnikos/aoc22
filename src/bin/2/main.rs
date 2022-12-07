fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn simulate_one(game: &str) -> i32 {
    match game {
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
        _ => unreachable!(),
    }
}

fn simulate_two(game: &str) -> i32 {
    match game {
        "A X" => 0 + 3,
        "A Y" => 3 + 1,
        "A Z" => 6 + 2,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 0 + 2,
        "C Y" => 3 + 3,
        "C Z" => 6 + 1,
        _ => unreachable!(),
    }
}

fn part_one(input: &str) {
    let res = input.lines().map(simulate_one).sum::<i32>();
    println!("{res}");
}

fn part_two(input: &str) {
    let res = input.lines().map(simulate_two).sum::<i32>();
    println!("{res}");
}
