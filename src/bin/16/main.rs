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
use std::collections::{BinaryHeap, HashMap, HashSet};

type Pressure = u64;
type Time = u32;
type Label = u16;
const LABEL_AA: Label = 0;

fn parse_line() -> impl Parser<char, (Label, Pressure, Vec<Label>), Error = Simple<char>> {
    let label = || {
        filter(char::is_ascii_uppercase)
            .repeated()
            .exactly(2)
            .map(|v| (v[0] as u16 - b'A' as u16) * 26 + (v[1] as u16 - b'A' as u16))
    };
    literal("Valve ")
        .ignore_then(label())
        .then_ignore(literal(" has flow rate="))
        .then(num())
        .then_ignore(literal("; tunnels lead to valves "))
        .then(num().separated_by(literal(", ")))
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

fn eat_input(input: &str) {
    let things = parse_line()
        .repeated()
        .then_ignore(end())
        .parse(input)
        .unwrap();
    // store walking distance from every label to every other label (at least the ones that matter)
    let mut graph: HashMap<(Label, Label), Time> = HashMap::new();
    for (label, pressure, _) in things.iter().cloned() {
        if label != LABEL_AA && pressure == 0 {
            continue;
        }
        // we begin dijkstra
        let mut candidates: BinaryHeap<Candidate> =
            BinaryHeap::from([Candidate { time: 0, label }]);
        while let Some(Candidate {
            time,
            label: current_label,
        }) = candidates.pop()
        {
            // update hashmap
            let my_entry = graph.entry((label, current_label)).or_insert(Time::MAX);
            *my_entry = time.min(*my_entry);

            // add to candidates (avoiding looping forever)
            for (candidate_label, candidate_pressure, )
        }
    }

    // TODO - return something
}

fn main() {}
