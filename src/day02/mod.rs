use super::*;

pub struct Day;

impl AocDay<usize> for Day {
    const DAY: u32 = 1;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        debug_assert!(input.as_bytes()[3] == b'\n');
        debug_assert!(!input.ends_with('\n'));

        let bytes = input.as_bytes();
        bytes
            .array_chunks::<4>()
            .map(|line| {
                debug_assert!(line[1] == b' ');
                debug_assert!(line[3] == b'\n');

                let (other, mine) = (line[0], line[2]);
                let (other, mine) = ((other - b'A'), (mine - b'X'));

                calculate_score(other, mine)
            })
            .sum::<usize>()
            + {
                let other = bytes[bytes.len() - 3] - b'A';
                let mine = bytes[bytes.len() - 1] - b'X';
                calculate_score(other, mine)
            }
    }
}

impl AocDayFull<usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        debug_assert!(input.as_bytes()[3] == b'\n');
        debug_assert!(!input.ends_with('\n'));

        let bytes = input.as_bytes();
        bytes
            .array_chunks::<4>()
            .map(|line| {
                debug_assert!(line[1] == b' ');
                debug_assert!(line[3] == b'\n');

                let (other, mine) = (line[0], line[2]);
                let (other, mine) = ((other - b'A'), (mine - b'X'));

                // `mine` is the outcome, not our play. convert it
                // 0 -> paper to rock (-1, or +2)
                // 1 -> same as other (+0)
                // 2 -> +1
                debug_assert!(mine < 3);
                let mine = (other + mine + 3 - 1) % 3;

                calculate_score(other, mine)
            })
            .sum::<usize>()
            + {
                let other = bytes[bytes.len() - 3] - b'A';
                let mine = bytes[bytes.len() - 1] - b'X';
                let mine = (other + mine + 3 - 1) % 3;
                calculate_score(other, mine)
            }
    }
}

#[inline]
fn calculate_score(other: u8, mine: u8) -> usize {
    debug_assert!(other < 3);
    debug_assert!(mine < 3);

    // rock=0, paper=1, scissor=2
    let win_mod = match (other + 3 - mine) % 3 {
        0 => 3, // same play, draw
        2 => 6, // one more, meaning win
        1 => 0, // one less, meaning loss
        _ => unreachable!(),
    };

    let play_mod = match mine {
        0 => 1, // rock
        1 => 2, // paper
        2 => 3, // scissor
        _ => unreachable!(),
    };

    play_mod + win_mod
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
