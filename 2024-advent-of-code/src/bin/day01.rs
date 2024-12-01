#![feature(test)]

extern crate test;

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

#[bench]
fn bench_solve_01_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (3508942, 26593248));
    });
}

#[bench]
fn bench_solve_02_heap_and_hashcounter(b: &mut test::Bencher) {
    type BinaryHeap = std::collections::BinaryHeap<u32>;

    use std::collections::HashMap;

    fn solve(input: &str) -> (usize, usize) {
        let input = input.trim();

        let (mut left, mut right) = (BinaryHeap::new(), BinaryHeap::new());

        let mut right_freqs = HashMap::<u32, usize>::new();

        for line in input.lines() {
            let (a, b) = line.split_once("   ").unwrap();

            let [a, b] = [a, b].map(|v| v.parse().unwrap());

            left.push(a);
            right.push(b);

            right_freqs
                .entry(b)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        let [left, right] = [left, right].map(|v| v.into_sorted_vec());

        (
            left.iter()
                .zip(right.iter())
                .map(|(&left, &right)| left.abs_diff(right) as usize)
                .sum::<usize>(),
            left.iter()
                .map(|&n| right_freqs.get(&n).unwrap_or(&0) * n as usize)
                .sum(),
        )
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (3508942, 26593248));
    });
}

#[bench]
fn bench_solve_03_just_hashcounter(b: &mut test::Bencher) {
    use std::collections::HashMap;

    fn solve(input: &str) -> (usize, usize) {
        let input = input.trim();

        let (mut left, mut right) = (Vec::new(), Vec::new());

        let mut right_freqs = HashMap::<u32, usize>::new();

        for line in input.lines() {
            let (a, b) = line.split_once("   ").unwrap();

            let [a, b] = [a, b].map(|v| v.parse().unwrap());

            left.push(a);
            right.push(b);

            right_freqs
                .entry(b)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        left.sort();
        right.sort();

        (
            left.iter()
                .zip(right.iter())
                .map(|(&left, &right)| left.abs_diff(right) as usize)
                .sum::<usize>(),
            left.iter()
                .map(|&n| right_freqs.get(&n).unwrap_or(&0) * n as usize)
                .sum(),
        )
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (3508942, 26593248));
    });
}

#[bench]
fn bench_solve_04_fxhash_hashcounter(b: &mut test::Bencher) {
    use fxhash::FxHashMap as HashMap;

    fn solve(input: &str) -> (usize, usize) {
        let input = input.trim();

        let (mut left, mut right) = (Vec::new(), Vec::new());

        let mut right_freqs = HashMap::<u32, usize>::default();

        for line in input.lines() {
            let (a, b) = line.split_once("   ").unwrap();

            let [a, b] = [a, b].map(|v| v.parse().unwrap());

            left.push(a);
            right.push(b);

            right_freqs
                .entry(b)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        left.sort();
        right.sort();

        (
            left.iter()
                .zip(right.iter())
                .map(|(&left, &right)| left.abs_diff(right) as usize)
                .sum::<usize>(),
            left.iter()
                .map(|&n| right_freqs.get(&n).unwrap_or(&0) * n as usize)
                .sum(),
        )
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (3508942, 26593248));
    });
}

#[bench]
fn bench_solve_part1_01_current(b: &mut test::Bencher) {
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
            0,
        )
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (3508942, 0));
    });
}

#[bench]
fn bench_solve_part1_02_heap(b: &mut test::Bencher) {
    type BinaryHeap = std::collections::BinaryHeap<u32>;

    fn solve(input: &str) -> (usize, usize) {
        let input = input.trim();

        let (mut left, mut right) = (BinaryHeap::new(), BinaryHeap::new());

        for line in input.lines() {
            let (a, b) = line.split_once("   ").unwrap();

            let [a, b] = [a, b].map(|v| v.parse().unwrap());

            left.push(a);
            right.push(b);
        }

        let [left, right] = [left, right].map(|v| v.into_sorted_vec());

        (
            left.iter()
                .zip(right.iter())
                .map(|(&left, &right)| left.abs_diff(right) as usize)
                .sum::<usize>(),
            0,
        )
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (3508942, 0));
    });
}
