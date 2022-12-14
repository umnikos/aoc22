use aoc22::*;
use std::collections::VecDeque;

struct Monkey {
    op: Box<dyn Fn(u64) -> u64>,
    throw: Box<dyn Fn(u64) -> usize>,
}

fn monkey_parser() -> impl Parser<char, (Monkey, Vec<u64>), Error = Simple<char>> {
    #[derive(Clone, Copy, Debug)]
    enum Atom {
        Var,
        Lit(u64),
    }
    fn atom() -> impl Parser<char, Atom, Error = Simple<char>> {
        let var = literal("old").to(Atom::Var);
        let atom = var.or(num().map(Atom::Lit as fn(u64) -> Atom));
        atom
    }
    let op = just('+').or(just('-')).or(just('*'));
    let exp = literal("new = ")
        .ignore_then(atom())
        .then(op.padded())
        .then(atom());

    literal("Monkey")
        .padded()
        .ignore_then(text::int(10))
        .ignore_then(just(':').padded())
        .ignore_then(literal("Starting items:").padded())
        .ignore_then(int_array())
        .then_ignore(literal("Operation:").padded())
        .then(exp.padded())
        .then_ignore(literal("Test: divisible by").padded())
        .then(num::<u64>())
        .then_ignore(literal("If true: throw to monkey").padded())
        .then(num::<u64>())
        .then_ignore(literal("If false: throw to monkey").padded())
        .then(num::<u64>())
        .map(move |((((i, ((l, o), r)), m), a), b)| {
            let mon = Monkey {
                op: Box::from(move |n| {
                    let x = match l {
                        Atom::Var => n,
                        Atom::Lit(q) => q,
                    };
                    let y = match r {
                        Atom::Var => n,
                        Atom::Lit(q) => q,
                    };
                    match o {
                        '+' => x + y,
                        '-' => x - y,
                        '*' => x * y,
                        _ => unreachable!("unknown operator"),
                    }
                }),
                throw: Box::from(move |n| if n % m == 0 { a as usize } else { b as usize }),
            };
            let items = i;
            (mon, items)
        })
}

fn int_array() -> impl Parser<char, Vec<u64>, Error = Simple<char>> {
    num().separated_by(literal(", "))
}

fn main() {
    let input = include_str!("input.txt");
    if let Err(m) = monkey_parser().parse(input) {
        println!("{m:?}");
    }
    let mut monkeys = Vec::new();
    let mut items = Vec::new();
    let res: Vec<(Monkey, Vec<u64>)> = monkey_parser().repeated().parse(input).unwrap();
    for (monkey, thing) in res.into_iter() {
        monkeys.push(monkey);
        items.push(VecDeque::from(thing));
    }

    part_one(&monkeys, items.clone());
    part_two(&monkeys, items.clone());
}

fn part_one(monkeys: &[Monkey], mut items: Vec<VecDeque<u64>>) {
    let mut rankings = [0; 8];
    for _round in 0..20 {
        for monkey in 0..8 {
            let op = &monkeys[monkey].op;
            let throw = &monkeys[monkey].throw;
            let mut old_queue = VecDeque::new();
            std::mem::swap(&mut items[monkey], &mut old_queue);
            for item in old_queue.into_iter() {
                rankings[monkey] += 1;
                let new_value = op(item) / 3;
                items[throw(new_value)].push_back(new_value);
            }
        }
    }
    rankings.sort();
    println!("{}", rankings[7] * rankings[6]);
}

fn part_two(monkeys: &[Monkey], mut items: Vec<VecDeque<u64>>) {
    let mut rankings: [u64; 8] = [0; 8];
    for _round in 0..10_000 {
        for monkey in 0..8 {
            let op = &monkeys[monkey].op;
            let throw = &monkeys[monkey].throw;
            let mut old_queue = VecDeque::new();
            std::mem::swap(&mut items[monkey], &mut old_queue);
            for item in old_queue.into_iter() {
                rankings[monkey] += 1;
                let new_value = op(item) % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19);

                items[throw(new_value)].push_back(new_value);
            }
        }
    }
    rankings.sort();
    println!("{}", rankings[7] * rankings[6]);
}
