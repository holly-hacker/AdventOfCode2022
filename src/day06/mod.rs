use super::*;

pub struct Day;

impl AocDay<usize> for Day {
    const DAY: u32 = 6;

    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");

    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut bit_buffers = [0u32; 4];
        let mut bit_buffer_index = 0;

        for b in input.as_bytes() {
            let bit = 1 << (b - b'a');
            bit_buffers[bit_buffer_index % 4] = bit;

            bit_buffer_index += 1;

            let sum = bit_buffers[0] | bit_buffers[1] | bit_buffers[2] | bit_buffers[3];
            if sum.count_ones() == 4 {
                return bit_buffer_index;
            }
        }

        unreachable!("no pattern found");
    }
}

impl AocDayFull<usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut bit_buffers = [0u32; 14];
        let mut bit_buffer_index = 0;

        for b in input.as_bytes() {
            let bit = 1 << (b - b'a');
            bit_buffers[bit_buffer_index % 14] = bit;

            bit_buffer_index += 1;

            let sum = bit_buffers.iter().fold(0, |acc, v| acc | v);
            if sum.count_ones() == 14 {
                return bit_buffer_index;
            }
        }

        unreachable!("no pattern found");
    }
}

#[test]
fn test_day_6_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(7, output);
}

#[test]
fn test_day_6_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(1175, output);
}

#[test]
fn test_day_6_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(19, output);
}

#[test]
fn test_day_6_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(3217, output);
}
