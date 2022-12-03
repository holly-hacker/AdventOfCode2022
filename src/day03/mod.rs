use super::*;

pub struct Day;

impl AocDay<usize> for Day {
    const DAY: u32 = 3;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        input
            .split('\n')
            .map(|l| {
                debug_assert_eq!(l.len() % 2, 0);

                let bytes = l.as_bytes();
                let len = bytes.len();

                let half_1 = &bytes[..len / 2];
                let half_2 = &bytes[len / 2..];
                debug_assert_eq!(half_1.len(), half_2.len());

                // you can do these separately, but zipping them gives a tiny performance boost (3-4%).
                let (bits_1, bits_2) = half_1
                    .iter()
                    .zip(half_2.iter())
                    .fold((0u64, 0u64), |(a1, a2), (&e1, &e2)| {
                        (a1 | to_bit(e1), a2 | to_bit(e2))
                    });

                let shared_bits = bits_1 & bits_2;

                (0..64)
                    .filter(|&i| (shared_bits & 1 << i) != 0)
                    .map(|i| get_score(i | 0b0100_0000))
                    .sum::<usize>()
            })
            .sum()
    }
}

impl AocDayFull<usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        input
            .split('\n')
            .array_chunks::<3>()
            .map(|l| {
                let [bytes_1, bytes_2, bytes_3] =
                    [l[0].as_bytes(), l[1].as_bytes(), l[2].as_bytes()];

                let bits_1 = bytes_1.iter().fold(0u64, |acc, &e| acc | to_bit(e));
                let bits_2 = bytes_2.iter().fold(0u64, |acc, &e| acc | to_bit(e));
                let bits_3 = bytes_3.iter().fold(0u64, |acc, &e| acc | to_bit(e));

                let shared_bits = bits_1 & bits_2 & bits_3;

                (0..64)
                    .filter(|&i| (shared_bits & 1 << i) != 0)
                    .map(|i| get_score(i | 0b0100_0000))
                    .sum::<usize>()
            })
            .sum()
    }
}

// A: 0b0100_0000
// a: 0b0110_0000
/// Calculate the score/priority of a character.
/// ```
/// use aoc2022::day03::get_score;
/// assert_eq!(get_score(b'a'), 1);
/// assert_eq!(get_score(b'z'), 26);
/// assert_eq!(get_score(b'A'), 27);
/// assert_eq!(get_score(b'Z'), 52);
/// ```
pub fn get_score(b: u8) -> usize {
    ((b & 0x1F) + (((!b & 0b0010_0000) >> 5) * 26)) as usize
}

fn to_bit(b: u8) -> u64 {
    1 << (b & 0b0011_1111)
}

#[test]
fn test_silver_sample() {
    assert_eq!(Day::calculate_silver(Day::INPUT_SAMPLE), 157);
}

#[test]
fn test_silver_real() {
    assert_eq!(Day::calculate_silver(Day::INPUT_REAL), 7980);
}

#[test]
fn test_gold_sample() {
    assert_eq!(Day::calculate_gold(Day::INPUT_SAMPLE), 70);
}

#[test]
fn test_gold_real() {
    assert_eq!(Day::calculate_gold(Day::INPUT_REAL), 2881);
}
