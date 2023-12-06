// #![feature(test)]

// extern crate test;

use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example04.txt");
const INPUT: &str = include_str!("input04.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let cards = parse(input);

    (
        cards
            .iter()
            .map(|(winning, have)| match (winning & have).len() {
                0 => 0,
                f => 2_usize.pow(f as u32 - 1),
            })
            .sum(),
        0,
    )
}

fn parse(input: &str) -> Vec<(HashSet<u8>, HashSet<u8>)> {
    input
        .lines()
        .map(|line| {
            let (_prefix, line) = line.split_once(':').unwrap();

            let (winning, have) = line.split_once('|').unwrap();

            (
                winning
                    .split_whitespace()
                    .map(str::parse::<u8>)
                    .collect::<Result<_, _>>()
                    .unwrap(),
                have.split_whitespace()
                    .map(str::parse::<u8>)
                    .collect::<Result<_, _>>()
                    .unwrap(),
            )
        })
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (13, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (24706, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
