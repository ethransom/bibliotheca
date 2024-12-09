// #![feature(test)]

// extern crate test;

use itertools::Itertools;

const EXAMPLE: &str = include_str!("example07.txt");
const INPUT: &str = include_str!("input07.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operator {
    Add,
    Mul,
    Con,
}

const BASIC_OPERATORS: [Operator; 2] = [Operator::Add, Operator::Mul];
const ADV_OPERATORS: [Operator; 3] = [Operator::Add, Operator::Mul, Operator::Con];

fn solve(input: &str) -> (usize, usize) {
    let eqs = parse(input);

    ([&BASIC_OPERATORS, &ADV_OPERATORS] as [&[Operator]; 2])
        .map(|operators| {
            eqs.iter()
                .filter(|(value, nums)| {
                    operator_permutations(operators, nums.len() - 1).any(|ops| {
                        let mut ops_iter = ops.iter();
                        let result = nums
                            .iter()
                            .cloned()
                            .reduce(|a, b| match ops_iter.next() {
                                Some(Operator::Add) => a + b,
                                Some(Operator::Mul) => a * b,
                                Some(Operator::Con) => concat(a, b),
                                None => unreachable!(),
                            })
                            .unwrap();

                        result == *value
                    })
                })
                .map(|(value, _nums)| value)
                .sum()
        })
        .into()
}

fn concat(a: usize, b: usize) -> usize {
    a * 10usize.pow(b.ilog10()) + b
}

fn operator_permutations(
    ops: &'static [Operator],
    len: usize,
) -> impl Iterator<Item = Vec<Operator>> {
    std::iter::repeat(ops.iter().cloned())
        .take(len)
        .multi_cartesian_product()
}

#[test]
fn test_permutations() {
    // TODO: can we be faster by re-rusing a vector here?

    // fn operator_permutations_2(
    //     ops: &'static [Operator],
    //     len: usize,
    // ) -> impl Iterator<Item = Vec<Operator>> {
    //     let v = vec![ops[0]; len];
    //     for i in 0..ops.len().pow(len as u32) {
    //         let mut v = v.clone();
    //         let mut i = i;
    //         for j in 0..len {
    //             v[j] = ops[i % ops.len()];
    //             i /= ops.len();
    //         }
    //         yield v;
    //     }
    // }

    assert_eq!(
        operator_permutations(&[Operator::Add, Operator::Mul], 2).collect_vec(),
        vec![
            [Operator::Add, Operator::Add],
            [Operator::Add, Operator::Mul],
            [Operator::Mul, Operator::Add],
            [Operator::Mul, Operator::Mul]
        ]
    );

    // assert_eq!(
    //     operator_permutations(&BASIC_OPERATORS, 3).collect_vec(),
    //     operator_permutations_2(&BASIC_OPERATORS, 3).collect_vec(),
    // );
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
    assert_eq!(solve(EXAMPLE), (3749, 11387));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (7885693428401, 348360680516005));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
