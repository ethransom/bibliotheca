// #![feature(test)]

// extern crate test;

use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example19.txt");
const INPUT: &str = include_str!("input19.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let (patterns, designs) = parse(input);

    let matches = designs
        .iter()
        .map(|design| match_designs(design, &patterns, &mut HashMap::default()))
        .collect::<Vec<usize>>();
    (
        matches.iter().filter(|&&m| m > 0).count(),
        matches.iter().sum(),
    )
}

fn match_designs<'a>(
    design: &'a [char],
    patterns: &[Vec<char>],
    memo: &mut HashMap<&'a [char], usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&result) = memo.get(design) {
        return result;
    }

    let mut matches = 0;

    for pattern in patterns {
        let starts_with = design.get(0..pattern.len()).is_some_and(|d| d == pattern);
        // println!("does {design:?} start with {pattern:?}? {starts_with:?}");

        if !starts_with {
            continue;
        }

        matches += match_designs(&design[pattern.len()..], patterns, memo);
    }

    memo.insert(design, matches);

    matches
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();

    (
        patterns
            .trim()
            .split(',')
            .map(|pattern| pattern.trim().chars().collect())
            .collect(),
        designs
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect(),
    )
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (6, 16));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (319, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
