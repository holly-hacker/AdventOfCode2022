use crate::utils::fast_parse_int;

use super::*;

pub struct Day;

impl AocDay<usize> for Day {
    const DAY: u32 = 7;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        debug_assert_eq!(input.lines().next(), Some("$ cd /"));
        let mut iterator = input.lines().skip(1);
        let (_, ret) = walk_directory_silver(&mut iterator);
        ret
    }
}

impl AocDayFull<usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        const DISK_SIZE_MAX_IN_USE: usize = 70_000_000 - 30_000_000;

        debug_assert_eq!(input.lines().next(), Some("$ cd /"));
        let mut iterator = input.lines().skip(1);
        let mut vec = vec![];
        let size_in_use = walk_directory_gold(&mut iterator, &mut vec);
        let size_over_limit = size_in_use - DISK_SIZE_MAX_IN_USE;
        vec.sort();
        debug_assert_eq!(*vec.last().unwrap(), size_in_use);
        let partition_point = vec.partition_point(|&x| x < size_over_limit);
        vec[partition_point - 0]
    }
}

fn walk_directory_silver<'a>(
    remaining_input: &mut impl Iterator<Item = &'a str>,
) -> (usize, usize) {
    let mut size_this_dir = 0;
    let mut size_children = 0;
    let mut sum_over_100000 = 0;
    loop {
        let Some(line) = remaining_input.next() else {
            return (size_this_dir + size_children, sum_over_100000);
        };
        debug_assert_ne!(line, "$ cd /");

        if line == "$ ls" {
            continue;
        } else if line.starts_with("$ cd ..") {
            return (size_this_dir + size_children, sum_over_100000);
        } else if line.starts_with("$ cd") {
            let (child_size, child_over_100000) = walk_directory_silver(remaining_input);
            sum_over_100000 += child_over_100000;
            if child_size <= 100000 {
                sum_over_100000 += child_size;
            }
            size_children += child_size;
        } else if line.starts_with("dir") {
            // do nothing
        } else {
            debug_assert!(!line.starts_with("$"));
            let (size, _) = line.split_once(' ').unwrap();
            let size = fast_parse_int(size);
            size_this_dir += size;
        }
    }
}

fn walk_directory_gold<'a>(
    remaining_input: &mut impl Iterator<Item = &'a str>,
    vec: &mut Vec<usize>,
) -> usize {
    let mut my_size = 0;
    loop {
        let Some(line) = remaining_input.next() else {
            vec.push(my_size);
            return my_size;
        };
        debug_assert_ne!(line, "$ cd /");

        if line == "$ ls" {
            continue;
        } else if line.starts_with("$ cd ..") {
            vec.push(my_size);
            return my_size;
        } else if line.starts_with("$ cd") {
            let child_size = walk_directory_gold(remaining_input, vec);
            my_size += child_size;
        } else if line.starts_with("dir") {
            // do nothing
        } else {
            debug_assert!(!line.starts_with("$"));
            let (size, _) = line.split_once(' ').unwrap();
            let size = fast_parse_int(size);
            my_size += size;
        }
    }
}

#[test]
fn test_day_6_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(95437, output);
}

#[test]
fn test_day_6_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(1555642, output);
}

#[test]
fn test_day_6_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(24933642, output);
}

#[test]
fn test_day_6_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(5974547, output);
}

// 45349983 too high
