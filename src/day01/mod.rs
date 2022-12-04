use super::*;
use crate::utils::*;

pub struct Day;

impl AocDay<usize> for Day {
    const DAY: u32 = 1;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut lines = input.split('\n').peekable();
        let mut max = 0;

        while lines.peek().is_some() {
            let sum = lines
                .by_ref()
                .take_while(|&l| !l.is_empty())
                .map(fast_parse_int)
                .sum();

            max = max.max(sum);
        }

        max
    }
}

impl AocDayFull<usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut lines = input.split('\n').peekable();

        // note: order is from least to most
        let mut max = [0, 0, 0];

        while lines.peek().is_some() {
            let sum = lines
                .by_ref()
                .take_while(|&l| !l.is_empty())
                .map(fast_parse_int)
                .sum();

            if sum <= max[0] {
                continue;
            }

            if sum <= max[1] {
                max[0] = sum;
                continue;
            }

            if sum <= max[2] {
                max[0] = max[1];
                max[1] = sum;
                continue;
            }

            max[0] = max[1];
            max[1] = max[2];
            max[2] = sum;
        }

        max.into_iter().sum()
    }
}

#[test]
fn test_day_1_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(24000, output);
}

#[test]
fn test_day_1_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(67633, output);
}

#[test]
fn test_day_1_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(45000, output);
}

#[test]
fn test_day_1_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(199628, output);
}
