fn from_snafu(num: &str) -> i64 {
    let mut res = 0;
    for c in num.chars() {
        res *= 5;
        res += match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("unknown digit: {c}"),
        }
    }
    res
}

fn to_snafu(num: i64) -> String {
    if num == 0 {
        return "".to_string();
    }
    let m = num % 5;
    let lastdigit = match m {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '=',
        4 => '-',
        _ => unreachable!(),
    };
    format!("{}{}", to_snafu((num + 2) / 5), lastdigit)
}

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
}

fn part_one(input: &str) {
    let s = input.lines().map(|l| from_snafu(l)).sum::<i64>();
    println!("the sum: {s}");
    println!("part one: {}", to_snafu(s));
}
