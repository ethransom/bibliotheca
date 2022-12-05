#![feature(test)]

use itertools::Itertools;

extern crate test;

const EXAMPLE: &str = include_str!("example05.txt");
const INPUT: &str = include_str!("input05.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let _blocks = parse(input);

    (0, 0)
}

// TODO: structs? 😮‍💨
type Stack = Vec<char>;
type Procedure = (u8, u8, u8);

fn parse(input: &str) -> (Vec<Stack>, Vec<Procedure>) {
    let (storage, procedure) = input.split_once("\n\n").unwrap();

    let mut crates = storage.lines().collect::<Vec<&str>>();
    let stack_count = crates.pop().unwrap().split_whitespace().count();
    let stacks = (0..stack_count)
        .map(|stack| {
            crates
                .iter()
                .rev()
                .flat_map(|row| {
                    row.chars()
                        .nth(stack * 4 + 1)
                        .filter(|c| ('A'..='Z').contains(c))
                })
                .collect()
        })
        .collect();

    let procedure = procedure
        .lines()
        .map(|line: &str| {
            line.split_whitespace()
                .filter_map(|i| i.parse().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    (stacks, procedure)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (0, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
