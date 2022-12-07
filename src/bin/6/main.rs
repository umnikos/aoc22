fn main() {
    let input = include_str!("input.txt");
    println!("{}", both_parts(input, 4));
    println!("{}", both_parts(input, 14));
}

fn to_bitvec(c: char) -> u32 {
    match c {
        'a'..='z' => 1 << (c as u32 - 'a' as u32),
        _ => unreachable!("unexpected character '{c}'"),
    }
}

fn both_parts(input: &str, len: usize) -> usize {
    let stream: Vec<u32> = input.chars().map(to_bitvec).collect();
    let mut acc = 0;
    for i in 0..(len - 1) {
        acc ^= stream[i];
    }
    for i in (len - 1)..stream.len() {
        acc ^= stream[i];
        if acc.count_ones() == len as u32 {
            return i + 1;
        }
        acc ^= stream[i - len + 1];
    }
    unreachable!("no beginning found");
}
