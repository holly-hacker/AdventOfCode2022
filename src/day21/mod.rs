use ahash::AHashMap;

use crate::utils::fast_parse_int_from_bytes;

use super::*;

pub struct Day;

impl SolutionSilver<isize> for Day {
    const DAY: u32 = 21;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> isize {
        let hashmap = parse_input(input);
        let root = u32::from_ne_bytes([b'r', b'o', b'o', b't']);

        solve_recursively(&hashmap, root)
    }
}

impl SolutionGold<isize, isize> for Day {
    fn calculate_gold(input: &str) -> isize {
        let map = parse_input(input);
        let root = u32::from_ne_bytes([b'r', b'o', b'o', b't']);

        let (a, b) = match map[&root] {
            Operation::Constant(_) => unreachable!("root should not be constant"),
            Operation::Add(a, b) => (a, b),
            Operation::Sub(a, b) => (a, b),
            Operation::Mul(a, b) => (a, b),
            Operation::Div(a, b) => (a, b),
        };

        let a = solve_recursively_inverse(&map, a);
        let b = solve_recursively_inverse(&map, b);

        match (a, b) {
            (InverseResult::Value(mut val), InverseResult::InverseOperations(ops))
            | (InverseResult::InverseOperations(ops), InverseResult::Value(mut val)) => {
                for op in ops.iter().rev() {
                    match op {
                        SimpleOperation::Add(c) => val += c,
                        SimpleOperation::Sub(c) => val -= c,
                        SimpleOperation::Mul(c) => val *= c,
                        SimpleOperation::Div(c) => val /= c,
                        SimpleOperation::Neg => val = -val,
                        SimpleOperation::Inv => val = 1 / val,
                    }
                }

                val
            }
            _ => unreachable!(),
        }
    }
}

fn solve_recursively(map: &AHashMap<u32, Operation>, name: u32) -> isize {
    match map[&name] {
        Operation::Constant(x) => x,
        Operation::Add(a, b) => solve_recursively(map, a) + solve_recursively(map, b),
        Operation::Sub(a, b) => solve_recursively(map, a) - solve_recursively(map, b),
        Operation::Mul(a, b) => solve_recursively(map, a) * solve_recursively(map, b),
        Operation::Div(a, b) => solve_recursively(map, a) / solve_recursively(map, b),
    }
}

