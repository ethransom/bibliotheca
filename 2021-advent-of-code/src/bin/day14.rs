#![feature(test)]
#![feature(slice_group_by)]

extern crate test;

use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example14.txt");
const INPUT: &str = include_str!("input14.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let (polymer, rules) = parse(input);

    let frequencies = simulate(&polymer, &rules, 10);
    let after_10_most_common = frequencies.iter().map(|(_k, v)| v).max().unwrap();
    let after_10_least_common = frequencies.iter().map(|(_k, v)| v).min().unwrap();

    let frequencies = simulate(&polymer, &rules, 40);
    let after_40_most_common = frequencies.iter().map(|(_k, v)| v).max().unwrap();
    let after_40_least_common = frequencies.iter().map(|(_k, v)| v).min().unwrap();

    (
        after_10_most_common - after_10_least_common,
        after_40_most_common - after_40_least_common,
    )
}

fn simulate(
    polymer: &[char],
    rules: &HashMap<[char; 2], char>,
    steps: usize,
) -> HashMap<char, usize> {
    // https://stackoverflow.com/a/70234563/2545138
    let mut pairs = polymer.windows(2).fold(HashMap::new(), |mut map, val| {
        map.entry([val[0], val[1]])
            .and_modify(|frq| *frq += 1)
            .or_insert(1);
        map
    });

    let mut counts = polymer.iter().fold(HashMap::default(), |mut map, &val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });

    for _step in 0..steps {
        pairs = pairs
            .iter()
            .fold(HashMap::new(), |mut map, (pair, &count)| {
                let element = rules[&[pair[0], pair[1]]];

                map.entry([pair[0], element])
                    .and_modify(|frq| *frq += count)
                    .or_insert(count);
                map.entry([element, pair[1]])
                    .and_modify(|frq| *frq += count)
                    .or_insert(count);

                counts
                    .entry(element)
                    .and_modify(|cnt| *cnt += count)
                    .or_insert(count);

                map
            });
    }

    counts
}

fn parse(input: &str) -> (Vec<char>, HashMap<[char; 2], char>) {
    let (polymer, rules) = input.split_once("\n\n").expect("expected two sections");

    let polymer: Vec<char> = polymer.chars().collect();

    let rules = rules
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" -> ").expect("expected ' -> '");

            let mut left = left.chars();

            let mut right = right.chars();

            (
                [left.next().unwrap(), left.next().unwrap()],
                right.next().unwrap(),
            )
        })
        .collect();

    (polymer, rules)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (1588, 2_188_189_693_529));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (2170, 2_422_444_761_283));
    });
}
