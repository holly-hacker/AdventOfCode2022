use std::fmt::Debug;

use ahash::{AHashMap, AHashSet};
use tinyvec::{tiny_vec, ArrayVec, TinyVec};

use crate::utils::fast_parse_int;

use super::*;

pub struct Day;

type ValveInfo = AHashMap<ValveName, (usize, Vec<ValveName>)>;
type ValveInfoWeighted = AHashMap<ValveName, (usize, TinyVec<[(ValveName, usize); 16]>)>;
type ValveDistanceMap = AHashMap<(ValveName, ValveName), usize>;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 16;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input: ValveInfo = parse_input(input);
        let input = optimize_input(&input);
        let distances = build_distance_map(&input);
        // println!("len: {}", distances.len());
        // println!("tree: {distances:#?}");

        recursive_search(&input, &distances, Path::new_start(), 0)
    }
}

fn recursive_search(
    input: &ValveInfoWeighted,
    distances: &ValveDistanceMap,
    path: Path,
    current_minute: usize,
) -> usize {
    // println!("Current minute: {current_minute}");
    if current_minute >= 30 {
        // println!("Exit because time limit is reached ({current_score}): {path:?}");
        return path.calculate_score();
    }

    // calculate what happens if you just wait at this point
    let mut max = path.calculate_score();

    let current_valve = path.route.last().unwrap();
    for target_valve in input.keys() {
        if target_valve == current_valve || path.route.contains(target_valve) {
            continue;
        }
        let target_distance = distances[&(*current_valve, *target_valve)];

        debug_assert_ne!(
            target_distance, 0,
            "target distance should never be zero for moving somewhere"
        );

        // move to the given location to turn the valve
        let target_pressure = input[target_valve].0;
        debug_assert_ne!(target_pressure, 0);
        let mut cloned_path = path.clone();
        cloned_path.route.push(*target_valve);
        cloned_path
            .opened_valves
            .push((target_pressure, current_minute + target_distance + 1));

        // start searching from this place now
        let new_score = recursive_search(
            input,
            distances,
            cloned_path,
            current_minute + target_distance + 1,
        );
        max = max.max(new_score);
    }

    max
}

/// Optimizes the input by folding valves with flow rates of 0. It ensures that `AA` is always kept.
fn optimize_input(input_info: &ValveInfo) -> ValveInfoWeighted {
    let mut output_info = ValveInfoWeighted::new();

    for (valve, (pressure, _targets)) in input_info {
        if *pressure != 0 || *valve == ValveName::AA {
            let new_targets = get_all_target_distances(*valve, input_info);
            output_info.insert(*valve, (*pressure, new_targets));
        }
    }

    output_info
}

fn get_all_target_distances(
    start: ValveName,
    valve_info: &ValveInfo,
) -> TinyVec<[(ValveName, usize); 16]> {
    let mut set = AHashSet::new();
    set.insert(start);
    let (_pressure, targets) = &valve_info[&start];
    let new_targets = get_all_target_distances_recursive(set, &targets, valve_info, 0);

    new_targets
}

fn get_all_target_distances_recursive(
    visited: AHashSet<ValveName>,
    targets: &[ValveName],
    valve_info: &ValveInfo,
    accumulated_distance: usize,
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

            for (nested_resolved_target, weight) in get_all_target_distances_recursive(
                new_visited,
                target_targets,
                valve_info,
                accumulated_distance + 1,
            ) {
                debug_assert!(!visited.contains(&nested_resolved_target));
                resolved_targets.push((nested_resolved_target, weight));
            }
        } else {
            // just return this
            debug_assert!(!visited.contains(target));
            resolved_targets.push((*target, accumulated_distance + 1));
        }
    }

    resolved_targets
}

/// Builds a map with the shortest distance between any 2 valves with non-zero pressure. Always includes AA as well.
fn build_distance_map(valve_info: &ValveInfoWeighted) -> ValveDistanceMap {
    let mut distance_map = ValveDistanceMap::new();
    for valve in valve_info.keys() {
        let mut distances = AHashMap::new();

        // calculate the distance to each other node
        calculate_distance_recursive(&mut distances, *valve, *valve, valve_info, 0);

        // add those distances to the main map
        for (target, distance) in distances {
            let target_pressure = valve_info[&target].0;
            if target_pressure != 0 && target != ValveName::AA {
                distance_map.insert((*valve, target), distance);
            }
        }
    }

    distance_map
}

fn calculate_distance_recursive(
    found_distances: &mut AHashMap<ValveName, usize>,
    start: ValveName,
    current: ValveName,
    valve_info: &ValveInfoWeighted,
    accumulated_distance: usize,
) {
    let (_, targets) = &valve_info[&current];

    for (target, distance) in targets {
        if start == *target {
            continue;
        }

        let total_distance = accumulated_distance + distance;
        if !found_distances.contains_key(target) {
            let target_pressure = valve_info[target].0;
            if target_pressure != 0 && *target != ValveName::AA {
                // println!("{start:?} > inserting {target:?} with val {total_distance}");
                found_distances.insert(*target, total_distance);
            }
            calculate_distance_recursive(
                found_distances,
                start,
                *target,
                valve_info,
                total_distance,
            );
        } else if total_distance < found_distances[&target] {
            // already exists but we got there in a faster way
            // println!("{start:?} > inserting {target:?} with val {total_distance} (faster!)");
            found_distances.insert(*target, total_distance);
            debug_assert_ne!(valve_info[target].0, 0);

            calculate_distance_recursive(
                found_distances,
                start,
                *target,
                valve_info,
                total_distance,
            );
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
struct ValveName(usize);

impl ValveName {
    const AA: ValveName = ValveName(0);

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
    /// List of opened valves, containing `(pressure, time)`.
    opened_valves: ArrayVec<[(usize, usize); 15]>,
    route: ArrayVec<[ValveName; 30]>,
}

impl Path {
    /// Creates a new path containing just AA
    pub fn new_start() -> Self {
        let aa = ValveName::AA;
        let mut route = ArrayVec::new();
        route.push(aa);
        Self {
            opened_valves: ArrayVec::default(),
            route,
        }
    }

    pub fn calculate_score(&self) -> usize {
        self.opened_valves
            .iter()
            .filter(|(_, time)| *time < 30)
            .map(|(pressure, time)| pressure * (30 - time))
            .sum()
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
