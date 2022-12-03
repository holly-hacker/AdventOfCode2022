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
                let bytes = l.as_bytes();
                let len = bytes.len();
                debug_assert_eq!(l.len() % 2, 0);

                let half_1 = &bytes[..len / 2];
                let half_2 = &bytes[len / 2..];
                debug_assert_eq!(half_1.len(), half_2.len());

                let mut used = vec![];
                half_1
                    .into_iter()
                    .filter(|b| {
                        let contains = half_2.contains(b) && !used.contains(b);
                        if contains {
                            used.push(b);
                        }
                        contains
                    })
                    .map(|&c| get_score(c))
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
                let [half_1, half_2, half_3] = l;
                let [half_1, half_2, half_3] =
                    [half_1.as_bytes(), half_2.as_bytes(), half_3.as_bytes()];

                let mut used = vec![];
                half_1
                    .into_iter()
                    .filter(|b| {
                        let contains =
                            half_2.contains(b) && half_3.contains(b) && !used.contains(b);
                        if contains {
                            used.push(b);
                        }
                        contains
                    })
                    .map(|&c| get_score(c))
                    .sum::<usize>()
            })
            .sum()
    }
}

fn get_score(b: u8) -> usize {
    (match b {
        b'a'..=b'z' => b - b'a' + 1,
        b'A'..=b'Z' => b - b'A' + 1 + 26,
        _ => panic!(),
    }) as usize
}

#[test]
fn test_silver_sample() {
    assert_eq!(Day::calculate_silver(Day::INPUT_SAMPLE), 157)
}

#[test]
fn test_silver_real() {
    assert_eq!(Day::calculate_silver(Day::INPUT_REAL), 7980)
}

#[test]
fn test_gold_sample() {
    assert_eq!(Day::calculate_gold(Day::INPUT_SAMPLE), 70)
}

#[test]
fn test_gold_real() {
    assert_eq!(Day::calculate_gold(Day::INPUT_REAL), 2881)
}
