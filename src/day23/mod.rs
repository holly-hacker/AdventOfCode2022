use ahash::{AHashMap, AHashSet};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 23;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input = parse_input(input);

        let mut positions = input;
        let mut movements = AHashMap::default();
        let mut directions = Direction::DEFAULT_DIRECTIONS;
        for _ in 0..10 {
            movements.clear();

            'positions: for (x, y) in &positions {
                let (x, y) = (*x, *y);

                let occ_n = positions.contains(&(x, y - 1));
                let occ_nw = positions.contains(&(x - 1, y - 1));
                let occ_ne = positions.contains(&(x + 1, y - 1));
                let occ_s = positions.contains(&(x, y + 1));
                let occ_sw = positions.contains(&(x - 1, y + 1));
                let occ_se = positions.contains(&(x + 1, y + 1));
                let occ_w = positions.contains(&(x - 1, y));
                let occ_e = positions.contains(&(x + 1, y));

                if [occ_n, occ_nw, occ_ne, occ_s, occ_sw, occ_se, occ_w, occ_e]
                    .iter()
                    .all(|x| !*x)
                {
                    // no elves nearby, so don't move
                    continue;
                }

                for direction in directions {
                    let (occupied, new_pos) = match direction {
                        Direction::North => (!occ_ne && !occ_n && !occ_nw, (x, y - 1)),
                        Direction::South => (!occ_se && !occ_s && !occ_sw, (x, y + 1)),
                        Direction::West => (!occ_w && !occ_sw && !occ_nw, (x - 1, y)),
                        Direction::East => (!occ_e && !occ_ne && !occ_se, (x + 1, y)),
                    };
                    if occupied {
                        let new_occupancy = match movements.get(&new_pos) {
                            Some(Occupancy::Multiple) => None,
                            Some(_) => Some(Occupancy::Multiple),
                            None => Some(Occupancy::Single((x, y))),
                        };
                        if let Some(insert) = new_occupancy {
                            movements.insert(new_pos, insert);
                        }

                        continue 'positions;
                    }
                }
            }

            // update positions

            movements
                .iter()
                .filter_map(|(to, occ)| match occ {
                    Occupancy::Single(from) => Some((from, to)),
                    Occupancy::Multiple => None,
                })
                .for_each(|(from, to)| {
                    let _was_present = positions.remove(from);
                    debug_assert!(_was_present);
                    let _newly_inserted = positions.insert(*to);
                    debug_assert!(_newly_inserted);
                });

            directions = [directions[1], directions[2], directions[3], directions[0]];
        }

        // calculate biggest rect
        let (x_min, y_min, x_max, y_max) =
            positions
                .iter()
                .fold((i32::MAX, i32::MAX, i32::MIN, i32::MIN), |acc, item| {
                    (
                        acc.0.min(item.0),
                        acc.1.min(item.1),
                        acc.2.max(item.0),
                        acc.3.max(item.1),
                    )
                });

        let x_diff = (x_max - x_min + 1) as usize;
        let y_diff = (y_max - y_min + 1) as usize;

        let total = x_diff * y_diff - positions.len();

        total
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let input = parse_input(input);

        let mut positions = input;
        let mut movements = AHashMap::default();
        let mut directions = Direction::DEFAULT_DIRECTIONS;
        let mut round = 0;
        loop {
            round += 1;
            movements.clear();

            'positions: for (x, y) in &positions {
                let (x, y) = (*x, *y);

                let occ_n = positions.contains(&(x, y - 1));
                let occ_nw = positions.contains(&(x - 1, y - 1));
                let occ_ne = positions.contains(&(x + 1, y - 1));
                let occ_s = positions.contains(&(x, y + 1));
                let occ_sw = positions.contains(&(x - 1, y + 1));
                let occ_se = positions.contains(&(x + 1, y + 1));
                let occ_w = positions.contains(&(x - 1, y));
                let occ_e = positions.contains(&(x + 1, y));

                if [occ_n, occ_nw, occ_ne, occ_s, occ_sw, occ_se, occ_w, occ_e]
                    .iter()
                    .all(|x| !*x)
                {
                    // no elves nearby, so don't move
                    continue;
                }

                for direction in directions {
                    let (occupied, new_pos) = match direction {
                        Direction::North => (!occ_ne && !occ_n && !occ_nw, (x, y - 1)),
                        Direction::South => (!occ_se && !occ_s && !occ_sw, (x, y + 1)),
                        Direction::West => (!occ_w && !occ_sw && !occ_nw, (x - 1, y)),
                        Direction::East => (!occ_e && !occ_ne && !occ_se, (x + 1, y)),
                    };
                    if occupied {
                        let new_occupancy = match movements.get(&new_pos) {
                            Some(Occupancy::Multiple) => None,
                            Some(_) => Some(Occupancy::Multiple),
                            None => Some(Occupancy::Single((x, y))),
                        };
                        if let Some(insert) = new_occupancy {
                            movements.insert(new_pos, insert);
                        }

                        continue 'positions;
                    }
                }
            }

            // update positions
            let mut movement = false;
            movements
                .iter()
                .filter_map(|(to, occ)| match occ {
                    Occupancy::Single(from) => Some((from, to)),
                    Occupancy::Multiple => None,
                })
                .for_each(|(from, to)| {
                    let _was_present = positions.remove(from);
                    debug_assert!(_was_present);
                    let _newly_inserted = positions.insert(*to);
                    debug_assert!(_newly_inserted);
                    movement = true;
                });

            directions = [directions[1], directions[2], directions[3], directions[0]];

            if !movement {
                return round;
            }
        }
    }
}

enum Occupancy {
    Single((i32, i32)),
    Multiple,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub const DEFAULT_DIRECTIONS: [Direction; 4] = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];
}

fn parse_input(input: &str) -> AHashSet<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(move |(x, c)| {
                    if *c == b'#' {
                        Some((x as i32, y as i32))
                    } else {
                        None
                    }
                })
        })
        .collect()
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(110, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(3923, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(20, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(1019, output);
}
