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

    parse(&mut &slice[..]);

    (0, 0)
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal(u8, Vec<u8>),
    Operator(u8, Vec<Packet>),
}

fn parse(slice: &mut &[u8]) -> Packet {
    println!("parse() slice of length {}", slice.len());
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
        Literal(version, payload)
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
            while payload.len() != 0 {
                children.push(parse(&mut payload));
            }
            *slice = payload; // ?!?!?!?!?
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
    assert_eq!(solve(EXAMPLE), (0, 0));
}

fn binary_slice(input: &str) -> Vec<u8> {
    // assert_eq!(input.len() % 2, 0);
    input
        .chars()
        .map(|c| hex_to_byte(c))
        .flat_map(|byte| {
            (0..4)
                .rev()
                .map(move |pos| if (byte & 1 << pos) > 0 { 1 } else { 0 })
        })
        .collect()
}

fn hex_to_byte(c: char) -> u8 {
    match c {
        '0'..='9' => c as u8 - '0' as u8,
        'A'..='F' => (c as u8 - 'A' as u8) + 10,
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
    assert_eq!(
        parse(&mut &binary_slice("D2FE28")[..]),
        Literal(6, bb("011111100101"))
    );
    assert_eq!(
        parse(&mut &binary_slice("38006F45291200")[..]),
        Operator(
            1,
            vec![
                Literal(6, /* 10 */ bb("1010")),
                Literal(2, /* 20 */ bb("00010100"))
            ]
        ),
    );
    println!("====================");
    assert_eq!(
        parse(&mut &binary_slice("EE00D40C823060")[..]),
        Operator(
            1,
            vec![
                Literal(2, /* 1 */ bb("000")),
                Literal(4, /* 2 */ bb("010")),
                Literal(1, /* 3 */ bb("011")),
            ]
        ),
    );
    // assert_eq!(parse(&binary_slice("D")), 6);
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
