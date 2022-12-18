use aoc22::*;
use std::collections::{HashSet, VecDeque};

type Point = (usize, usize, usize);

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn parse_point() -> impl Parser<char, Point, Error = Simple<char>> {
    num()
        .then_ignore(just(','))
        .then(num())
        .then_ignore(just(','))
        .then(num())
        .map(|((a, b), c)| (a, b, c))
}

fn surface_area(points: &Vec<Point>) -> i32 {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut total = 0;
    for &point in points {
        visited.insert(point);
        total += 6;
        for i in [1, -1] {
            for offset in [(i, 0, 0), (0, i, 0), (0, 0, i)] {
                let Some(neighbour) = offset_coords_3d(point, offset) else {continue};
                if visited.contains(&neighbour) {
                    total -= 2;
                }
            }
        }
    }
    total
}

fn part_one(input: &str) {
    let points = parse_point()
        .padded()
        .repeated()
        .then_ignore(end())
        .parse(input)
        .unwrap();
    let total = surface_area(&points);
    println!("{total}");
}

fn part_two(input: &str) {
    let points = parse_point()
        .padded()
        .repeated()
        .then_ignore(end())
        .parse(input)
        .unwrap();
    let shape: HashSet<Point> = HashSet::from_iter(points.into_iter());
    let m = shape
        .iter()
        .map(|&(a, b, c)| a.max(b).max(c))
        .max()
        .unwrap();
    let is_in_bounds = |n: &Point| n.0.max(n.1).max(n.2) <= m + 1;

    let mut unvisited: VecDeque<Point> = VecDeque::from([(0, 0, 0)]);
    let mut considered: HashSet<Point> = HashSet::from([(0, 0, 0)]);
    let mut total: i32 = 0;
    while !unvisited.is_empty() {
        let point = unvisited.pop_front().unwrap();
        for i in [1, -1] {
            for offset in [(i, 0, 0), (0, i, 0), (0, 0, i)] {
                let Some(neighbour) = offset_coords_3d(point, offset) else {continue;};
                if shape.contains(&neighbour) {
                    total += 1;
                } else if !considered.contains(&neighbour) && is_in_bounds(&neighbour) {
                    unvisited.push_back(neighbour);
                    considered.insert(neighbour);
                }
            }
        }
    }
    println!("{total}");
}
