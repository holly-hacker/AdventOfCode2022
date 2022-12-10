use crate::utils::{fast_parse_int_from_bytes, split_once_2};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 7;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        debug_assert_eq!(input.lines().next(), Some("$ cd /"));
        let mut iterator = input.as_bytes().split(|&x| x == b'\n');
        let (_, sum_under_100000) = walk_directory_silver(&mut iterator);
        sum_under_100000
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        const DISK_SIZE_MAX_IN_USE: usize = 70_000_000 - 30_000_000;

        debug_assert_eq!(input.lines().next(), Some("$ cd /"));
        let mut iterator = input.as_bytes().split(|&x| x == b'\n');
        let mut vec = vec![];
        let size_in_use = walk_directory_gold(&mut iterator, &mut vec);
        debug_assert_eq!(*vec.iter().max().unwrap(), size_in_use);

        let size_over_limit = size_in_use - DISK_SIZE_MAX_IN_USE;
        *vec.iter().filter(|&&x| x > size_over_limit).min().unwrap()
    }
}

fn walk_directory_silver<'a>(
    remaining_input: &mut impl Iterator<Item = &'a [u8]>,
) -> (usize, usize) {
    let mut my_size = 0;
    let mut sum_under_100000 = 0;
    loop {
        let Some(line) = remaining_input.next() else {
            return (my_size, sum_under_100000);
        };

        if line.starts_with(b"$ cd") {
            if line[5] == b'.' {
                return (my_size, sum_under_100000);
            }

            let (child_size, child_over_100000) = walk_directory_silver(remaining_input);
            sum_under_100000 += child_over_100000;
            if child_size <= 100000 {
                sum_under_100000 += child_size;
            }
            my_size += child_size;
        } else if !line.starts_with(b"dir ") && !line.starts_with(b"$") {
            // file
            let (size, _) = split_once_2(line, b' ');
            let size = fast_parse_int_from_bytes(size);
            my_size += size;
        }
    }
}

fn walk_directory_gold<'a>(
    remaining_input: &mut impl Iterator<Item = &'a [u8]>,
    vec: &mut Vec<usize>,
) -> usize {
    let mut my_size = 0;
    loop {
        let Some(line) = remaining_input.next() else {
            vec.push(my_size);
            return my_size;
        };

        if line.starts_with(b"$ cd") {
            if line[5] == b'.' {
                vec.push(my_size);
                return my_size;
            }
            let child_size = walk_directory_gold(remaining_input, vec);
            my_size += child_size;
        } else if !line.starts_with(b"dir ") && !line.starts_with(b"$") {
            // file
            let (size, _) = split_once_2(line, b' ');
            let size = fast_parse_int_from_bytes(size);
            my_size += size;
        }
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(95437, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(1555642, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(24933642, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(5974547, output);
}

// 45349983 too high
