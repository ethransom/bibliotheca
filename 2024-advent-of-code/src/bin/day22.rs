// #![feature(test)]

// extern crate test;

use std::num::ParseIntError;

const EXAMPLE: &str = include_str!("example22.txt");
const INPUT: &str = include_str!("input22.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let nums = parse(input).expect("parse error");

    (
        nums.iter()
            .cloned()
            .map(|initial| {
                // foobar
                let mut num = initial;
                for _ in 0..2000 {
                    num = next(num);
                }

                println!("{initial}: {num}");

                num
            })
            .sum(),
        0,
    )
}

fn next(num: usize) -> usize {
    // Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
    let num = prune(mix(num, num * 64));

    // Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
    let num = prune(mix(num, num / 32));

    // Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
    let num = prune(mix(num, num * 2048));

    num
}

#[test]
fn test_next() {
    assert_eq!(next(123), 15887950);
}

fn mix(a: usize, b: usize) -> usize {
    a ^ b
}

#[test]
fn test_mix() {
    assert_eq!(mix(42, 15), 37);
}

fn prune(num: usize) -> usize {
    num % 16777216
}

#[test]
fn test_prune() {
    assert_eq!(prune(100000000), 16113920);
}

fn parse(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.lines().map(|line| line.parse()).collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (37327623, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (17262627539, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
