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
                let (range_1, range_2) = l.split_once(',').unwrap();
                let (range_1_1, range_1_2) = range_1.split_once('-').unwrap();
                let (range_2_1, range_2_2) = range_2.split_once('-').unwrap();

                let range_1: (usize, usize) =
                    (range_1_1.parse().unwrap(), range_1_2.parse().unwrap());
                let range_2: (usize, usize) =
                    (range_2_1.parse().unwrap(), range_2_2.parse().unwrap());

                if range_1.0 >= range_2.0 && range_1.1 <= range_2.1 {
                    1
                } else if range_2.0 >= range_1.0 && range_2.1 <= range_1.1 {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

impl AocDayFull<usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        input
            .split('\n')
            .map(|l| {
                let (range_1, range_2) = l.split_once(',').unwrap();
                let (range_1_1, range_1_2) = range_1.split_once('-').unwrap();
                let (range_2_1, range_2_2) = range_2.split_once('-').unwrap();

                let range_1: (usize, usize) =
                    (range_1_1.parse().unwrap(), range_1_2.parse().unwrap());
                let range_2: (usize, usize) =
                    (range_2_1.parse().unwrap(), range_2_2.parse().unwrap());

                if range_1.0 >= range_2.0 && range_1.0 <= range_2.1 {
                    // range_1.0 is within range_2
                    1
                } else if range_1.1 >= range_2.0 && range_1.1 <= range_2.1 {
                    // range_1.1 is within range_2
                    1
                } else if range_2.0 >= range_1.0 && range_2.0 <= range_1.1 {
                    // range_2.0 is within range_1
                    1
                } else if range_2.1 >= range_1.0 && range_2.1 <= range_1.1 {
                    // range_2.1 is within range_1
                    1
                } else {
                    0
                }
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
