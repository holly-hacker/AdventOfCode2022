use crate::utils::{fast_parse_int_from_bytes, split_once, split_once_2};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 11;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut monkeys = Monkey::parse_input(input.as_bytes());

        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let mut monkey = monkeys[i].clone();
                monkey.inspect_count += monkey.items.len();
                for item in monkey.items.drain(..) {
                    let evaluated = monkey.operation.evaluate(item);
                    let evaluated = evaluated / 3;
                    let divided = evaluated % monkey.division_check;
                    let target_monkey = if divided == 0 {
                        monkey.target_true
                    } else {
                        monkey.target_false
                    };

                    debug_assert_ne!(i, target_monkey); // the monkey at `i` will be overridden later
                    monkeys[target_monkey].items.push(evaluated);
                }

                // insert monkey back in
                monkeys[i] = monkey;
            }
        }

        let (max1, max2) = monkeys.into_iter().fold((0, 0), |mut acc, m| {
            if m.inspect_count > acc.0 {
                acc.1 = acc.0;
                acc.0 = m.inspect_count;
            } else if m.inspect_count > acc.1 {
                acc.1 = m.inspect_count;
            }

            (acc.0, acc.1)
        });

        max1 * max2
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut monkeys = Monkey::parse_input(input.as_bytes());

        let gcd: usize = monkeys.iter().map(|m| m.division_check).product();

        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                let mut monkey = monkeys[i].clone();
                monkey.inspect_count += monkey.items.len();
                for item in monkey.items.drain(..) {
                    let evaluated = monkey.operation.evaluate(item);
                    let evaluated = evaluated % gcd;
                    let divided = evaluated % monkey.division_check;
                    let target_monkey = if divided == 0 {
                        monkey.target_true
                    } else {
                        monkey.target_false
                    };

                    debug_assert_ne!(i, target_monkey); // the monkey at `i` will be overridden later
                    monkeys[target_monkey].items.push(evaluated);
                }

                // insert monkey back in
                monkeys[i] = monkey;
            }
        }

        let (max1, max2) = monkeys.into_iter().fold((0, 0), |mut acc, m| {
            if m.inspect_count > acc.0 {
                acc.1 = acc.0;
                acc.0 = m.inspect_count;
            } else if m.inspect_count > acc.1 {
                acc.1 = m.inspect_count;
            }

            (acc.0, acc.1)
        });

        max1 * max2
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    division_check: usize,
    target_true: usize,
    target_false: usize,
    inspect_count: usize,
}

impl Monkey {
    pub fn parse_input(mut bytes: &[u8]) -> Vec<Monkey> {
        let mut monkeys = vec![]; // TODO: arrayvec

        // parse all monkeys
        loop {
            debug_assert_eq!(&bytes[.."Monkey ".len()], b"Monkey ");
            debug_assert_eq!((bytes["Monkey ".len()] - b'0') as usize, monkeys.len());
            let monkey;
            (monkey, bytes) = Monkey::parse(bytes);
            monkeys.push(monkey);

            if bytes.is_empty() {
                return monkeys;
            }

            bytes = &bytes[2..]; // skip 2 newlines
        }
    }

    pub fn parse(mut bytes: &[u8]) -> (Monkey, &[u8]) {
        debug_assert!(&bytes[0..7] == b"Monkey ");
        bytes = &bytes[(12 + "Starting items".len())..];
        let mut nums;
        (nums, bytes) = split_once(bytes, b'\n').unwrap();

        let mut items = vec![];
        let mut num_bytes;
        while !nums.is_empty() {
            nums = &nums[1..];
            (num_bytes, nums) = split_once_2(nums, b',');
            num_bytes = &num_bytes[1..];

            items.push(fast_parse_int_from_bytes(num_bytes));
        }

        debug_assert!(&bytes[0..13] == b"  Operation: ");
        bytes = &bytes["  Operation: new = old ".len()..];
        let op_bytes;
        (op_bytes, bytes) = split_once(bytes, b'\n').unwrap();
        let operation = Operation::parse(op_bytes);

        debug_assert!(&bytes[0..6] == b"  Test");
        bytes = &bytes["  Test: divisible by ".len()..];
        let num_bytes;
        (num_bytes, bytes) = split_once(bytes, b'\n').unwrap();
        let division_check = fast_parse_int_from_bytes(num_bytes);

        debug_assert!(&bytes[0..12] == b"    If true:");
        let target_true = (bytes["    If true: throw to monkey ".len()] - b'0') as usize;
        bytes = &bytes["    If true: throw to monkey N\n".len()..];

        debug_assert!(&bytes[0..13] == b"    If false:");
        let target_false = (bytes["    If false: throw to monkey ".len()] - b'0') as usize;
        bytes = &bytes["    If false: throw to monkey N".len()..];

        (
            Monkey {
                items,
                operation,
                division_check,
                target_true,
                target_false,
                inspect_count: 0,
            },
            bytes,
        )
    }
}

#[derive(Clone)]
enum Operation {
    AddConstant(usize),
    MultiplyConstant(usize),
    Square,
}

impl Operation {
    pub fn parse(bytes: &[u8]) -> Self {
        let is_mult = bytes[0] == b'*';
        let on_self = bytes[2] == b'o';

        match (is_mult, on_self) {
            (false, false) => Self::AddConstant(fast_parse_int_from_bytes(&bytes[2..])),
            (true, false) => Self::MultiplyConstant(fast_parse_int_from_bytes(&bytes[2..])),
            (false, true) => Self::MultiplyConstant(2),
            (true, true) => Self::Square,
        }
    }

    fn evaluate(&self, item: usize) -> usize {
        match self {
            Operation::AddConstant(c) => item + c,
            Operation::MultiplyConstant(c) => item * c,
            Operation::Square => item * item,
        }
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(10605, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(62491, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(2713310158, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(17408399184, output);
}
