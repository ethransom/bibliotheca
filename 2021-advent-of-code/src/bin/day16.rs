#![feature(test)]
#![feature(exclusive_range_pattern)]

extern crate core;
extern crate test;

use crate::Operation::{Equal, GreaterThan, LessThan, Maximum, Minimum, Product, Sum};
use crate::Packet::{Literal, Operator};

const EXAMPLE: &str = include_str!("example16.txt");
const INPUT: &str = include_str!("input16.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let slice = binary_slice(input);

    let root = parse(&mut &slice[..]);

    (version_sum(&root), evaluate(&root))
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal(u8, usize),
    Operator(u8, Operation, Vec<Packet>),
}

fn version_sum(packet: &Packet) -> usize {
    match packet {
        Literal(version, _) => *version as usize,
        Operator(version, _, children) => {
            let mut sum = *version as usize;
            for child in children {
                sum += version_sum(child);
            }
            sum
        }
    }
}

fn evaluate(packet: &Packet) -> usize {
    match packet {
        Literal(_, value) => *value as usize,
        Operator(_, operator, children) => {
            let results: Vec<usize> = children.iter().map(evaluate).collect();
            match (operator, &results[..]) {
                (Sum, results) => results.iter().sum(),
                (Product, results) => results.iter().product(),
                (Minimum, results) => *results.iter().min().unwrap(),
                (Maximum, results) => *results.iter().max().unwrap(),
                (GreaterThan, [left, right]) => (left > right) as usize,
                (GreaterThan, _) => panic!("GreaterThan packets must have exactly two sub-packets"),
                (LessThan, [left, right]) => (left < right) as usize,
                (LessThan, _) => panic!("LessThan packets must have exactly two sub-packets"),
                (Equal, [left, right]) => (left == right) as usize,
                (Equal, _) => panic!("Equal packets must have exactly two sub-packets"),
            }
        }
    }
}

fn parse(slice: &mut &[u8]) -> Packet {
    let version = slice_to_byte(&slice[..3]);
    let type_id = slice_to_byte(&slice[3..6]); // TODO: safely handle out-of-range errors?
    *slice = &slice[6..];
    if type_id == 4 {
        let mut payload = vec![];
        loop {
            let stop = slice[0] == 0;
            payload.extend_from_slice(&slice[1..5]);
            *slice = &slice[5..];
            if stop {
                break;
            }
        }
        Literal(version, slice_to_byte_usize(&payload))
    } else {
        let operation = match type_id {
            0 => Sum,
            1 => Product,
            2 => Minimum,
            3 => Maximum,
            5 => GreaterThan,
            6 => LessThan,
            7 => Equal,
            other => panic!("unknown packet id {}", other),
        };

        let length_type_id = slice[0] != 0;
        *slice = &slice[1..];

        let mut children = vec![];
        if length_type_id {
            let length = slice_to_byte_usize(&slice[..11]);
            *slice = &slice[11..];

            for _packet in 0..length {
                children.push(parse(slice));
            }
        } else {
            let length = slice_to_byte_usize(&slice[..15]);
            *slice = &slice[15..];

            let mut payload = &slice[..length];
            while !payload.is_empty() {
                children.push(parse(&mut payload));
            }
            *slice = &slice[length..];
        };

        Operator(version, operation, children)
    }
}

// TODO: make generic
// fn slice_to_byte<T>(slice: &[u8]) -> T {
//     slice.iter().fold(0, |byte, bit| (byte << 1) + bit)
// }
fn slice_to_byte(slice: &[u8]) -> u8 {
    slice.iter().fold(0, |byte, bit| (byte << 1) + bit)
}
fn slice_to_byte_usize(slice: &[u8]) -> usize {
    slice
        .iter()
        .fold(0, |byte, bit| (byte << 1) + *bit as usize)
}

#[test]
fn test_slice_to_byte() {
    assert_eq!(slice_to_byte(&vec![1, 1, 0]), 6);
    assert_eq!(slice_to_byte(&vec![1, 0, 0]), 4);
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (31, 54));
}

fn binary_slice(input: &str) -> Vec<u8> {
    // assert_eq!(input.len() % 2, 0);
    input
        .chars()
        .map(hex_to_byte)
        .flat_map(|byte| {
            (0..4)
                .rev()
                .map(move |pos| if (byte & 1 << pos) > 0 { 1 } else { 0 })
        })
        .collect()
}

fn hex_to_byte(c: char) -> u8 {
    match c {
        '0'..='9' => c as u8 - b'0',
        'A'..='F' => (c as u8 - b'A') + 10,
        _ => panic!("unknown char: {}", c),
    }
}

#[cfg(test)]
fn bb(b: &str) -> Vec<u8> {
    b.chars().map(|c| (c == '1') as u8).collect::<Vec<u8>>()
}

#[test]
fn test_binary_slice() {
    assert_eq!(binary_slice("D2FE28"), bb("110100101111111000101000"));
    assert_eq!(
        binary_slice("38006F45291200"),
        bb("00111000000000000110111101000101001010010001001000000000")
    );
}

#[test]
fn test_parse() {
    for (hex, packets) in [
        ("D2FE28", Literal(6, 2021)),
        (
            "38006F45291200",
            Operator(1, LessThan, vec![Literal(6, 10), Literal(2, 20)]),
        ),
        (
            "EE00D40C823060",
            Operator(
                7,
                Maximum,
                vec![Literal(2, 1), Literal(4, 2), Literal(1, 3)],
            ),
        ),
        (
            "8A004A801A8002F478",
            Operator(
                4,
                Minimum,
                vec![Operator(
                    1,
                    Minimum,
                    vec![Operator(5, Minimum, vec![Literal(6, 15)])],
                )],
            ),
        ),
    ] {
        dbg!(hex, &packets);
        assert_eq!(parse(&mut &binary_slice(hex)[..]), packets);
    }
    // assert_eq!(parse(&binary_slice("D")), 6);
}

#[test]
fn test_version_sum() {
    for (hex, sum) in [
        ("8A004A801A8002F478", 16),
        ("620080001611562C8802118E34", 12),
        ("C0015000016115A2E0802F182340", 23),
        ("A0016C880162017C3686B18A3D4780", 31), // same as example16.txt
    ] {
        dbg!(hex, sum);
        assert_eq!(version_sum(&parse(&mut &binary_slice(hex)[..])), sum);
    }
}
#[test]
fn test_evaluate() {
    for (hex, value) in [
        ("C200B40A82", 3),
        ("04005AC33890", 54),
        ("880086C3E88112", 7),
        ("CE00C43D881120", 9),
        ("D8005AC2A8F0", 1),
        ("F600BC2D8F", 0),
        ("9C005AC2F8F0", 0),
        ("9C0141080250320F1802104A08", 1),
    ] {
        dbg!(hex, value);
        let packets = parse(&mut &binary_slice(hex)[..]);
        dbg!(&packets);
        assert_eq!(evaluate(&packets), value);
    }
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (989, 7936430475134));
    });
}
