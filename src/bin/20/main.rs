use aoc22::*;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn mix(mut nums: VecDeque<i64>, repetitions: i64) -> i64 {
    let l = nums.len() as i64;
    // make 0 the first number
    let zero_place = nums.iter().position(|x| x == &0).unwrap();
    for _ in 0..zero_place {
        let val = nums.pop_front().unwrap();
        nums.push_back(val);
    }
    // from now on 0 is our point of reference and will remain static

    // a map from a position to the number in that position
    let mut positions: Vec<i64> = (0..l).collect();
    let q = l - zero_place as i64;
    for _repetition in 0..repetitions {
        for i in (q..l).chain(1..q) {
            let mut n = nums[i as usize];
            let mut npos = positions.iter().position(|x| x == &i).unwrap();
            // normalize
            n = n.rem_euclid(2 * (l - 1));
            while n + npos as i64 >= l {
                n = n - l + 1;
            }
            while n + npos as i64 <= 0 {
                n = n + l - 1;
            }

            // and sliiide
            while n != 0 {
                if n < 0 {
                    positions.swap(npos, npos - 1);
                    npos -= 1;
                    n += 1;
                } else {
                    positions.swap(npos, npos + 1);
                    npos += 1;
                    n -= 1;
                }
            }
        }
    }

    [1000, 2000, 3000]
        .into_iter()
        .map(|i| nums[positions[i] as usize])
        .sum()
}

fn part_one(input: &str) {
    // like a vec, but also a dequeue!
    let nums: VecDeque<i64> = int()
        .padded()
        .repeated()
        .parse(input)
        .unwrap()
        .into_iter()
        .collect();
    let res = mix(nums, 1);
    println!("{res}");
}

const DECRYPT_CONST: i64 = 811589153;
fn part_two(input: &str) {
    let nums: VecDeque<i64> = int()
        .padded()
        .repeated()
        .parse(input)
        .unwrap()
        .into_iter()
        .map(|n: i64| n * DECRYPT_CONST)
        .collect();
    let res = mix(nums, 10);
    println!("{res}");
}
