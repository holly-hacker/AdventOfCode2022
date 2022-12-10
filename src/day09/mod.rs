use ahash::AHashSet;

use crate::utils::{fast_parse_int_from_bytes, split_once};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 9;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut visited = AHashSet::default();
        let mut bytes = input.as_bytes();

        let mut head_positions = (0, 0);
        let mut tail_position = (0, 0);

        visited.insert(tail_position);

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
                let pos_before = tail_position;
                tail_position = calculate_tail_position(head_positions, tail_position);

                if pos_before != tail_position {
                    visited.insert(tail_position);
                }
            }
        }

        visited.len()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut visited = AHashSet::default();
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

                let pos_before = snake_positions[9];
                for i in 0..9 {
                    snake_positions[i + 1] =
                        calculate_tail_position(snake_positions[i], snake_positions[i + 1]);
                }

                if pos_before != snake_positions[9] {
                    visited.insert(snake_positions[9]);
                }
            }
        }

        visited.len()
    }
}

fn calculate_tail_position(head: (isize, isize), tail: (isize, isize)) -> (isize, isize) {
    let tail_relative = (tail.0 - head.0, tail.1 - head.1);

    let tail_relative = match (tail_relative.0.abs(), tail_relative.1.abs()) {
        (0 | 1, 0 | 1) => return tail,
        (2, 0 | 1) => (tail_relative.0.signum(), 0),
        (0 | 1, 2) => (0, tail_relative.1.signum()),
        (2, 2) => (tail_relative.0.signum(), tail_relative.1.signum()),
        _ => unreachable!(),
    };

    (head.0 + tail_relative.0, head.1 + tail_relative.1)
}

#[test]
fn test_calculate_tail_position() {
    assert_eq!(calculate_tail_position((0, 0), (1, 1)), (1, 1));
    assert_eq!(calculate_tail_position((0, 0), (1, 2)), (0, 1));
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(13, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(6339, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(1, output);
}

#[test]
fn test_gold_sample_2() {
    let output = Day::calculate_gold(include_str!("input_sample_2.txt"));
    assert_eq!(36, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(2541, output);
}
