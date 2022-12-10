use crate::utils::{fast_parse_int_from_bytes, split_once};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 4;

    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");

    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut input = input.as_bytes();
        let mut sum = 0;
        while !input.is_empty() {
            let (range_1_1, range_1_2, range_2_1, range_2_2);

            (range_1_1, input) = split_once(input, b'-').unwrap();
            let range_1_start = fast_parse_int_from_bytes(range_1_1);

            (range_1_2, input) = split_once(input, b',').unwrap();
            let range_1_end = fast_parse_int_from_bytes(range_1_2);

            (range_2_1, input) = split_once(input, b'-').unwrap();
            let range_2_start = fast_parse_int_from_bytes(range_2_1);

            (range_2_2, input) = split_once(input, b'\n').unwrap_or((input, &[]));
            let range_2_end = fast_parse_int_from_bytes(range_2_2);

            sum += usize::from(range_1_start >= range_2_start && range_1_end <= range_2_end)
                | usize::from(range_2_start >= range_1_start && range_2_end <= range_1_end);
        }
        sum
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut input = input.as_bytes();
        let mut sum = 0;
        while !input.is_empty() {
            let (range_1_1, range_1_2, range_2_1, range_2_2);

            (range_1_1, input) = split_once(input, b'-').unwrap();
            let range_1_start = fast_parse_int_from_bytes(range_1_1);

            (range_1_2, input) = split_once(input, b',').unwrap();
            let range_1_end = fast_parse_int_from_bytes(range_1_2);

            (range_2_1, input) = split_once(input, b'-').unwrap();
            let range_2_start = fast_parse_int_from_bytes(range_2_1);

            (range_2_2, input) = split_once(input, b'\n').unwrap_or((input, &[]));
            let range_2_end = fast_parse_int_from_bytes(range_2_2);

            sum += usize::from(!(range_2_end < range_1_start || range_2_start > range_1_end));
        }
        sum
    }
}

#[test]
fn test_silver_sample() {
    assert_eq!(Day::calculate_silver(Day::INPUT_SAMPLE), 2);
}

#[test]
fn test_silver_real() {
    assert_eq!(Day::calculate_silver(Day::INPUT_REAL), 456);
}

#[test]
fn test_gold_sample() {
    assert_eq!(Day::calculate_gold(Day::INPUT_SAMPLE), 4);
}

#[test]
fn test_gold_real() {
    assert_eq!(Day::calculate_gold(Day::INPUT_REAL), 808);
}
