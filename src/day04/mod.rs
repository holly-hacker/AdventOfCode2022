use crate::utils::fast_parse_int;

use super::*;

pub struct Day;

impl AocDay<usize> for Day {
    const DAY: u32 = 4;

    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");

    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        input
            .split('\n')
            .map(|l| {
                let (range_1_1, rest) = l.split_once('-').unwrap();
                let (range_1_2, rest) = rest.split_once(',').unwrap();
                let (range_2_1, range_2_2) = rest.split_once('-').unwrap();

                let range_1_start = fast_parse_int(range_1_1);
                let range_1_end = fast_parse_int(range_1_2);
                let range_2_start = fast_parse_int(range_2_1);
                let range_2_end = fast_parse_int(range_2_2);

                usize::from(range_1_start >= range_2_start && range_1_end <= range_2_end)
                    | usize::from(range_2_start >= range_1_start && range_2_end <= range_1_end)
            })
            .sum()
    }
}

impl AocDayFull<usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        input
            .split('\n')
            .map(|l| {
                let (range_1_1, rest) = l.split_once('-').unwrap();
                let (range_1_2, rest) = rest.split_once(',').unwrap();
                let (range_2_1, range_2_2) = rest.split_once('-').unwrap();

                let range_1_start = fast_parse_int(range_1_1);
                let range_1_end = fast_parse_int(range_1_2);
                let range_2_start = fast_parse_int(range_2_1);
                let range_2_end = fast_parse_int(range_2_2);

                usize::from(!(range_2_end < range_1_start || range_2_start > range_1_end))
            })
            .sum()
    }
}

#[test]
fn solve_day_4_silver_sample() {
    assert_eq!(Day::calculate_silver(Day::INPUT_SAMPLE), 2);
}

#[test]
fn solve_day_4_silver_real() {
    assert_eq!(Day::calculate_silver(Day::INPUT_REAL), 456);
}

#[test]
fn solve_day_4_gold_sample() {
    assert_eq!(Day::calculate_gold(Day::INPUT_SAMPLE), 4);
}

#[test]
fn solve_day_4_gold_real() {
    assert_eq!(Day::calculate_gold(Day::INPUT_REAL), 808);
}
