use std::ops::ControlFlow;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 6;

    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");

    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        calculate_bitwise_golf::<4>(input.as_bytes())
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        calculate_bitwise_skip::<14>(input.as_bytes())
    }
}

#[allow(unused)]
fn calculate_bitwise<const SIZE: usize>(bytes: &[u8]) -> usize {
    let mut bit_buffers = [0u32; SIZE];
    let mut bit_buffer_index = 0;

    for b in bytes {
        let bit = 1 << (b - b'a');
        bit_buffers[bit_buffer_index % SIZE] = bit;

        bit_buffer_index += 1;

        let sum = bit_buffers.iter().fold(0, |acc, v| acc | v);
        if sum.count_ones() == SIZE as u32 {
            return bit_buffer_index;
        }
    }

    unreachable!("no pattern found");
}

#[allow(unused)]
fn calculate_bitwise_golf<const SIZE: usize>(bytes: &[u8]) -> usize {
    bytes
        .array_windows::<SIZE>()
        .map(|w| w.iter().fold(0, |a, v| a | (1u32 << (v - b'a'))))
        .position(|v| v.count_ones() as usize == SIZE)
        .unwrap()
        + SIZE
}

#[allow(unused)]
fn calculate_bitwise_skip<const SIZE: usize>(bytes: &[u8]) -> usize {
    let mut index = SIZE;

    while index < bytes.len() {
        let cflow = ((index - SIZE)..(index)).rev().try_fold(0, |bits, i| {
            let new_bit = 1u32 << (bytes[i] - b'a');
            let new_bits = bits | new_bit;
            let is_original = bits != new_bits; // whether this is a char we didnt have yet
            if is_original {
                ControlFlow::Continue(new_bits)
            } else {
                ControlFlow::Break(new_bits)
            }
        });

        match cflow {
            ControlFlow::Continue(x) => return index,
            ControlFlow::Break(bits) => {
                let count_to_skip = SIZE - (bits.count_ones() as usize);
                index += count_to_skip;
            }
        }
    }

    unreachable!("no pattern found");
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(7, output);
}

#[test]
fn test_silver_extra_samples() {
    assert_eq!(5, Day::calculate_silver("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(6, Day::calculate_silver("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(
        10,
        Day::calculate_silver("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
    );
    assert_eq!(
        11,
        Day::calculate_silver("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
    );
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(1175, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(19, output);
}

#[test]
fn test_gold_extra_samples() {
    assert_eq!(23, Day::calculate_gold("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(23, Day::calculate_gold("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(29, Day::calculate_gold("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(26, Day::calculate_gold("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(3217, output);
}
