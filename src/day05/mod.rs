use tinyvec::ArrayVec;

use crate::utils::{fast_parse_int_from_bytes, split_once_2};

use super::*;

pub struct Day;

impl AocDay<String> for Day {
    const DAY: u32 = 5;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> String {
        let mut bytes = input.as_bytes();
        let mut header;
        (header, bytes) = parse_header(bytes);

        while !bytes.is_empty() {
            let mut buf;

            debug_assert_eq!(&bytes[.."move ".len()], b"move ");
            bytes = &bytes["move ".len()..];

            (buf, bytes) = split_once_2(bytes, b' ');
            let count = fast_parse_int_from_bytes(buf);

            bytes = &bytes[" from ".len()..];

            (buf, bytes) = split_once_2(bytes, b' ');
            let from = fast_parse_int_from_bytes(buf);

            bytes = &bytes[" to ".len()..];

            (buf, bytes) = split_once_2(bytes, b'\n');
            let to = fast_parse_int_from_bytes(buf);

            if !bytes.is_empty() {
                bytes = &bytes["\n".len()..];
            }

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
        let mut bytes = input.as_bytes();
        let mut header;
        (header, bytes) = parse_header(bytes);

        while !bytes.is_empty() {
            let mut buf;

            debug_assert_eq!(&bytes[.."move ".len()], b"move ");
            bytes = &bytes["move ".len()..];

            (buf, bytes) = split_once_2(bytes, b' ');
            let count = fast_parse_int_from_bytes(buf);

            bytes = &bytes[" from ".len()..];

            (buf, bytes) = split_once_2(bytes, b' ');
            let from = fast_parse_int_from_bytes(buf);

            bytes = &bytes[" to ".len()..];

            (buf, bytes) = split_once_2(bytes, b'\n');
            let to = fast_parse_int_from_bytes(buf);

            if !bytes.is_empty() {
                bytes = &bytes["\n".len()..];
            }

            let from_bucket = &mut header[from - 1];
            let from_bucket_len = from_bucket.len();
            let from_slice = from_bucket
                .drain(from_bucket_len - count..from_bucket_len)
                .collect::<ArrayVec<[u8; 64]>>();
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

fn parse_header(bytes: &[u8]) -> (ArrayVec<[ArrayVec<[u8; 64]>; 10]>, &[u8]) {
    let line_len = bytes.iter().position(|&x| x == b'\n').unwrap() + 1;
    let bucket_count = (line_len) / 4;

    let bucket_capacity = bytes
        .iter()
        .step_by(line_len)
        .position(|&c| c == b'\n')
        .unwrap()
        - 1;

    let total_len = (bucket_capacity + 1) * line_len + 1;

    let mut buckets = ArrayVec::<[ArrayVec<[u8; 64]>; 10]>::new();
    for _ in 0..bucket_count {
        buckets.push(ArrayVec::<[u8; 64]>::new());
    }

    for line_num in (0..bucket_capacity).rev() {
        let offset = line_num * line_len;
        for i in 0..bucket_count {
            let index = offset + i * 4 + 1;
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

    (buckets, &bytes[total_len..])
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
