// #![feature(test)]

// extern crate test;

use itertools::Itertools;

const EXAMPLE: &str = include_str!("example07.txt");
const INPUT: &str = include_str!("input07.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Mul,
}

const OPERATORS: [Operator; 2] = [Operator::Add, Operator::Mul];

fn solve(input: &str) -> (usize, usize) {
    let eqs = parse(input);

    let part1 = eqs
        .iter()
        .filter(|(value, nums)| {
            operator_permutations(nums.len() - 1).any(|ops| {
                let mut ops_iter = ops.iter();
                let result = nums
                    .iter()
                    .cloned()
                    .reduce(|a, b| match ops_iter.next() {
                        Some(Operator::Add) => a + b,
                        Some(Operator::Mul) => a * b,
                        None => unreachable!(),
                    })
                    .unwrap();

                result == *value
            })
        })
        .map(|(value, _nums)| value)
        .sum();

    (part1, 0)
}

fn operator_permutations(len: usize) -> impl Iterator<Item = Vec<Operator>> {
    std::iter::repeat(OPERATORS.into_iter())
        .take(len)
        .multi_cartesian_product()
}

fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (value, nums) = line.split_once(": ").unwrap();

            (
                value.parse().unwrap(),
                nums.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (3749, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (7885693428401, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
