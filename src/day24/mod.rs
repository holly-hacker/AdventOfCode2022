use std::{cmp::Reverse, collections::BinaryHeap};

use ahash::{AHashMap, AHashSet};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 24;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input = Input::parse(input);

        do_dfs_forward(&input, 0)
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut input = Input::parse(input);

        let time_0 = 0;
        let time_1 = do_dfs_forward(&input, time_0);
        input.is_swapped = !input.is_swapped;
        let time_2 = do_dfs_backwards(&input, time_1);
        input.is_swapped = !input.is_swapped;
        do_dfs_forward(&input, time_2)
    }
}

fn do_dfs_forward(input: &Input, start_time: usize) -> usize {
    // we're essentially pathfinding in a 3D field, where time is the Z direction. We cannot go
    // back in the Z direction, but an exit is available at every value of Z.
    // this can be optimized this more by building up these Z levels.

    let mut visited = AHashSet::default();
    let mut queue = BinaryHeap::new();

    queue.push(ProgressPosition(input.get_start_pos(), start_time));

    let mut best_result = usize::MAX;

    'main_loop: while let Some(current) = queue.pop() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());

        let ProgressPosition(pos, minute) = current;

        if (minute + 1) >= best_result {
            continue;
        }

        debug_assert!(input.pos_free_at_minute(pos, minute));
        // println!("Evaluating {pos:?} at minute {minute:?}");

        // try waiting, if we can
        if input.pos_free_at_minute(pos, minute + 1) {
            let next_pos = ProgressPosition(pos, minute + 1);
            if next_pos.best_end_time(input) < best_result {
                queue.push(next_pos);
            }
        }

        if pos == input.get_start_pos() {
            // we need special handling for the start position. We can only move down or wait then.
            if input.pos_free_at_minute(input.get_first_move(), minute + 1) {
                let next_pos = ProgressPosition(input.get_first_move(), minute + 1);
                if next_pos.best_end_time(input) < best_result {
                    queue.push(next_pos);
                }
            }
        } else {
            // non-start position, look at all 4 directions
            // because we use a priority queue, the order should not be relevant
            let offsets = [(-1, 0), (0, -1), (1, 0), (0, 1)];

            for offset in offsets {
                let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
                if new_pos == input.get_end_pos() {
                    // we got there!
                    best_result = best_result.min(minute + 1);
                    // dbg!(best_result);

                    // println!("Len before cleanup: {}", queue.len());
                    // clear some branches with no potential
                    queue.retain(|x| x.best_end_time(input) < best_result);
                    // println!("Len after cleanup: {}", queue.len());

                    continue 'main_loop;
                }

                // if we're not moving to the end position, check that the new position is in bounds
                if (0..input.width).contains(&(new_pos.0 as usize))
                    && (0..input.height).contains(&(new_pos.1 as usize))
                    && input.pos_free_at_minute(new_pos, minute + 1)
                {
                    let next_pos = ProgressPosition(new_pos, minute + 1);
                    if next_pos.best_end_time(input) < best_result {
                        queue.push(next_pos);
                    }
                }
            }
        }
    }

    best_result
}

fn do_dfs_backwards(input: &Input, start_time: usize) -> usize {
    let mut visited = AHashSet::default();
    let mut queue = BinaryHeap::new();

    queue.push(Reverse(ProgressPosition(input.get_start_pos(), start_time)));

    let mut best_result = usize::MAX;

    'main_loop: while let Some(Reverse(current)) = queue.pop() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());

        let ProgressPosition(pos, minute) = current;

        if (minute + 1) >= best_result {
            continue;
        }

        debug_assert!(input.pos_free_at_minute(pos, minute));

        if input.pos_free_at_minute(pos, minute + 1) {
            let next_pos = ProgressPosition(pos, minute + 1);
            if next_pos.best_end_time(input) < best_result {
                queue.push(Reverse(next_pos));
            }
        }

        if pos == input.get_start_pos() {
            if input.pos_free_at_minute(input.get_first_move(), minute + 1) {
                let next_pos = ProgressPosition(input.get_first_move(), minute + 1);
                if next_pos.best_end_time(input) < best_result {
                    queue.push(Reverse(next_pos));
                }
            }
        } else {
            let offsets = [(-1, 0), (0, -1), (1, 0), (0, 1)];

            for offset in offsets {
                let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
                if new_pos == input.get_end_pos() {
                    best_result = best_result.min(minute + 1);

                    queue.retain(|x| x.0.best_end_time(input) < best_result);

                    continue 'main_loop;
                }

                if (0..input.width).contains(&(new_pos.0 as usize))
                    && (0..input.height).contains(&(new_pos.1 as usize))
                    && input.pos_free_at_minute(new_pos, minute + 1)
                {
                    let next_pos = ProgressPosition(new_pos, minute + 1);
                    if next_pos.best_end_time(input) < best_result {
                        queue.push(Reverse(next_pos));
                    }
                }
            }
        }
    }

    best_result
}

