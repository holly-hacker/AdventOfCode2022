use crate::utils::{fast_parse_int_from_bytes, split_once};

use super::*;

pub struct Day;

impl AocDay<isize> for Day {
    const DAY: u32 = 10;

    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> isize {
        let mut cycle = 0;
        let mut x_register = 1;
        let mut signal_strength = 0;

        let mut bytes = input.as_bytes();
        while !bytes.is_empty() {
            let instruction;
            (instruction, bytes) = bytes.split_at(4);

            if instruction[0] == b'n' {
                // noop
                cycle += 1;

                if ((cycle - 20) % 40) == 0 {
                    signal_strength += cycle * x_register;
                }

                if !bytes.is_empty() {
                    debug_assert_eq!(bytes[0], b'\n');
                    bytes = &bytes[1..]; // skip newline
                }
            } else {
                // addx
                let operand;
                bytes = &bytes[1..]; // skip space
                (operand, bytes) = split_once(bytes, b'\n').unwrap_or((bytes, b""));

                let is_negative = operand[0] == b'-';

                let operand = if is_negative {
                    -(fast_parse_int_from_bytes(&operand[1..]) as isize)
                } else {
                    fast_parse_int_from_bytes(operand) as isize
                };

                cycle += 1;

                if ((cycle - 20) % 40) == 0 {
                    signal_strength += cycle * x_register;
                }

                cycle += 1;

                if ((cycle - 20) % 40) == 0 {
                    signal_strength += cycle * x_register;
                }

                x_register += operand;

                debug_assert!(bytes.is_empty() || bytes[0] != b'\n');
            }
        }

        signal_strength
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(13140, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(10760, output);
}
