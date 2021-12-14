#![feature(test)]
#![feature(slice_group_by)]

extern crate test;

use fxhash::FxHashMap;
use iterslide::SlideIterator;

const EXAMPLE: &str = include_str!("example14.txt");
const INPUT: &str = include_str!("input14.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let (polymer, rules) = parse(input);

    let frequencies = simulate(&polymer, &rules, 10);
    dbg!(&frequencies);
    let after_10_most_common = frequencies.iter().map(|(_k, v)| v).max().unwrap();
    let after_10_least_common = frequencies.iter().map(|(_k, v)| v).min().unwrap();

    let frequencies = simulate(&polymer, &rules, 40);
    dbg!(&frequencies);
    let after_40_most_common = frequencies.iter().map(|(_k, v)| v).max().unwrap();
    let after_40_least_common = frequencies.iter().map(|(_k, v)| v).min().unwrap();

    (
        after_10_most_common - after_10_least_common,
        after_40_most_common - after_40_least_common,
    )
}

fn simulate(
    polymer: &[char],
    rules: &FxHashMap<[char; 2], char>,
    steps: usize,
) -> FxHashMap<char, usize> {
    let mut polymer: Box<dyn Iterator<Item = char>> = Box::new(polymer.iter().cloned());
    for _step in 0..steps {
        polymer = Box::new(polymer.slide(2).enumerate().flat_map(|(index, pair)| {
            let element = rules[&[pair[0], pair[1]]];
            if index == 0 {
                return vec![pair[0], element, pair[1]];
            } else {
                return vec![element, pair[1]];
            }
        }));
    }
    // dbg!(polymer.collect::<String>());

    // https://stackoverflow.com/a/70234563/2545138
    polymer
        .enumerate()
        .fold(FxHashMap::default(), |mut map, (idx, val)| {
            if idx % 100_000_000 == 0 {
                println!("at index {}", idx);
            }
            map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
            map
        })
}

fn parse(input: &str) -> (Vec<char>, FxHashMap<[char; 2], char>) {
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
    assert_eq!(solve(EXAMPLE), (1588, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (2170, 0));
    });
}
