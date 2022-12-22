use aoc22::*;
use std::iter;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Command {
    Walk(u32),
    Left,
    Right,
}

fn command_parser() -> impl Parser<char, Command, Error = Simple<char>> {
    num()
        .map(Command::Walk)
        .or(just('L').to(Command::Left))
        .or(just('R').to(Command::Right))
}

type Grid = Array2<char>;

fn eat_input(input: &str) -> (Grid, Vec<Command>) {
    let (grid_str, commands_str) = input.split_once("\n\n").unwrap();
    let commands = command_parser()
        .repeated()
        .padded()
        .parse(commands_str)
        .unwrap();
    let grid_chars = grid_str
        .lines()
        .map(|l| l.chars().chain(iter::repeat(' ')).take(150))
        .flat_map(|x| x);
    let grid = Array2::from_shape_vec((200, 150), grid_chars.collect()).unwrap();
    (grid, commands)
}

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
}

fn part_one(input: &str) {
    let (grid, commands) = eat_input(input);
    fn get_dir_offset(dir: isize) -> (isize, isize) {
        match dir {
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, -1),
            3 => (-1, 0),
            _ => unreachable!("invalid dir"),
        }
    }
    fn walk(pos: &mut (isize, isize), dir: isize) {
        let offset = get_dir_offset(dir);
        pos.0 += offset.0;
        pos.1 += offset.1;
    }
    fn snap_to_grid(grid: &Array2<char>, pos: &mut (isize, isize), mut dir: isize) {
        loop {
            let current_square = grid.get((pos.0 as usize, pos.1 as usize));
            // println!("current square at {pos:?}: {current_square:?}");
            match current_square {
                Some('.') => {
                    return;
                }
                Some(' ') => {
                    walk(pos, dir);
                }
                Some('#') => {
                    // this doesn't change the global dir, just this function call's dir
                    dir = (dir + 2) % 4;
                    walk(pos, dir);
                }
                None => {
                    pos.0 = (pos.0 + 200) % 200;
                    pos.1 = (pos.1 + 150) % 150;
                }
                _ => panic!("unhandled case: {current_square:?}"),
            }
        }
    }
    let mut dir = 0;
    let mut pos: (isize, isize) = (0, 0);
    snap_to_grid(&grid, &mut pos, dir);

    for c in commands {
        match c {
            Command::Left => dir = (dir + 3) % 4,
            Command::Right => dir = (dir + 1) % 4,
            Command::Walk(n) => {
                for _ in 0..n {
                    walk(&mut pos, dir);
                    snap_to_grid(&grid, &mut pos, dir);
                    // println!("pos: {pos:?}, dir: {dir:?}");
                }
            }
        }
    }
    println!("part one: {}", 1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + dir);
}