#[derive(Debug, Default)]
struct Input {
    /// The width, excluding walls
    width: usize,
    /// The height, excluding walls
    height: usize,

    /// Whether the start and end is swapped.
    is_swapped: bool,

    /// A hashmap containing all blizzards moving right for a certain the line number (the Y
    /// coordinate).
    horizontal_right: AHashMap<usize, Vec<usize>>,
    /// A hashmap containing all blizzards moving left for a certain the line number (the Y
    /// coordinate).
    horizontal_left: AHashMap<usize, Vec<usize>>,
    /// A hashmap containing all blizzards moving down for a certain the column number (the X
    /// coordinate).
    vertical_down: AHashMap<usize, Vec<usize>>,
    /// A hashmap containing all blizzards moving up for a certain the column number (the X
    /// coordinate).
    vertical_up: AHashMap<usize, Vec<usize>>,
}

impl Input {
    fn parse(input: &str) -> Input {
        let mut ret = Input::default();
        // the outer characters will always be either a wall (`#`) or the start/end position. Because
        // the start/end position are always the same, we don't need to keep track of rows/columns.
        input.lines().skip(1).enumerate().for_each(|(y, line)| {
            let bytes = line.as_bytes();
            if bytes[1] == b'#' {
                // this is the last line
                ret.height = y;
                return;
            }
            bytes.into_iter().skip(1).enumerate().for_each(|(x, char)| {
                let y = y;
                match char {
                    b'<' => ret.horizontal_left.entry(y).or_insert(vec![]).push(x),
                    b'>' => ret.horizontal_right.entry(y).or_insert(vec![]).push(x),
                    b'^' => ret.vertical_up.entry(x).or_insert(vec![]).push(y),
                    b'v' => ret.vertical_down.entry(x).or_insert(vec![]).push(y),
                    b'.' => {} // open space, ignore it
                    b'#' => {
                        // end of a line, update the width
                        ret.width = x;
                    }
                    _ => panic!("unexpected char: {}", *char as char),
                }
            });
        });

        ret
    }

    const fn get_start_pos(&self) -> (isize, isize) {
        if self.is_swapped {
            (self.width as isize - 1, self.height as isize)
        } else {
            (0, -1)
        }
    }

    const fn get_end_pos(&self) -> (isize, isize) {
        if self.is_swapped {
            (0, -1)
        } else {
            (self.width as isize - 1, self.height as isize)
        }
    }
    const fn get_first_move(&self) -> (isize, isize) {
        if self.is_swapped {
            (self.width as isize - 1, self.height as isize - 1)
        } else {
            (0, 0)
        }
    }

    pub fn pos_free_at_minute(&self, pos: (isize, isize), minute: usize) -> bool {
        let (x, y) = pos;

        let w = self.width as isize;
        let h = self.height as isize;
        let m = minute as isize;

        // check if this position contains a blizzard at this time, by checking where the blizzards were at minute 0
        !self
            .horizontal_right
            .get(&(y as usize))
            .map(|vec| vec.contains(&((x - m).rem_euclid(w) as usize)))
            .unwrap_or(false)
            && !self
                .horizontal_left
                .get(&(y as usize))
                .map(|vec| vec.contains(&((x + m).rem_euclid(w) as usize)))
                .unwrap_or(false)
            && !self
                .vertical_down
                .get(&(x as usize))
                .map(|vec| vec.contains(&((y - m).rem_euclid(h) as usize)))
                .unwrap_or(false)
            && !self
                .vertical_up
                .get(&(x as usize))
                .map(|vec| vec.contains(&((y + m).rem_euclid(h) as usize)))
                .unwrap_or(false)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ProgressPosition((isize, isize), usize);

impl ProgressPosition {
    fn score(&self) -> isize {
        // TODO: also consider time?
        self.0 .0 + self.0 .1
    }

    /// Calculates the best time this can reach the end with
    fn best_end_time(&self, input: &Input) -> usize {
        let current_time = self.1;
        let (end_x, end_y) = input.get_end_pos();
        let manhattan_distance = end_x.abs_diff(self.0 .0) + end_y.abs_diff(self.0 .1);

        current_time + manhattan_distance
    }
}

impl PartialOrd for ProgressPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ProgressPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(18, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(240, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(54, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(717, output);
}
