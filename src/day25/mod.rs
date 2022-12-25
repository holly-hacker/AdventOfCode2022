use tinyvec::TinyVec;

use super::*;

pub struct Day;

impl SolutionSilver<String> for Day {
    const DAY: u32 = 25;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> String {
        encode_snafu(input.lines().map(decode_snafu).sum())
    }
}

// NOTE: this solution stub is here to make the benchmarking setup work. it didn't expect a fully
// solved day to not have a gold part.
impl SolutionGold<String, usize> for Day {
    fn calculate_gold(_input: &str) -> usize {
        0
    }
}

fn encode_snafu(input: isize) -> String {
    let mut buffer = TinyVec::<[char; 8]>::new();

    let mut with_offset = input;
    while with_offset != 0 {
        let char_val = ((with_offset + 2) % 5) - 2;
        let char = encode_snafu_digit(char_val) as char;

        buffer.push(char);
        with_offset -= char_val;
        with_offset /= 5;
    }

    buffer.reverse();

    let str = String::from_iter(buffer.as_slice());

    if str.is_empty() {
        "0".to_string()
    } else {
        str
    }
}

fn encode_snafu_digit(val: isize) -> u8 {
    match val {
        0..=2 => val as u8 + b'0',
        -1 => b'-',
        -2 => b'=',
        _ => panic!("out-of-bounds snafu value to convert to single digit: {val}"),
    }
}

fn decode_snafu(val: &str) -> isize {
    val.as_bytes()
        .iter()
        .rev()
        .enumerate()
        .map(|(i, b)| decode_snafu_digit(*b) * 5_isize.pow(i as u32))
        .sum()
}

fn decode_snafu_digit(digit: u8) -> isize {
    match digit {
        b'0'..=b'2' => (digit - b'0') as isize,
        b'-' => -1,
        b'=' => -2,
        _ => panic!("Unexpected snafu digit: {digit}"),
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!("2=-1=0", output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!("20-=0=02=-21=00-02=2", output);
}

#[test]
fn test_snafu_encode() {
    assert_eq!("0", encode_snafu(0));

    assert_eq!("1", encode_snafu(1));
    assert_eq!("2", encode_snafu(2));
    assert_eq!("1=", encode_snafu(3));
    assert_eq!("1-", encode_snafu(4));
    assert_eq!("10", encode_snafu(5));
    assert_eq!("11", encode_snafu(6));
    assert_eq!("12", encode_snafu(7));
    assert_eq!("2=", encode_snafu(8));
    assert_eq!("2-", encode_snafu(9));
    assert_eq!("20", encode_snafu(10));
    assert_eq!("1=0", encode_snafu(15));
    assert_eq!("1-0", encode_snafu(20));
    assert_eq!("1=11-2", encode_snafu(2022));
    assert_eq!("1-0---0", encode_snafu(12345));
    assert_eq!("1121-1110-1=0", encode_snafu(314159265));

    assert_eq!("1=-0-2", encode_snafu(1747));
    assert_eq!("12111", encode_snafu(906));
    assert_eq!("2=0=", encode_snafu(198));
    assert_eq!("21", encode_snafu(11));
    assert_eq!("2=01", encode_snafu(201));
    assert_eq!("111", encode_snafu(31));
    assert_eq!("20012", encode_snafu(1257));
    assert_eq!("112", encode_snafu(32));
    assert_eq!("1=-1=", encode_snafu(353));
    assert_eq!("1-12", encode_snafu(107));
    assert_eq!("12", encode_snafu(7));
    assert_eq!("1=", encode_snafu(3));
    assert_eq!("122", encode_snafu(37));
}

#[test]
fn test_snafu_decode() {
    assert_eq!(0, decode_snafu("0"));

    assert_eq!(1, decode_snafu("1"));
    assert_eq!(2, decode_snafu("2"));
    assert_eq!(3, decode_snafu("1="));
    assert_eq!(4, decode_snafu("1-"));
    assert_eq!(5, decode_snafu("10"));
    assert_eq!(6, decode_snafu("11"));
    assert_eq!(7, decode_snafu("12"));
    assert_eq!(8, decode_snafu("2="));
    assert_eq!(9, decode_snafu("2-"));
    assert_eq!(10, decode_snafu("20"));
    assert_eq!(15, decode_snafu("1=0"));
    assert_eq!(20, decode_snafu("1-0"));
    assert_eq!(2022, decode_snafu("1=11-2"));
    assert_eq!(12345, decode_snafu("1-0---0"));
    assert_eq!(314159265, decode_snafu("1121-1110-1=0"));

    assert_eq!(1747, decode_snafu("1=-0-2"));
    assert_eq!(906, decode_snafu("12111"));
    assert_eq!(198, decode_snafu("2=0="));
    assert_eq!(11, decode_snafu("21"));
    assert_eq!(201, decode_snafu("2=01"));
    assert_eq!(31, decode_snafu("111"));
    assert_eq!(1257, decode_snafu("20012"));
    assert_eq!(32, decode_snafu("112"));
    assert_eq!(353, decode_snafu("1=-1="));
    assert_eq!(107, decode_snafu("1-12"));
    assert_eq!(7, decode_snafu("12"));
    assert_eq!(3, decode_snafu("1="));
    assert_eq!(37, decode_snafu("122"));
}