fn solve_recursively_inverse(map: &AHashMap<u32, Operation>, name: u32) -> InverseResult {
    let humn = u32::from_ne_bytes([b'h', b'u', b'm', b'n']);

    debug_assert_ne!(name, u32::from_ne_bytes([b'r', b'o', b'o', b't']));

    match map[&name] {
        Operation::Constant(_) if name == humn => InverseResult::InverseOperations(vec![]),
        Operation::Constant(x) => InverseResult::Value(x),
        Operation::Add(a, b) => {
            let a = solve_recursively_inverse(map, a);
            let b = solve_recursively_inverse(map, b);

            match (a, b) {
                (InverseResult::Value(a), InverseResult::Value(b)) => InverseResult::Value(a + b),
                (InverseResult::Value(c), InverseResult::InverseOperations(mut ops)) => {
                    // X = c + H -> H = X - c
                    ops.push(SimpleOperation::Sub(c));
                    InverseResult::InverseOperations(ops)
                }
                (InverseResult::InverseOperations(mut ops), InverseResult::Value(c)) => {
                    // X = H + c -> H = X - c
                    ops.push(SimpleOperation::Sub(c));
                    InverseResult::InverseOperations(ops)
                }
                (InverseResult::InverseOperations(_), InverseResult::InverseOperations(_)) => {
                    unreachable!("there should only be 1 human");
                }
            }
        }
        Operation::Sub(a, b) => {
            let a = solve_recursively_inverse(map, a);
            let b = solve_recursively_inverse(map, b);

            match (a, b) {
                (InverseResult::Value(a), InverseResult::Value(b)) => InverseResult::Value(a - b),
                (InverseResult::Value(c), InverseResult::InverseOperations(mut ops)) => {
                    // X = c - H -> H = -X + c
                    ops.push(SimpleOperation::Add(c));
                    ops.push(SimpleOperation::Neg);
                    InverseResult::InverseOperations(ops)
                }
                (InverseResult::InverseOperations(mut ops), InverseResult::Value(c)) => {
                    // X = H - c -> H = X + c
                    ops.push(SimpleOperation::Add(c));
                    InverseResult::InverseOperations(ops)
                }
                (InverseResult::InverseOperations(_), InverseResult::InverseOperations(_)) => {
                    unreachable!("there should only be 1 human");
                }
            }
        }
        Operation::Mul(a, b) => {
            let a = solve_recursively_inverse(map, a);
            let b = solve_recursively_inverse(map, b);

            match (a, b) {
                (InverseResult::Value(a), InverseResult::Value(b)) => InverseResult::Value(a * b),
                (InverseResult::Value(c), InverseResult::InverseOperations(mut ops)) => {
                    // X = c * H -> H = X / c
                    ops.push(SimpleOperation::Div(c));
                    InverseResult::InverseOperations(ops)
                }
                (InverseResult::InverseOperations(mut ops), InverseResult::Value(c)) => {
                    // X = H * c -> H = X / c
                    ops.push(SimpleOperation::Div(c));
                    InverseResult::InverseOperations(ops)
                }
                (InverseResult::InverseOperations(_), InverseResult::InverseOperations(_)) => {
                    unreachable!("there should only be 1 human");
                }
            }
        }
        Operation::Div(a, b) => {
            let a = solve_recursively_inverse(map, a);
            let b = solve_recursively_inverse(map, b);

            match (a, b) {
                (InverseResult::Value(a), InverseResult::Value(b)) => InverseResult::Value(a / b),
                (InverseResult::Value(c), InverseResult::InverseOperations(mut ops)) => {
                    // X = c / H -> H = 1 / X * c
                    // NOTE: `inv` is extremely lossy on ints! we should not be doing this!
                    println!("Adding inv");
                    ops.push(SimpleOperation::Mul(c));
                    ops.push(SimpleOperation::Inv);
                    InverseResult::InverseOperations(ops)
                }
                (InverseResult::InverseOperations(mut ops), InverseResult::Value(c)) => {
                    // X = H / c -> H = X * c
                    ops.push(SimpleOperation::Mul(c));
                    InverseResult::InverseOperations(ops)
                }
                (InverseResult::InverseOperations(_), InverseResult::InverseOperations(_)) => {
                    unreachable!("there should only be 1 human");
                }
            }
        }
    }
}

fn parse_input(input: &str) -> AHashMap<u32, Operation> {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let name = u32::from_ne_bytes(*bytes.array_chunks::<4>().next().unwrap());
            let has_num = (bytes[6] as char).is_ascii_digit();
            debug_assert_ne!(bytes[6], b'-'); // sanity check

            let operation = if has_num {
                Operation::Constant(fast_parse_int_from_bytes(&bytes[6..]) as isize)
            } else {
                let name1 = u32::from_ne_bytes(*(bytes[6..].array_chunks::<4>().next().unwrap()));
                let name2 = u32::from_ne_bytes(*(bytes[13..].array_chunks::<4>().next().unwrap()));
                let operation_char = bytes[11];
                match operation_char {
                    b'+' => Operation::Add(name1, name2),
                    b'-' => Operation::Sub(name1, name2),
                    b'*' => Operation::Mul(name1, name2),
                    b'/' => Operation::Div(name1, name2),
                    _ => unreachable!(),
                }
            };

            (name, operation)
        })
        .collect()
}

#[derive(Debug)]
enum Operation {
    Constant(isize),
    Add(u32, u32),
    Sub(u32, u32),
    Mul(u32, u32),
    Div(u32, u32),
}

#[derive(Debug)]
enum InverseResult {
    Value(isize),
    InverseOperations(Vec<SimpleOperation>),
}

#[derive(Debug)]
enum SimpleOperation {
    Add(isize),
    Sub(isize),
    Mul(isize),
    Div(isize),
    Neg,
    Inv,
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(152, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(159591692827554, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(301, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(3509819803065, output);
}
