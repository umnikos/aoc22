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

fn main() {}
