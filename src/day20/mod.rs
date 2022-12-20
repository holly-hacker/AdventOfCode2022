use std::collections::VecDeque;

use super::*;

pub struct Day;

impl SolutionSilver<isize> for Day {
    const DAY: u32 = 20;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> isize {
        let input_original: Vec<isize> = input.lines().map(|l| l.parse().unwrap()).collect();

        // add an `enumerate` with original indices. We can't scan later because there are duplicate numbers.
        let mut values = input_original
            .iter()
            .cloned()
            .enumerate()
            .collect::<VecDeque<_>>();

        let len = input_original.len();

        for to_move in input_original.iter().enumerate() {
            let start_index = values.iter().position(|v| v.0 == to_move.0).unwrap();

            // rotate the vecdeque so the number is at the start and take it out...
            values.rotate_left(start_index);
            let element = values.pop_front().unwrap();

            // ... then move the entire queue (rather than the number) and insert it again
            values.rotate_left(element.1.rem_euclid((len - 1) as isize) as usize);
            values.push_front(element);
        }

        let index_of_0 = values.iter().position(|v| v.1 == 0).unwrap();
        values[(index_of_0 + 1000) % values.len()].1
            + values[(index_of_0 + 2000) % values.len()].1
            + values[(index_of_0 + 3000) % values.len()].1
    }
}

impl SolutionGold<isize, isize> for Day {
    fn calculate_gold(input: &str) -> isize {
        const KEY: isize = 811_589_153;
        let input_original: Vec<isize> = input.lines().map(|l| l.parse().unwrap()).collect();

        // add an `enumerate` with original indices. We can't scan later because there are duplicate numbers.
        let mut values = input_original
            .iter()
            .cloned()
            .map(|v| v * KEY)
            .enumerate()
            .collect::<VecDeque<_>>();

        let len = input_original.len();

        for _ in 0..10 {
            for to_move in input_original.iter().enumerate() {
                let start_index = values.iter().position(|v| v.0 == to_move.0).unwrap();

                // rotate the vecdeque so the number is at the start and take it out...
                values.rotate_left(start_index);
                let element = values.pop_front().unwrap();

                // ... then move the entire queue (rather than the number) and insert it again
                values.rotate_left(element.1.rem_euclid((len - 1) as isize) as usize);
                values.push_front(element);
            }
        }

        let index_of_0 = values.iter().position(|v| v.1 == 0).unwrap();
        values[(index_of_0 + 1000) % values.len()].1
            + values[(index_of_0 + 2000) % values.len()].1
            + values[(index_of_0 + 3000) % values.len()].1
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(3, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(6387, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(1623178306, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(2455057187825, output);
}
