// STRATEGY:
// there are 15 valves that actually matter.
// the whole game is that you move to a valve, open it, then repeat
// start with 0 points and 30 remaining time
// you pay time and then increment points with remaining time times the valve's score

// we simulate the game non-deterministically
// with the optimization being that at a given time t and a given point p there's only 2^15 ways to reach it
// (only possible difference between two routes being how many points they rack up in the meantime, pick the bigger one)
// which is way less than the 15 factorial combinations you'd simulate otherwise
// there's at most 30 time slots to simulate, with 15 points, so that's 30*15*2^15 time and space
// which is well within the realm of computability

use aoc22::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

type Pressure = u64;
type Time = u32;
type Label = usize;
type LabelSet = u16; // exactly enough
const LABEL_AA: Label = 0;

fn parse_line() -> impl Parser<char, (Label, Pressure, Vec<Label>), Error = Simple<char>> {
    let label = || {
        filter(char::is_ascii_uppercase)
            .repeated()
            .exactly(2)
            .map(|v| (v[0] as Label - b'A' as Label) * 26 + (v[1] as Label - b'A' as Label))
    };
    literal("Valve ")
        .ignore_then(label())
        .then_ignore(literal(" has flow rate="))
        .then(num())
        .then_ignore(
            literal("; ")
                .then(literal("tunnels lead to valves ").or(literal("tunnel leads to valve "))),
        )
        .then(label().separated_by(literal(", ")))
        .map(|((l, f), v)| (l, f, v))
}

// for dijkstra
#[derive(PartialEq, Eq)]
struct Candidate {
    time: Time,
    label: Label,
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .time
            .cmp(&self.time)
            .then_with(|| self.label.cmp(&other.label))
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn eat_input(
    input: &str,
) -> (
    Vec<(Label, Pressure, Vec<Label>)>,
    HashMap<(Label, Label), Time>,
) {
    let things = parse_line()
        .padded()
        .repeated()
        .then_ignore(end())
        .parse(input)
        .expect("parse error");
    // store walking distance from every label (at least the ones that matter) to every other label
    let mut graph: HashMap<(Label, Label), Time> = HashMap::new();
    for (label, pressure, _) in things.iter().cloned() {
        if label != LABEL_AA && pressure == 0 {
            continue;
        }
        // we begin bfs
        let mut unexplored: Vec<Candidate> = Vec::from([Candidate { time: 0, label }]);
        let mut considered: HashSet<Label> = HashSet::from([label]);
        while !unexplored.is_empty() {
            let mut newly_discovered = Vec::new();
            for Candidate {
                time,
                label: current_label,
            } in unexplored
            {
                // update hashmap (unnecessarily complicated since I thought we were doing dijkstra)
                let my_entry = graph.entry((label, current_label)).or_insert(Time::MAX);
                *my_entry = time.min(*my_entry);
                // add to unexplored if not considered
                for (candidate_label, _candidate_pressure, candidate_neighbours) in things.iter() {
                    if considered.contains(candidate_label) {
                        continue;
                    }
                    if !candidate_neighbours.contains(&current_label) {
                        continue;
                    }
                    newly_discovered.push(Candidate {
                        time: time + 1,
                        label: *candidate_label,
                    });
                    considered.insert(*candidate_label);
                }
            }
            unexplored = newly_discovered;
        }
    }

    (things, graph)
}

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    let (things, graph) = eat_input(input);
    let mut enumeration: [LabelSet; 1 << 16] = [0; 1 << 16];
    let mut i = 0;
    let mut interesting_labels: Vec<(Label, Pressure)> = Vec::new();
    for (label, pressure, _) in things.iter().cloned() {
        if pressure != 0 || label == LABEL_AA {
            enumeration[label] = 1 << i;
            interesting_labels.push((label, pressure));
            i += 1;
        }
    }
    let mut timeline: [HashMap<(Label, LabelSet), Pressure>; 31] = [(); 31].map(|_| HashMap::new());
    timeline[0].insert((LABEL_AA, enumeration[LABEL_AA]), 0);
    let mut max = 0;
    for t in 0_u32..30 {
        for (&(label, set), &pressure) in timeline[t as usize].clone().iter() {
            for (possibly_next, possibly_next_pressure) in interesting_labels.iter().cloned() {
                if enumeration[possibly_next] & set != 0 {
                    continue;
                }
                let walking_time = graph[&(label, possibly_next)];
                let future_time = t + walking_time + 1;
                if future_time > 30 {
                    continue;
                }
                let my_entry = timeline[future_time as usize]
                    .entry((possibly_next, set | enumeration[possibly_next]))
                    .or_insert(0);
                *my_entry = (pressure + possibly_next_pressure * (30 - future_time as Pressure))
                    .max(*my_entry);
                max = max.max(*my_entry);
            }
        }
    }
    println!("{max}");
}

fn part_two(input: &str) {}
