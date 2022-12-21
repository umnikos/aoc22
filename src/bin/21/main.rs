use aoc22::*;
use std::collections::HashMap;

type Name = String;
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Operator {
    Mul,
    Div,
    Add,
    Sub,
}
#[derive(Eq, PartialEq, Clone, Debug)]
enum Job {
    Yell(i64),
    Math(Name, Operator, Name),
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Monkey {
    name: Name,
    job: Job,
}

fn monkey_parser() -> impl Parser<char, Monkey, Error = Simple<char>> {
    fn name() -> impl Parser<char, Name, Error = Simple<char>> {
        filter(char::is_ascii_alphabetic)
            .repeated()
            .exactly(4)
            .collect()
    }
    fn operator() -> impl Parser<char, Operator, Error = Simple<char>> {
        literal(" + ")
            .to(Operator::Add)
            .or(literal(" - ").to(Operator::Sub))
            .or(literal(" * ").to(Operator::Mul))
            .or(literal(" / ").to(Operator::Div))
    }
    fn job() -> impl Parser<char, Job, Error = Simple<char>> {
        num().map(|x| Job::Yell(x)).or(name()
            .then(operator())
            .then(name())
            .map(|((a, op), b)| Job::Math(a, op, b)))
    }
    name()
        .then_ignore(literal(": "))
        .then(job())
        .map(|(n, j)| Monkey { name: n, job: j })
}

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn simulate_monkey(monkey_map: &HashMap<Name, Monkey>, monkey_name: &str) -> i64 {
    let my_monkey = monkey_map.get(monkey_name).unwrap();
    match &my_monkey.job {
        Job::Yell(n) => *n,
        Job::Math(a, op, b) => {
            let aa = simulate_monkey(monkey_map, a);
            let bb = simulate_monkey(monkey_map, b);
            match op {
                Operator::Add => aa + bb,
                Operator::Sub => aa - bb,
                Operator::Mul => aa * bb,
                Operator::Div => aa / bb,
            }
        }
    }
}

fn part_one(input: &str) {
    let monkeys: Vec<Monkey> = monkey_parser()
        .padded()
        .repeated()
        .then_ignore(end())
        .parse(input)
        .unwrap();
    let monkey_map: HashMap<Name, Monkey> =
        monkeys.into_iter().map(|m| (m.name.clone(), m)).collect();
    let res = simulate_monkey(&monkey_map, "root");
    println!("part one: {res}");
}

fn part_two(input: &str) {
    let monkeys: Vec<Monkey> = monkey_parser()
        .padded()
        .repeated()
        .then_ignore(end())
        .parse(input)
        .unwrap();
    let mut monkey_map: HashMap<Name, Monkey> =
        monkeys.into_iter().map(|m| (m.name.clone(), m)).collect();
    monkey_map.entry(String::from("root")).and_modify(|m| {
        match &mut m.job {
            Job::Math(_, op, _) => {
                *op = Operator::Sub;
            }
            _ => unreachable!("root has weird job"),
        };
    });
    fn try_number(monkey_map: &mut HashMap<Name, Monkey>, n: i64) -> i64 {
        monkey_map.entry(String::from("humn")).and_modify(|m| {
            match &mut m.job {
                Job::Yell(num) => {
                    *num = n;
                }
                _ => unreachable!("human has weird job"),
            };
        });
        simulate_monkey(&monkey_map, "root")
    }
    let mut guess = 0;
    loop {
        let real = try_number(&mut monkey_map, guess);
        println!("{guess} -> {real}");
        if real == 0 {
            break;
        }
        for delta in (2..).step_by(2) {
            let epsilon = try_number(&mut monkey_map, guess + delta) - real;
            if epsilon != 0 {
                guess -= real * delta / epsilon;
                break;
            }
        }
    }
    println!("part two: {guess}");
}
