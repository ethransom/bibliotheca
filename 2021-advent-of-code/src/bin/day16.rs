#![feature(test)]
#![feature(exclusive_range_pattern)]

extern crate test;
extern crate core;

const EXAMPLE: &str = include_str!("example16.txt");
const INPUT: &str = include_str!("input16.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let _blocks = parse(input);

    (0, 0)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (0, 0));
}

fn parse(input: &str) -> Vec<u8> {
    // assert_eq!(input.len() % 2, 0);
    input
        .chars()
        .map(|c| hex_to_byte(c))
        .flat_map(|byte| {
            (0..4).rev().map(move |pos| if (byte & 1 << pos) > 0 { 1 } else { 0 })
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

#[test]
fn test_parse() {
    assert_eq!(parse("D2FE28"),
               "110100101111111000101000".chars().map(|c| (c == '1') as u8).collect::<Vec<u8>>());
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
