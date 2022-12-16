use std::fmt::Debug;

use ahash::{AHashMap, AHashSet};
use tinyvec::{tiny_vec, ArrayVec, TinyVec};

use crate::utils::fast_parse_int;

use super::*;

pub struct Day;

type ValveInfo = AHashMap<ValveName, (usize, Vec<ValveName>)>;
type ValveInfoWeighted = AHashMap<ValveName, (usize, TinyVec<[(ValveName, usize); 16]>)>;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 16;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input: ValveInfo = parse_input(input);
        let input = optimize_input(&input);

        recursive_search(&input, Path::new(), 0, 0, &tiny_vec!())
    }
}

fn recursive_search(
    input: &ValveInfoWeighted,
    path: Path,
    max_score_so_far: usize,
    current_minute: usize,
    nodes_since_last_valve_turn: &TinyVec<[ValveName; 20]>,
) -> usize {
    // println!("Current minute: {current_minute}");
    let current_valve = path.route.last().unwrap();
    let (pressure, targets) = &input[current_valve];

    let current_score = path.calculate_score(input);

    if current_minute >= 30 {
        // println!("Exit because time limit is reached ({current_score}): {path:?}");
        return current_score;
    }

    // early exit if all valves have been opened
    // TODO: currently assumes that `AA` always has pressure=0
    if path.opened_valves.len() == input.len() - 1 {
        // println!("we've opened everything already, early exit");
        return current_score;
    }

    // check if this branch has the potential to be faster
    if max_score_so_far != 0 {
        let potential = path.potential_score(input, current_minute);
        // println!("Potential/max: {potential}/{max_score_so_far}");
        if potential <= max_score_so_far {
            // we can't win with this line
            // println!("Exiting early because potential is not high enough!");
            return current_score;
        }
    }

    let mut max = current_score;

    if *pressure != 0
        && !path
            .opened_valves
            .iter()
            .rev()
            .position(|(n, _)| n == current_valve)
            .is_some()
    {
        // open a valve. insert an "open" event in the next minute
        let mut cloned_path = path.clone();
        cloned_path
            .opened_valves
            .push((*current_valve, current_minute + 1));
        let new_score = recursive_search(input, cloned_path, max, current_minute + 1, &tiny_vec!());
        max = max.max(new_score);
    }

    // try movements, where we don't a valve
    // not turning a valve means that turning back is pointless, which is why a blocklist is kept.
    let mut nodes_since_last_valve_turn = nodes_since_last_valve_turn.clone();
    nodes_since_last_valve_turn.push(*current_valve);

    // let mut targets_sorted = targets.clone();
    // targets_sorted.sort_unstable_by(|a, b| (a.1.cmp(&b.1).reverse()));
    for (possible_target, target_cost) in targets {
        // as long as we don't turn on a valve, don't re-visit the same nodes.
        if nodes_since_last_valve_turn.contains(possible_target) {
            continue;
        }

        let mut cloned_path = path.clone();
        cloned_path.route.push(*possible_target);
        debug_assert_ne!(
            *target_cost, 0,
            "target cost should never be zero for moving somewhere"
        );
        let new_score = recursive_search(
            input,
            cloned_path,
            max,
            current_minute + target_cost,
            &nodes_since_last_valve_turn,
        );
        max = max.max(new_score);
    }

    max
}

/// Optimizes the input by folding valves with flow rates of 0. It ensures that `AA` is always kept.
fn optimize_input(input: &ValveInfo) -> ValveInfoWeighted {
    let mut response = ValveInfoWeighted::new();

    for (valve_name, (pressure, targets)) in input {
        if *pressure != 0 || *valve_name == ValveName::from_bytes(b"AA") {
            let mut set = AHashSet::new();
            set.insert(*valve_name);
            let new_targets = resolve_targets(set, targets, input, 0);
            response.insert(*valve_name, (*pressure, new_targets));
        }
    }

    response
}

