#![feature(test)]

use std::num::ParseIntError;

extern crate test;

const EXAMPLE: &str = include_str!("example01.txt");
const INPUT: &str = include_str!("input01.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (u32, u32) {
    let list = parse(input).unwrap();

    let max = list
        .iter()
        .map(|calories| calories.iter().sum())
        .max()
        .expect("no elves with calories");

    (max, 0)
}

fn parse(input: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    input
        .split("\n\n")
        .map(|line| line.lines().map(str::parse::<u32>).collect())
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (24_000, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (71_023, 0));
    });
}
