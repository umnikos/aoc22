use aoc22::*;

fn parse_line() -> impl Parser<char, ((i64, i64), (i64, i64)), Error = Simple<char>> {
    let sensor = literal("Sensor at x=")
        .ignore_then(int())
        .then_ignore(literal(", y="))
        .then(int());
    let beacon = literal("closest beacon is at x=")
        .ignore_then(int())
        .then_ignore(literal(", y="))
        .then(int());
    sensor.then_ignore(literal(": ")).then(beacon)
}

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    const MY_Y: i64 = 2000000;
    let pairs = parse_line().padded().repeated().parse(input).unwrap();

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    for &(sensor, beacon) in &pairs {
        let range = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        let distance = (sensor.1 - MY_Y).abs();
        if distance > range {
            continue;
        }
        let left = sensor.0 - range + distance;
        let right = sensor.0 + range - distance;
        ranges.push((left, right));
    }

    ranges.sort_by_key(|&(start, _end)| start);

    let mut count = 0;
    let mut last_end = i64::MIN;
    for mut myrange in ranges.clone() {
        myrange.0 = myrange.0.max(last_end + 1);
        if myrange.1 < myrange.0 {
            continue;
        }
        last_end = myrange.1;
        count += myrange.1 - myrange.0 + 1;

        // we want "places where a beacon cannot be"
        // and not places where a *distress* beacon cannot be,
        // so we have to subtract the beacons.
        count -= pairs
            .iter()
            .filter(|&(_, beacon)| {
                beacon.1 == MY_Y && (myrange.0 <= beacon.0 && beacon.0 <= myrange.1)
            })
            .map(|(_, beacon)| beacon)
            .dedup()
            .count() as i64;
    }

    println!("{count}");
}

fn part_two(input: &str) {
    let pairs = parse_line().padded().repeated().parse(input).unwrap();

    let sensor_ranges: Vec<((i64, i64), i64)> = pairs
        .iter()
        .map(|&(sensor, beacon)| {
            (
                sensor,
                (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs(),
            )
        })
        .collect();

    // candidates are points just outside a sensor's range
    let evaluate_candidate = |(x, y): (i64, i64)| {
        if x < 0 || y < 0 || x > 4000000 || y > 4000000 {
            return false;
        }
        for &(sensor, range) in &sensor_ranges {
            let dist = (sensor.0 - x).abs() + (sensor.1 - y).abs();
            if dist <= range {
                return false;
            }
        }
        true
    };

    let found: Vec<(i64, i64)> = pairs
        .par_iter()
        .filter_map(|&(sensor, beacon)| {
            let range = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
            let dist = range + 1;
            for x in (sensor.0 - dist)..=(sensor.0 + dist) {
                let slack = dist - (sensor.0 - x).abs();
                let highy = sensor.1 + slack;
                let lowy = sensor.1 - slack;
                if evaluate_candidate((x, highy)) {
                    return Some((x, highy));
                }
                if evaluate_candidate((x, lowy)) {
                    return Some((x, lowy));
                }
            }
            None
        })
        .collect();

    println!("points found: {}", found.iter().dedup().count());
    let point = found[0];
    println!("{:?} with freq of {}", point, point.0 * 4000000 + point.1);
}