fn resolve_targets(
    visited: AHashSet<ValveName>,
    targets: &[ValveName],
    valve_info: &ValveInfo,
    weight_offset: usize,
) -> TinyVec<[(ValveName, usize); 16]> {
    let mut resolved_targets = tiny_vec![];
    for target in targets {
        if visited.contains(target) {
            continue;
        }

        let (target_pressure, target_targets) = &valve_info[target];

        if *target_pressure == 0 {
            // "fold" this target by recursively finding its targets until it finds one with a non-zero pressure
            let mut new_visited = visited.clone();
            new_visited.insert(*target);

            for (nested_resolved_target, weight) in
                resolve_targets(new_visited, target_targets, valve_info, weight_offset + 1)
            {
                debug_assert!(!visited.contains(&nested_resolved_target));
                resolved_targets.push((nested_resolved_target, weight));
            }
        } else {
            // just return this
            debug_assert!(!visited.contains(target));
            resolved_targets.push((*target, weight_offset + 1));
        }
    }

    resolved_targets
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
struct ValveName(usize);

impl ValveName {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(((bytes[1] - b'A') as usize) * 26 + (bytes[0] - b'A') as usize)
    }
}

impl ToString for ValveName {
    fn to_string(&self) -> String {
        String::from_utf8(vec![
            ((self.0 / 26) as u8 + b'A'),
            ((self.0 % 26) as u8 + b'A'),
        ])
        .unwrap()
        .to_string()
    }
}

impl Debug for ValveName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ValveName").field(&self.to_string()).finish()
    }
}

#[derive(Clone, Debug)]
struct Path {
    opened_valves: ArrayVec<[(ValveName, usize); 15]>, // can only open 15 valves at most in 30s
    route: ArrayVec<[ValveName; 30]>,
}

impl Path {
    pub fn new() -> Self {
        let aa = ValveName::from_bytes(b"AA");
        let mut route = ArrayVec::new();
        route.push(aa);
        Self {
            opened_valves: ArrayVec::default(),
            route,
        }
    }

    pub fn calculate_score(&self, input: &ValveInfoWeighted) -> usize {
        self.opened_valves
            .iter()
            .filter(|(_, start_time)| *start_time <= 30)
            .map(|(v, start_time)| input[v].0 * (30 - start_time))
            .sum()
    }

    // NOTE: the more accurate this is (ie. lower), the more branches can be trimmed
    pub fn potential_score(&self, input: &ValveInfoWeighted, current_minute: usize) -> usize {
        let current_valve_value = self
            .opened_valves
            .iter()
            .map(|(name, minute)| {
                let pressure = input[name].0;
                let effective_minutes = 30 - minute;
                effective_minutes * pressure
            })
            .sum::<usize>();

        let mut remaining_valves: TinyVec<[usize; 20]> = input
            .iter()
            .filter(|(valve, (pressure, _))| {
                *pressure != 0
                    && !self
                        .opened_valves
                        .iter()
                        .position(|(v, _)| v == *valve)
                        .is_some()
            })
            .map(|(_name, (pressure, _targets))| *pressure)
            .collect();

        remaining_valves.sort_unstable_by(|a, b| a.cmp(b).reverse());

        current_valve_value
            + remaining_valves
                .iter()
                .enumerate()
                .map(|(index, pressure)| {
                    // array is sorted to have highest value first, so give highest value first
                    // multiplying the index by 2 because you first need to move to the valve.
                    // this check is very pessimistic in assuming it takes 1 minute to get to every place
                    let minute_start = current_minute + index * 2 + 1;
                    if minute_start >= 30 {
                        return 0;
                    }
                    let active_turns = 30 - minute_start;
                    active_turns * pressure
                })
                .sum::<usize>()
    }
}

fn parse_input(input: &str) -> ValveInfo {
    input
        .split('\n')
        .map(|line| {
            let (part1, part2) = line.split_once(';').unwrap();
            let valve_name = &part1["Valve ".len().."Valve XX".len()];
            let rate_str = &part1["Valve XX has flow rate=".len()..];

            let part2_start_len = if part2.as_bytes()[7] == b's' {
                " tunnels lead to valves ".len()
            } else {
                " tunnel leads to valve ".len()
            };
            let part2 = &part2[part2_start_len..];
            let targets: Vec<_> = part2
                .split(", ")
                .map(|s| ValveName::from_bytes(s.as_bytes()))
                .collect();

            let valve_name = ValveName::from_bytes(valve_name.as_bytes());
            let rate = fast_parse_int(rate_str);

            (valve_name, (rate, targets))
        })
        .collect()
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(1651, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(1754, output);
}
