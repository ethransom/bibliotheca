// #![feature(test)]

// extern crate test;

use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example11.txt");
const INPUT: &str = include_str!("input11.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (u64, u64) {
    let stones = parse(input);

    let mut stones = stones
        .into_iter()
        .map(|s| (s, 1))
        .collect::<HashMap<_, _>>();

    for _i in 0..25 {
        stones = blink(stones);
    }

    let after_25 = stones.values().sum();

    for _i in 0..50 {
        stones = blink(stones);
    }

    let after_75 = stones.values().sum();

    (after_25, after_75)
}

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut next = HashMap::new();

    fn insert(counts: &mut HashMap<u64, u64>, stone: u64, count: u64) {
        *counts.entry(stone).or_insert(0) += count;
    }

    for (stone, count) in stones.into_iter() {
        match stone {
            0 => insert(&mut next, 1, count),
            n if n.ilog10() % 2 == 1 => {
                let (left, right) = split(n);
                insert(&mut next, left, count);
                insert(&mut next, right, count);
            }
            n => insert(&mut next, n * 2024, count),
        }
    }

    next
}

fn split(n: u64) -> (u64, u64) {
    let split = 1 + n.ilog10() / 2;
    (n / 10u64.pow(split), n % 10u64.pow(split))
    // let s = format!("{n}");
    // let (left, right) = s.split_at(s.len() / 2);
    // (left.parse().unwrap(), right.parse().unwrap())
}

#[test]
fn test_split() {
    assert_eq!(split(2024), (20, 24));

    assert_eq!(split(17), (1, 7));
}

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (55312, 65601038650482));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (189167, 225253278506288));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
