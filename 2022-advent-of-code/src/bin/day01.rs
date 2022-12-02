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

    let mut per_elf: Vec<u32> = list.iter().map(|calories| calories.iter().sum()).collect();

    per_elf.sort();

    let max = per_elf.iter().rev().take(1).sum();

    let top_three = per_elf.iter().rev().take(3).sum();

    (max, top_three)
}

fn parse(input: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    input
        .split("\n\n")
        .map(|line| line.lines().map(str::parse::<u32>).collect())
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (24_000, 45_000));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (71_023, 206_289));
    });
}
