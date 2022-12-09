use ndarray::*;

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn parse(input: &str) -> Array2<u8> {
    let arr: Vec<u8> = input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c as u8 - b'0')
        .collect();
    Array2::from_shape_vec((99, 99), arr).unwrap()
}

fn rotate<T>(a: Array2<T>) -> Array2<T> {
    let mut transposed = a.reversed_axes();
    transposed.invert_axis(Axis(0));
    transposed
}

fn part_one(input: &str) {
    let mut arr = parse(input);
    let mut visible = Array2::from_elem((99, 99), false);
    for _ in 0..4 {
        visible.index_axis_mut(Axis(0), 0).map_mut(|x| *x = true);
        for column in 0..99 {
            let trees = arr.index_axis(Axis(1), column);
            let mut visible_column = visible.index_axis_mut(Axis(1), column);
            let mut max = trees[0];
            for i in 1..99 {
                if trees[i] > max {
                    visible_column[i] = true;
                    max = trees[i];
                }
            }
        }

        arr = rotate(arr);
        visible = rotate(visible);
    }
    let res = visible.iter().filter(|x| **x).count();
    println!("{res}");
}

fn scenic_score(arr: &Array2<u8>, pos: (i32, i32), dir: (i32, i32)) -> i32 {
    let mut pos = pos;
    let house = arr[(pos.0 as usize, pos.1 as usize)];
    let mut score = 0;
    // let mut max = 0;
    loop {
        pos.0 += dir.0;
        pos.1 += dir.1;
        if pos.0 < 0 || pos.1 < 0 {
            break;
        }
        let attempt = arr.get((pos.0 as usize, pos.1 as usize));
        match attempt {
            Some(&tree) => {
                // if tree <= max {
                //     continue;
                // }
                score += 1;
                if tree >= house {
                    break;
                }
                // max = tree;
            }
            None => {
                break;
            }
        }
    }
    score
}

fn part_two(input: &str) {
    let arr = parse(input);
    let mut max = 0;
    for x in 1..98 {
        for y in 1..98 {
            let score: i32 = [(0, 1), (1, 0), (-1, 0), (0, -1)]
                .iter()
                .map(|&dir| scenic_score(&arr, (x, y), dir))
                .product();
            max = max.max(score);
        }
    }
    println!("{max}");
}
