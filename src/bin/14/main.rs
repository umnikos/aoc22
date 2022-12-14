use chumsky::prelude::*;
use itertools::Itertools;
use ndarray::prelude::*;

fn num() -> impl Parser<char, usize, Error = Simple<char>> {
    text::int::<char, Simple<char>>(10).from_str().unwrapped()
}

fn literal(s: &str) -> impl Parser<char, (), Error = Simple<char>> {
    let s = String::from(s);
    any().repeated().at_most(s.len()).try_map(move |ss, span| {
        if ss.into_iter().collect::<String>() == s {
            Ok(())
        } else {
            Err(Simple::expected_input_found(span, None, None))
        }
    })
}

fn shape_parser() -> impl Parser<char, Vec<(usize, usize)>, Error = Simple<char>> {
    let tuple = num().then_ignore(just(',')).then(num());
    tuple.separated_by(literal("->").padded()).at_least(1)
}

fn offset_coords(coords: (usize, usize), offset: (isize, isize)) -> Option<(usize, usize)> {
    let a = coords.0 as isize + offset.0;
    let b = coords.1 as isize + offset.1;
    if a < 0 || b < 0 {
        None
    } else {
        Some((a as usize, b as usize))
    }
}

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn build_grid(input: &str) -> Array2<u8> {
    let mut grid = Array2::zeros((1000, 200));
    let shapes = shape_parser().padded().repeated().parse(input).unwrap();
    for shape in shapes {
        let mut current = shape[0];
        grid[current] = 1;
        for &new in shape[1..].iter() {
            let (first, second) = if new.0 > current.0 || new.1 > current.1 {
                (current, new)
            } else {
                (new, current)
            };
            let mut window = grid.slice_mut(s![first.0..=second.0, first.1..=second.1]);
            window.fill(1);
            current = new;
        }
    }
    grid
}

fn simulate_sand(mut grid: Array2<u8>) -> u32 {
    let mut backtrack: Vec<(usize, usize)> = Vec::new();
    let mut solidified = 0;
    let mut current = (500, 0);
    'simulating: while current.1 != 199 {
        for offset in [(0, 1), (-1, 1), (1, 1)] {
            let belowme = offset_coords(current, offset).unwrap();
            if grid[belowme] == 0 {
                backtrack.push(current);
                current = belowme;
                continue 'simulating;
            }
        }
        solidified += 1;
        grid[current] = 1;
        match backtrack.pop() {
            Some(above) => {
                current = above;
            }
            None => {
                return solidified;
            }
        }
    }

    solidified
}

fn part_one(input: &str) {
    let grid = build_grid(input);
    println!("{}", simulate_sand(grid));
}

fn part_two(input: &str) {
    let mut grid = build_grid(input);
    let maxy = (0..1000)
        .cartesian_product(0..200)
        .filter(|&c| grid[c] == 1)
        .map(|(_, y)| y)
        .max()
        .expect("nothing in the grid???");
    let mut floor = grid.slice_mut(s![.., maxy + 2]);
    floor.fill(1);
    println!("{}", simulate_sand(grid));
}
