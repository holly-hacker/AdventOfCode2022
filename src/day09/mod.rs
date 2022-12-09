use std::collections::HashSet;

use crate::utils::{fast_parse_int_from_bytes, split_once};

use super::*;

pub struct Day;

impl AocDay<usize> for Day {
    const DAY: u32 = 9;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut visited = HashSet::new(); // TODO: for perf, use a different hash function
        let mut bytes = input.as_bytes();

        let mut head_positions = (0, 0);
        let mut tail_positions = (0, 0);

        visited.insert(tail_positions);

        while !bytes.is_empty() {
            let line;
            (line, bytes) = split_once(bytes, b'\n').unwrap_or((bytes, b""));

            let direction = line[0];
            let length = fast_parse_int_from_bytes(&line[2..]);

            for _ in 0..length {
                head_positions = match direction {
                    b'L' => (head_positions.0 - 1, head_positions.1),
                    b'R' => (head_positions.0 + 1, head_positions.1),
                    b'U' => (head_positions.0, head_positions.1 - 1),
                    b'D' => (head_positions.0, head_positions.1 + 1),
                    _ => unreachable!(),
                };
                tail_positions = calculate_tail_position(head_positions, tail_positions);

                visited.insert(tail_positions);
            }
        }

        visited.len()
    }
}

impl AocDayFull<usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut visited = HashSet::new(); // TODO: for perf, use a different hash function
        let mut bytes = input.as_bytes();

        let mut snake_positions = [(0, 0); 10];

        visited.insert(snake_positions[9]);

        while !bytes.is_empty() {
            let line;
            (line, bytes) = split_once(bytes, b'\n').unwrap_or((bytes, b""));

            let direction = line[0];
            let length = fast_parse_int_from_bytes(&line[2..]);

            for _ in 0..length {
                snake_positions[0] = match direction {
                    b'L' => (snake_positions[0].0 - 1, snake_positions[0].1),
                    b'R' => (snake_positions[0].0 + 1, snake_positions[0].1),
                    b'U' => (snake_positions[0].0, snake_positions[0].1 - 1),
                    b'D' => (snake_positions[0].0, snake_positions[0].1 + 1),
                    _ => unreachable!(),
                };

                for i in 0..9 {
                    snake_positions[i + 1] =
                        calculate_tail_position(snake_positions[i], snake_positions[i + 1]);
                }

                visited.insert(snake_positions[9]);
            }
        }

        visited.len()
    }
}

fn calculate_tail_position(head: (isize, isize), tail: (isize, isize)) -> (isize, isize) {
    let tail_relative = (tail.0 - head.0, tail.1 - head.1);
    let x_sign = tail_relative.0.signum();
    let y_sign = tail_relative.1.signum();

    let tail_relative = match (tail_relative.0.abs(), tail_relative.1.abs()) {
        (0, 0) | (0, 1) | (1, 0) | (1, 1) => tail_relative,
        (2, 0) | (2, 1) => (1 * x_sign, 0),
        (0, 2) | (1, 2) => (0, 1 * y_sign),
        (2, 2) => (1 * x_sign, 1 * y_sign),
        x => unreachable!("{x:?}"),
    };

    (head.0 + tail_relative.0, head.1 + tail_relative.1)
}

#[test]
fn test_calculate_tail_position() {
    assert_eq!(calculate_tail_position((0, 0), (1, 1)), (1, 1));
    assert_eq!(calculate_tail_position((0, 0), (1, 2)), (0, 1));
}

#[test]
fn test_day_9_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(13, output);
}

#[test]
fn test_day_9_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(6339, output);
}

#[test]
fn test_day_9_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(1, output);
}

#[test]
fn test_day_9_gold_sample_2() {
    let output = Day::calculate_gold(include_str!("input_sample_2.txt"));
    assert_eq!(36, output);
}

#[test]
fn test_day_9_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(2541, output);
}
