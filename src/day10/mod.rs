use crate::utils::{fast_parse_int_from_bytes, split_once};

use super::*;

pub struct Day;

impl SolutionSilver<isize> for Day {
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

const WIDTH: usize = 40;
const HEIGHT: usize = 6;
impl SolutionGold<isize, CrtDisplay<240, 40>> for Day {
    fn calculate_gold(input: &str) -> CrtDisplay<240, 40> {
        let mut cycle = 0usize;
        let mut x_register = 1isize;
        let mut display = CrtDisplay::<{ WIDTH * HEIGHT }, WIDTH>([false; WIDTH * HEIGHT]);

        let mut bytes = input.as_bytes();

        let draw = |display: &mut CrtDisplay<240, 40>, cycle: usize, x: isize| {
            let abs_diff = ((cycle % WIDTH) as isize).abs_diff(x);
            if abs_diff == 0 || abs_diff == 1 {
                display.0[cycle % (WIDTH * HEIGHT)] = true;
            }
        };

        while !bytes.is_empty() {
            let instruction;
            (instruction, bytes) = bytes.split_at(4);

            if instruction[0] == b'n' {
                // noop
                draw(&mut display, cycle, x_register);
                cycle += 1;

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

                draw(&mut display, cycle, x_register);
                cycle += 1;

                draw(&mut display, cycle, x_register);
                cycle += 1;

                x_register += operand;

                debug_assert!(bytes.is_empty() || bytes[0] != b'\n');
            }
        }

        display
    }
}

pub struct CrtDisplay<const SIZE: usize, const STRIDE: usize>([bool; SIZE]);

impl<const SIZE: usize, const STRIDE: usize> std::fmt::Debug for CrtDisplay<SIZE, STRIDE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_assert_eq!(SIZE % STRIDE, 0);

        for i in 0..SIZE {
            if i % STRIDE == 0 {
                writeln!(f)?;
            }

            let val = self.0[i];
            write!(f, "{}", if val { '#' } else { ' ' })?;
        }

        Ok(())
    }
}

impl<const SIZE: usize, const STRIDE: usize> Display for CrtDisplay<SIZE, STRIDE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_assert_eq!(SIZE % STRIDE, 0);
        let height = SIZE / STRIDE;
        let lines_required = (height - 1) / 4 + 1;

        if lines_required > 1 {
            writeln!(f, "<<<")?;
        }

        // currently assumes an even stride
        debug_assert!(STRIDE % 2 == 0);
        for line in 0..lines_required {
            // see https://en.wikipedia.org/wiki/File:Braille8dotCellNumbering.svg
            for line_index in 0..((STRIDE + 1) / 2) {
                const BRAILLE_ZERO: u32 = '⠀' as u32;
                let get_pos = |x: usize, y: usize| ((line * 4 + y) * STRIDE) + (line_index * 2 + x);

                let bit_0 = (*self.0.get(get_pos(0, 0)).unwrap_or(&false) as u8) << 0;
                let bit_1 = (*self.0.get(get_pos(0, 1)).unwrap_or(&false) as u8) << 1;
                let bit_2 = (*self.0.get(get_pos(0, 2)).unwrap_or(&false) as u8) << 2;
                let bit_3 = (*self.0.get(get_pos(1, 0)).unwrap_or(&false) as u8) << 3;
                let bit_4 = (*self.0.get(get_pos(1, 1)).unwrap_or(&false) as u8) << 4;
                let bit_5 = (*self.0.get(get_pos(1, 2)).unwrap_or(&false) as u8) << 5;
                let bit_6 = (*self.0.get(get_pos(0, 3)).unwrap_or(&false) as u8) << 6;
                let bit_7 = (*self.0.get(get_pos(1, 3)).unwrap_or(&false) as u8) << 7;

                let offset = bit_0 | bit_1 | bit_2 | bit_3 | bit_4 | bit_5 | bit_6 | bit_7;

                let character = char::from_u32(BRAILLE_ZERO + offset as u32).unwrap();
                write!(f, "{}", character)?;
            }

            if lines_required > 1 {
                writeln!(f)?;
            }
        }

        if lines_required > 1 {
            write!(f, ">>>")?;
        }

        Ok(())
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

#[test]
fn test_gold_sample() {
    const EXPECTED: &str = "<<<
⣿⣦⡉⠒⠯⣤⣛⡂⠭⠶⣋⣀⡿⠦⠉⣒⣯⡤⠛⠂
⠛⠛⠛⠂⠀⠀⠉⠛⠛⠒⠂⠀⠉⠉⠛⠒⠒⠂⠉⠉
>>>";
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(EXPECTED, output.to_string());
}

#[test]
fn test_gold_real() {
    const EXPECTED: &str = "<<<
⡯⠍⢸⣉⠆⡎⣑⢸⣉⠆⡧⢼⢸⠭⠁⡎⣑⢸⠤⡇
⠃⠀⠘⠀⠀⠑⠚⠘⠀⠀⠃⠘⠘⠀⠀⠑⠚⠘⠀⠃
>>>";
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(EXPECTED, output.to_string());
}

#[test]
fn test_crt_display() {
    assert_eq!("⠀", format!("{}", CrtDisplay::<8, 2>([false; 8])));
    assert_eq!("⣿", format!("{}", CrtDisplay::<8, 2>([true; 8])));
    assert_eq!("⠿", format!("{}", CrtDisplay::<6, 2>([true; 6])));
    assert_eq!(
        "⢩",
        format!(
            "{}",
            CrtDisplay::<8, 2>([true, true, false, false, false, true, false, true])
        )
    );
    assert_eq!(
        "⠑⢄",
        format!(
            "{}",
            CrtDisplay::<16, 4>([
                true, false, false, false, false, true, false, false, false, false, true, false,
                false, false, false, true,
            ])
        )
    );
    assert_eq!(
        "<<<\n⢕\n⡪\n>>>",
        format!(
            "{}",
            CrtDisplay::<16, 2>([
                true, false, false, true, true, false, false, true, false, true, true, false,
                false, true, true, false,
            ])
        )
    );
}
