use std::{collections::VecDeque, ops::Add};

use ahash::AHashSet;
use tinyvec::ArrayVec;

use crate::utils::{fast_parse_int_from_bytes, split_once};

use super::*;

pub struct Day;

const IMMEDIATE_NEIGHBOURS: [Vec3; 6] = [
    Vec3(-1, 0, 0),
    Vec3(1, 0, 0),
    Vec3(0, -1, 0),
    Vec3(0, 1, 0),
    Vec3(0, 0, -1),
    Vec3(0, 0, 1),
];
const ALL_NEIGHBOURS: [Vec3; 27 - 1] = [
    Vec3(-1, -1, -1),
    Vec3(-1, -1, 0),
    Vec3(-1, -1, 1),
    Vec3(-1, 0, -1),
    Vec3(-1, 0, 0),
    Vec3(-1, 0, 1),
    Vec3(-1, 1, -1),
    Vec3(-1, 1, 0),
    Vec3(-1, 1, 1),
    Vec3(0, -1, -1),
    Vec3(0, -1, 0),
    Vec3(0, -1, 1),
    Vec3(0, 0, -1),
    Vec3(0, 0, 1),
    Vec3(0, 1, -1),
    Vec3(0, 1, 0),
    Vec3(0, 1, 1),
    Vec3(1, -1, -1),
    Vec3(1, -1, 0),
    Vec3(1, -1, 1),
    Vec3(1, 0, -1),
    Vec3(1, 0, 0),
    Vec3(1, 0, 1),
    Vec3(1, 1, -1),
    Vec3(1, 1, 0),
    Vec3(1, 1, 1),
];

impl SolutionSilver<isize> for Day {
    const DAY: u32 = 18;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> isize {
        let mut cubes = AHashSet::<Vec3>::default();
        input
            .as_bytes()
            .split(|b| *b == b'\n')
            .map(|line| {
                let (num1, line) = split_once(line, b',').unwrap();
                let (num2, num3) = split_once(line, b',').unwrap();

                let num1 = fast_parse_int_from_bytes(num1) as isize;
                let num2 = fast_parse_int_from_bytes(num2) as isize;
                let num3 = fast_parse_int_from_bytes(num3) as isize;

                let vec = Vec3(num1, num2, num3);

                let sides = 6;
                let count_touching = IMMEDIATE_NEIGHBOURS
                    .into_iter()
                    .filter(|offset| {
                        let neighbour = vec + *offset;
                        cubes.contains(&neighbour)
                    })
                    .count() as isize;
                cubes.insert(Vec3(num1, num2, num3));
                sides - count_touching * 2
            })
            .sum()
    }
}

impl SolutionGold<isize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        // insert all cubes
        let voxels: AHashSet<Vec3> = input
            .as_bytes()
            .split(|b| *b == b'\n')
            .map(|line| {
                let (num1, line) = split_once(line, b',').unwrap();
                let (num2, num3) = split_once(line, b',').unwrap();

                let num1 = fast_parse_int_from_bytes(num1) as isize;
                let num2 = fast_parse_int_from_bytes(num2) as isize;
                let num3 = fast_parse_int_from_bytes(num3) as isize;

                Vec3(num1, num2, num3)
            })
            .collect();

        let first_voxel = *voxels.iter().min().unwrap();
        let exposed_direction = 0;
        let first_air = first_voxel + IMMEDIATE_NEIGHBOURS[exposed_direction];
        debug_assert!(!voxels.contains(&(first_voxel + IMMEDIATE_NEIGHBOURS[exposed_direction])));

        let mut visited_voxels = AHashSet::<Vec3>::default();
        let mut visited_air = AHashSet::<Vec3>::default();

        visited_air.insert(first_air);

        let mut count = 0;

        let to_add =
            find_other_exposed_voxels(&voxels, first_voxel, &mut visited_voxels, &mut visited_air);

        count += to_add;

        count
    }
}

/// DFS to find other exposed cubes.
///
/// We need to take care to only count faces that are actually exposed to outside air.
fn find_other_exposed_voxels(
    all_voxels: &AHashSet<Vec3>,
    first_position: Vec3,
    visited_voxels: &mut AHashSet<Vec3>,
    outside_air: &mut AHashSet<Vec3>,
) -> usize {
    let mut nodes_to_visit = VecDeque::<Vec3>::new();
    nodes_to_visit.push_back(first_position);
    let _new_insert = visited_voxels.insert(first_position);
    debug_assert!(_new_insert);

    let mut total_count = 0;

    while let Some(current_position) = nodes_to_visit.pop_front() {
        // find neighbouring air blocks and mark their surrounding air blocks as "outside air"
        // do this multiple times to make sure we have everything

        // TODO: very inefficient! fix me!
        for _ in 0..4 {
            for surrounding_outside_air in (-2..=2)
                .flat_map(|a| (-2..=2).flat_map(move |b| (-2..=2).map(move |c| (a, b, c))))
                .map(|(a, b, c)| Vec3(a, b, c))
                .into_iter()
                .map(|offset| current_position + offset)
                .filter(|possible_air| outside_air.contains(possible_air))
                .collect::<ArrayVec<[_; 5 * 5 * 5]>>()
            {
                // we have block that is marked as "outside air", now mark its immediate neighbours
                // as outside air too.
                for actual_air in IMMEDIATE_NEIGHBOURS
                    .into_iter()
                    .map(|offset| surrounding_outside_air + offset)
                    .filter(|air_pos| !all_voxels.contains(&air_pos))
                {
                    outside_air.insert(actual_air);
                }
            }
        }

        // add the faces that touch the outside air to the count
        total_count += IMMEDIATE_NEIGHBOURS
            .into_iter()
            .map(|o| current_position + o)
            .filter(|neighbour| outside_air.contains(&neighbour))
            .count();

        // add all neighbours that are not checked yet
        let mut recurse_positions = ALL_NEIGHBOURS
            .into_iter()
            .map(|offset| current_position + offset)
            .filter(|pos| all_voxels.contains(&pos))
            .collect::<ArrayVec<[_; 26]>>();
        recurse_positions.sort_unstable(); // prevent RNG during debugging
        for recurse_pos in recurse_positions {
            if visited_voxels.contains(&recurse_pos) {
                continue;
            }

            nodes_to_visit.push_front(recurse_pos);
            let _new_insert = visited_voxels.insert(recurse_pos);
            debug_assert!(_new_insert);
        }
    }

    total_count
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
struct Vec3(isize, isize, isize);

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vec3 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .cmp(&other.0)
            .then(self.1.cmp(&other.1))
            .then(self.2.cmp(&other.2))
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(64, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(4370, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(58, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(2458, output);
}
