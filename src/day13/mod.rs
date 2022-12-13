use crate::utils::{fast_parse_int_from_bytes, split_once_2};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 13;

    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut bytes = input.as_bytes();
        let mut val = 0;
        let mut index = 1;
        loop {
            let (line1, line2);
            (line1, bytes) = split_once_2(bytes, b'\n');
            bytes = &bytes[1..];
            (line2, bytes) = split_once_2(bytes, b'\n');

            debug_assert_eq!(line1[0], b'[');
            debug_assert_eq!(line2[0], b'[');
            let (packet_1, remaining_1) = Packet::parse(line1);
            let (packet_2, remaining_2) = Packet::parse(line2);

            debug_assert_eq!(remaining_1, b"");
            debug_assert_eq!(remaining_2, b"");

            if packet_1 < packet_2 {
                val += index;
            }

            if bytes.is_empty() {
                break;
            }

            bytes = &bytes[2..];
            index += 1;
        }

        val
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut bytes = input.as_bytes();

        let distress_1 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
        let distress_2 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
        let mut packets = vec![distress_1.clone(), distress_2.clone()];
        loop {
            let (line1, line2);
            (line1, bytes) = split_once_2(bytes, b'\n');
            bytes = &bytes[1..];
            (line2, bytes) = split_once_2(bytes, b'\n');

            debug_assert_eq!(line1[0], b'[');
            debug_assert_eq!(line2[0], b'[');
            let (packet_1, remaining_1) = Packet::parse(line1);
            let (packet_2, remaining_2) = Packet::parse(line2);

            debug_assert_eq!(remaining_1, b"");
            debug_assert_eq!(remaining_2, b"");

            packets.push(packet_1);
            packets.push(packet_2);

            if bytes.is_empty() {
                break;
            }

            bytes = &bytes[2..];
        }

        packets.sort();

        (packets.iter().position(|p| p == &distress_1).unwrap() + 1)
            * (packets.iter().position(|p| p == &distress_2).unwrap() + 1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Packet {
    Number(usize),
    List(Vec<Packet>),
}

impl Packet {
    pub fn parse(mut bytes: &[u8]) -> (Self, &[u8]) {
        match &bytes[0] {
            b'[' => {
                let mut list = vec![];

                if bytes[1] == b']' {
                    return (Self::List(vec![]), &bytes[2..]);
                }

                while bytes[0] != b']' {
                    bytes = &bytes[1..];
                    let packet;
                    (packet, bytes) = Self::parse(bytes);
                    list.push(packet);
                    debug_assert!(matches!(bytes[0], b',' | b']'));
                }
                bytes = &bytes[1..];

                (Self::List(list), bytes)
            }
            _ => {
                debug_assert!(matches!(&bytes[0], b'0'..=b'9'));
                let num;
                (num, bytes) = Self::parse_number(bytes);
                (Self::Number(num), bytes)
            }
        }
    }

    fn parse_number(bytes: &[u8]) -> (usize, &[u8]) {
        let num_len = bytes
            .iter()
            .take_while(|b| matches!(b, b'0'..=b'9'))
            .count();
        let (num_bytes, rem_bytes) = bytes.split_at(num_len);
        let num = fast_parse_int_from_bytes(num_bytes);

        (num, rem_bytes)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => a
                .iter()
                .zip(b.iter())
                .flat_map(|(a, b)| match a.cmp(b) {
                    std::cmp::Ordering::Equal => None,
                    c => Some(c),
                })
                .next()
                .unwrap_or_else(|| a.len().cmp(&b.len())),
            (Packet::Number(_), Packet::List(_)) => Packet::List(vec![self.clone()]).cmp(other),
            (Packet::List(_), Packet::Number(_)) => self.cmp(&Packet::List(vec![other.clone()])),
        }
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(13, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(6478, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(140, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(21922, output);
}
