use scan_fmt::*;

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn simulate(program: &str) -> Vec<i32> {
    let mut x = 1;
    let mut out = Vec::new();
    for instruction in program.lines() {
        if instruction == "noop" {
            out.push(x);
        }
        if let Ok(n) = scan_fmt!(instruction, "addx {}", i32) {
            out.push(x);
            out.push(x);
            x += n;
        }
    }
    out.push(x);
    out
}

fn part_one(input: &str) {
    let states = simulate(input);
    let res: i32 = (19..220)
        .step_by(40)
        .map(|n| states[n] * (n + 1) as i32)
        .sum();
    println!("{res}");
}

fn part_two(input: &str) {
    let states = simulate(input);
    for cycle in 0..240 {
        let x = cycle % 40;
        if (states[cycle] - x as i32).abs() <= 1 {
            print!("#");
        } else {
            print!(" ");
        }
        if x == 39 {
            print!("\n");
        }
    }
}
