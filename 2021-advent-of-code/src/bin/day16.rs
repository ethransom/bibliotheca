#![feature(test)]
#![feature(exclusive_range_pattern)]

extern crate core;
extern crate test;

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

    (version_sum(&root), 0)
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal(u8, usize),
    Operator(u8, Vec<Packet>),
}

fn version_sum(packet: &Packet) -> usize {
    match packet {
        Literal(version, _) => *version as usize,
        Operator(version, children) => {
            let mut sum = *version as usize;
            for child in children {
                sum += version_sum(child);
            }
            sum
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
        let length_type_id = slice[0] != 0;
        *slice = &slice[1..];

        if length_type_id {
            let length = slice_to_byte_usize(&slice[..11]);
            *slice = &slice[11..];

            let mut children = vec![];
            for _packet in 0..length {
                children.push(parse(slice));
            }
            Operator(version, children)
        } else {
            let length = slice_to_byte_usize(&slice[..15]);
            *slice = &slice[15..];

            let mut payload = &slice[..length];
            let mut children = vec![];
            while !payload.is_empty() {
                children.push(parse(&mut payload));
            }
            *slice = &slice[length..];
            Operator(version, children)
        }
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
    assert_eq!(solve(EXAMPLE), (31, 0));
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
            Operator(1, vec![Literal(6, 10), Literal(2, 20)]),
        ),
        (
            "EE00D40C823060",
            Operator(7, vec![Literal(2, 1), Literal(4, 2), Literal(1, 3)]),
        ),
        (
            "8A004A801A8002F478",
            Operator(
                4,
                vec![Operator(1, vec![Operator(5, vec![Literal(6, 15)])])],
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

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (989, 0));
    });
}
