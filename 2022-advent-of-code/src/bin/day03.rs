#![feature(test)]
#![feature(array_chunks)]

extern crate test;

use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example03.txt");
const INPUT: &str = include_str!("input03.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let priorities_of_splits = parse(input)
        .map(|line| {
            let (front, back) = line.split_at(line.len() / 2);

            let [items1, items2]: [HashSet<char>; 2] = [
                front.chars().collect::<HashSet<char>>(),
                back.chars().collect::<HashSet<char>>(),
            ];

            HashSet::intersection(&items1, &items2)
                .map(item_priority)
                .sum::<usize>()
        })
        .sum();

    let priorities_of_badges = parse(input)
        .map(|line| line.chars().collect::<HashSet<char>>())
        .collect::<Vec<HashSet<char>>>()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let badge = &(a & b) & c;

            assert_eq!(badge.len(), 1);

            let badge = badge.iter().next().unwrap();

            item_priority(badge)
        })
        .sum();

    (priorities_of_splits, priorities_of_badges)
}

fn item_priority(&item: &char) -> usize {
    1 + ('a'..='z')
        .chain('A'..='Z')
        .position(|c| c == item)
        .unwrap_or_else(|| panic!("unknown item type {item}"))
}

fn parse(input: &str) -> impl Iterator<Item = &str> {
    input.lines()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (157, 70));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (7_727, 2_609));
    });
}
