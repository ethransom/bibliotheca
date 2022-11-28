#![feature(test)]

use std::num::ParseIntError;

extern crate test;

const EXAMPLE: &str = include_str!("example01.txt");
const INPUT: &str = include_str!("input01.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> u64 {
    let masses = input
        .lines()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<u64>, ParseIntError>>()
        .expect("couldn't parse input file");

    masses.into_iter().map(module_fuel).sum::<u64>()
}

fn module_fuel(mass: u64) -> u64 {
    mass / 3 - 2
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), 34_241);
}

#[test]
fn test_module_fuel() {
    assert_eq!(module_fuel(12), 2);
    assert_eq!(module_fuel(14), 2);
    assert_eq!(module_fuel(1969), 654);
    assert_eq!(module_fuel(100_756), 33_583);
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), 3_167_282);
    });
}
