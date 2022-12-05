use crate::utils::fast_parse_int_from_bytes;

use super::*;

pub struct Day;

impl AocDay<String> for Day {
    const DAY: u32 = 5;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> String {
        let (header, instructions) = input.split_once("\n\n").unwrap();

        let mut header = parse_header(header);

        for line in instructions.split('\n') {
            // TODO: can be a lot more optimal!
            let mut split_iter = line.split(' ');
            split_iter.next();
            let count = fast_parse_int_from_bytes(split_iter.next().unwrap().as_bytes());
            split_iter.next();
            let from = fast_parse_int_from_bytes(split_iter.next().unwrap().as_bytes());
            split_iter.next();
            let to = fast_parse_int_from_bytes(split_iter.next().unwrap().as_bytes());

            debug_assert!(split_iter.next().is_none());

            for _ in 0..count {
                let item = header[from - 1].pop().unwrap();
                header[to - 1].push(item);
            }
        }

        let header_len = header.len();
        header
            .into_iter()
            .fold(String::with_capacity(header_len), |mut acc, mut v| {
                acc.push(v.pop().unwrap() as char);
                acc
            })
    }
}

impl AocDayFull<String> for Day {
    fn calculate_gold(input: &str) -> String {
        let (header, instructions) = input.split_once("\n\n").unwrap();

        let mut header = parse_header(header);

        for line in instructions.split('\n') {
            // TODO: can be a lot more optimal!
            let mut split_iter = line.split(' ');
            split_iter.next();
            let count = fast_parse_int_from_bytes(split_iter.next().unwrap().as_bytes());
            split_iter.next();
            let from = fast_parse_int_from_bytes(split_iter.next().unwrap().as_bytes());
            split_iter.next();
            let to = fast_parse_int_from_bytes(split_iter.next().unwrap().as_bytes());

            debug_assert!(split_iter.next().is_none());

            // TODO: very unoptimal to_vec call!
            let from_bucket = &mut header[from - 1];
            let from_bucket_len = from_bucket.len();
            let from_slice = from_bucket[from_bucket_len - count..from_bucket_len].to_vec();
            from_bucket.truncate(from_bucket_len - count);
            header[to - 1].extend_from_slice(&from_slice);
        }

        let header_len = header.len();
        header
            .into_iter()
            .fold(String::with_capacity(header_len), |mut acc, mut v| {
                acc.push(v.pop().unwrap() as char);
                acc
            })
    }
}

fn parse_header(header: &str) -> Vec<Vec<u8>> {
    let bucket_count = (header.split_once('\n').unwrap().0.len() + 1) / 4;

    let capacity = header.split('\n').count() - 1;

    let mut buckets = vec![Vec::<u8>::with_capacity(capacity); bucket_count];

    for line in header.split('\n').rev().skip(1) {
        let bytes = line.as_bytes();
        for i in 0..bucket_count {
            let index = i * 4 + 1;
            let char = bytes[index];

            if char != b' ' {
                buckets[i].push(char);

                debug_assert_eq!(bytes[index - 1], b'[');
                debug_assert_eq!(bytes[index + 1], b']');
            } else {
                debug_assert_eq!(bytes[index - 1], b' ');
                debug_assert_eq!(bytes[index + 1], b' ');
            }
        }
    }

    buckets
}

#[test]
fn test_day_5_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!("CMZ", output);
}

#[test]
fn test_day_5_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!("QGTHFZBHV", output);
}

#[test]
fn test_day_5_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!("MCD", output);
}

#[test]
fn test_day_5_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!("MGDMPSZTM", output);
}
