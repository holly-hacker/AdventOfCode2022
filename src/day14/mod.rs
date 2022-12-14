use ahash::AHashSet;

use crate::utils::fast_parse_int;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 14;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut grid = parse_input(input);

        // simulate the sand
        let mut iteration = 0;
        loop {
            let mut position = (500, 0);

            // fall down
            loop {
                // we fell off (the world)
                if position.1 > 1000 {
                    return iteration;
                }

                if !grid.contains(&(position.0, position.1 + 1)) {
                    position = (position.0, position.1 + 1);
                    continue;
                }

                if !grid.contains(&(position.0 - 1, position.1 + 1)) {
                    position = (position.0 - 1, position.1 + 1);
                    continue;
                }

                if !grid.contains(&(position.0 + 1, position.1 + 1)) {
                    position = (position.0 + 1, position.1 + 1);
                    continue;
                }

                // we cannot drop down further, place the sand here
                grid.insert(position);
                break;
            }

            iteration += 1;
        }
    }
}
impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut grid = parse_input(input);

        // what the fuck? this should be offset by 2 but it works as-is
        let rock_bottom = grid.iter().map(|(_x, y)| y).max().unwrap() + 0;

        // simulate the sand
        let mut iteration = 0;
        loop {
            let mut position = (500, 0);

            if grid.contains(&position) {
                return iteration;
            }

            // fall down
            loop {
                // we hit rock bottom
                if position.1 > rock_bottom {
                    grid.insert(position);
                    break;
                }

                if !grid.contains(&(position.0, position.1 + 1)) {
                    position = (position.0, position.1 + 1);
                    continue;
                }

                if !grid.contains(&(position.0 - 1, position.1 + 1)) {
                    position = (position.0 - 1, position.1 + 1);
                    continue;
                }

                if !grid.contains(&(position.0 + 1, position.1 + 1)) {
                    position = (position.0 + 1, position.1 + 1);
                    continue;
                }

                // we cannot drop down further, place the sand here
                grid.insert(position);
                break;
            }

            iteration += 1;
        }
    }
}

fn parse_input(input: &str) -> AHashSet<(usize, usize)> {
    // fuck performance all my homies hate performance
    let iterator = input.lines().map(|l| {
        l.split(" -> ")
            .map(|p| p.split_once(',').unwrap())
            .map(|(a, b)| (fast_parse_int(a), fast_parse_int(b)))
    });

    let mut grid = AHashSet::new(); // TODO: oh no my perfomance!
    for mut line in iterator {
        let mut last = line.next().unwrap();

        for curr in line {
            if curr.0 != last.0 {
                for i in curr.0.min(last.0)..=curr.0.max(last.0) {
                    grid.insert((i, curr.1));
                }
            } else {
                for i in curr.1.min(last.1)..=curr.1.max(last.1) {
                    grid.insert((curr.0, i));
                }
            }

            last = curr;
        }
    }

    grid
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(24, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(808, output);
}

#[test]
fn test_silver_gold() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(93, output);
}
