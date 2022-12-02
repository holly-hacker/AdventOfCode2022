use super::*;

pub struct Day;

impl AocDay<usize> for Day {
    const DAY: u32 = 1;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        input
            .split('\n')
            .map(|line| {
                let (other, mine) = line.split_once(' ').unwrap();
                let (other, mine) = (
                    ((other.bytes().next().unwrap() - b'A') as usize),
                    ((mine.bytes().next().unwrap() - b'X') as usize),
                );

                // rock=0, paper=1, scissor=2
                let win_mod = match (other + 3 - mine) % 3 {
                    0 => 3usize, // same play, draw
                    2 => 6usize, // one more, meaning win
                    1 => 0usize, // one less, meaning loss
                    _ => unreachable!(),
                };

                let play_mod = match mine {
                    0 => 1, // rock
                    1 => 2, // paper
                    2 => 3, // scissor
                    _ => unreachable!(),
                };

                play_mod + win_mod
            })
            .sum()
    }
}

impl AocDayFull<usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        input
            .split('\n')
            .map(|line| {
                let (other, mine) = line.split_once(' ').unwrap();
                let (other, mine) = (
                    ((other.bytes().next().unwrap() - b'A') as usize),
                    ((mine.bytes().next().unwrap() - b'X') as usize),
                );

                // `mine` is the outcome, not our play. convert it
                // 0 -> paper to rock (-1, or +2)
                // 1 -> same as other (+0)
                // 2 -> +1
                let mine = (other + mine + 3 - 1) % 3;

                // rock=0, paper=1, scissor=2
                let win_mod = match (other + 3 - mine) % 3 {
                    0 => 3usize, // same play, draw
                    2 => 6usize, // one more, meaning win
                    1 => 0usize, // one less, meaning loss
                    _ => unreachable!(),
                };

                let play_mod = match mine {
                    0 => 1, // rock
                    1 => 2, // paper
                    2 => 3, // scissor
                    _ => unreachable!(),
                };

                play_mod + win_mod
            })
            .sum()
    }
}

#[test]
fn test_sample_silver() {
    assert_eq!(15, Day::calculate_silver(Day::INPUT_SAMPLE));
}

#[test]
fn test_real_silver() {
    assert_eq!(13484, Day::calculate_silver(Day::INPUT_REAL));
}

#[test]
fn test_sample_gold() {
    assert_eq!(12, Day::calculate_gold(Day::INPUT_SAMPLE));
}

#[test]
fn test_real_gold() {
    assert_eq!(13433, Day::calculate_gold(Day::INPUT_REAL));
}
