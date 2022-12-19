// PLAN:
// there's 4 robots to choose to buy
// and there's time to buy roughly 12 robots total
// for a total of 4^12 options, which is within compute range.

use aoc22::*;

type Blueprint = [[u32; 4]; 4];

fn blueprint_parser() -> impl Parser<char, Blueprint, Error = Simple<char>> {
    let header = literal("Blueprint ")
        .ignore_then(num::<u32>())
        .then_ignore(literal(": "));
    let ore = literal("Each ore robot costs ")
        .ignore_then(num())
        .then_ignore(literal(" ore. "));
    let clay = literal("Each clay robot costs ")
        .ignore_then(num())
        .then_ignore(literal(" ore. "));
    let obsidian = literal("Each obsidian robot costs ")
        .ignore_then(num())
        .then_ignore(literal(" ore and "))
        .then(num())
        .then_ignore(literal(" clay. "));
    let geode = literal("Each geode robot costs ")
        .ignore_then(num())
        .then_ignore(literal(" ore and "))
        .then(num())
        .then_ignore(literal(" obsidian."));
    header
        .ignore_then(ore)
        .then(clay)
        .then(obsidian)
        .then(geode)
        .map(|(((ore, clay), obsidian), geode)| {
            [
                [ore, 0, 0, 0],
                [clay, 0, 0, 0],
                [obsidian.0, obsidian.1, 0, 0],
                [geode.0, 0, geode.1, 0],
            ]
        })
}

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn ceil_div(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

fn evaluate_blueprint(blueprint: Blueprint, total_time: u32) -> u32 {
    fn recurse(blueprint: Blueprint, robots: [u32; 4], materials: [u32; 4], time: u32) -> u32 {
        // println!("robots: {robots:?}, materials: {materials:?}, time: {time}");
        (0..4)
            .map(|choice| {
                if robots[0] > 4 {
                    return 0;
                }
                // if robots.iter().sum::<u32>() >= MAX_ROBOTS {
                //     return 0;
                // }
                let time_taken = (0..4)
                    .map(|material| {
                        if blueprint[choice][material] == 0 {
                            return 0;
                        }
                        if materials[material] >= blueprint[choice][material] {
                            return 0;
                        }
                        if robots[material] == 0 {
                            return u32::MAX - 100;
                        }
                        ceil_div(
                            blueprint[choice][material] - materials[material],
                            robots[material],
                        )
                    })
                    .max()
                    .unwrap();
                if (time_taken + 1) > time {
                    return 0;
                }
                let new_resources: [u32; 4] = [0, 1, 2, 3].map(|material| {
                    robots[material] * (time_taken + 1) + materials[material]
                        - blueprint[choice][material]
                });
                let mut new_robots = robots;
                new_robots[choice] += 1;

                recurse(
                    blueprint,
                    new_robots,
                    new_resources,
                    time - (time_taken + 1),
                )
            })
            .max()
            .unwrap()
            .max({
                // or we just don't buy a thing
                materials[3] + robots[3] * time
            })
    }

    let res = recurse(blueprint, [1, 0, 0, 0], [0, 0, 0, 0], total_time);
    // println!("blueprint: {blueprint:?}");
    res
}

fn part_one(input: &str) {
    let blueprints: Vec<Blueprint> = blueprint_parser().padded().repeated().parse(input).unwrap();
    let res: u32 = blueprints
        .par_iter() // TODO: make it par_iter
        .map(|blueprint| evaluate_blueprint(*blueprint, 24))
        .zip(1..100)
        .map(|(value, id)| {
            println!("value: {value}, id: {id}");
            value * id
        })
        .sum();

    println!("{res}");
}

fn part_two(input: &str) {
    let blueprints: Vec<Blueprint> = blueprint_parser().padded().repeated().parse(input).unwrap();
    let res: u32 = blueprints[0..3]
        .par_iter()
        .map(|blueprint| evaluate_blueprint(*blueprint, 32))
        .map(|value| {
            println!("value: {value}");
            value
        })
        .product();

    println!("{res}");
}
