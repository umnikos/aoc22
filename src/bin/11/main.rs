use std::collections::VecDeque;

struct Monkey {
    op: fn(u64) -> u64,
    throw: fn(u64) -> usize,
}

fn main() {
    // TODO: get this info from a frikkin parser
    let monkeys = vec![
        Monkey {
            op: (|n| n * 5),
            throw: (|n| if n % 11 == 0 { 2 } else { 3 }),
        },
        Monkey {
            op: (|n| n * 11),
            throw: (|n| if n % 5 == 0 { 4 } else { 0 }),
        },
        Monkey {
            op: (|n| n + 2),
            throw: (|n| if n % 19 == 0 { 5 } else { 6 }),
        },
        Monkey {
            op: (|n| n + 5),
            throw: (|n| if n % 13 == 0 { 2 } else { 6 }),
        },
        Monkey {
            op: (|n| n * n),
            throw: (|n| if n % 7 == 0 { 0 } else { 3 }),
        },
        Monkey {
            op: (|n| n + 4),
            throw: (|n| if n % 17 == 0 { 7 } else { 1 }),
        },
        Monkey {
            op: (|n| n + 6),
            throw: (|n| if n % 2 == 0 { 7 } else { 5 }),
        },
        Monkey {
            op: (|n| n + 7),
            throw: (|n| if n % 3 == 0 { 4 } else { 1 }),
        },
    ];
    let items = vec![
        VecDeque::from([83, 88, 96, 79, 86, 88, 70]),
        VecDeque::from([59, 63, 98, 85, 68, 72]),
        VecDeque::from([90, 79, 97, 52, 90, 94, 71, 70]),
        VecDeque::from([97, 55, 62]),
        VecDeque::from([74, 54, 94, 76]),
        VecDeque::from([58]),
        VecDeque::from([66, 63]),
        VecDeque::from([56, 56, 90, 96, 68]),
    ];

    part_one(&monkeys, items.clone());
    part_two(&monkeys, items.clone());
}

fn part_one(monkeys: &[Monkey], items: Vec<VecDeque<u64>>) {
    let mut items = items;
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

fn part_two(monkeys: &[Monkey], items: Vec<VecDeque<u64>>) {
    let mut items = items;
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
