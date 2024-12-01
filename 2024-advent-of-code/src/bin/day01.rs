// #![feature(test)]

// extern crate test;

use core::iter::Iterator;

const EXAMPLE: &str = include_str!("example01.txt");
const INPUT: &str = include_str!("input01.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let input = parse(input);

    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input.into_iter().unzip();

    left.sort();
    right.sort();

    (
        left.iter()
            .zip(right.iter())
            .map(|(&left, &right)| left.abs_diff(right) as usize)
            .sum::<usize>(),
        left.iter()
            .map(|&n| right.iter().filter(|&&v| v == n).count() * n as usize)
            .sum(),
    )
}

fn parse(input: &str) -> Vec<(u32, u32)> {
    let input = input.trim();

    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();

            dbg!(left, right);

            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (11, 31));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (3508942, 26593248));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
