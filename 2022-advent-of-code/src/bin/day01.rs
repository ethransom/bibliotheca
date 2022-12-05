#![feature(test)]
#![feature(is_some_and)]
#![feature(let_chains)]

use std::num::ParseIntError;

extern crate test;

const EXAMPLE: &str = include_str!("example01.txt");
const INPUT: &str = include_str!("input01.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (u32, u32) {
    let list = parse(input).unwrap();

    let mut per_elf: Vec<u32> = list.iter().map(|calories| calories.iter().sum()).collect();

    per_elf.sort();

    let max = per_elf.iter().rev().take(1).sum();

    let top_three = per_elf.iter().rev().take(3).sum();

    (max, top_three)
}

fn parse(input: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    input
        .split("\n\n")
        .map(|line| line.lines().map(str::parse::<u32>).collect())
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (24_000, 45_000));
}

#[bench]
fn bench_solve_00_original(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (71_023, 206_289));
    });
}

#[bench]
fn bench_solve_01_onealloc(b: &mut test::Bencher) {
    fn solve(input: &str) -> (u32, u32) {
        let mut per_elf = input
            .split("\n\n")
            .map(|line| {
                line.lines().try_fold(0, |sum, item| {
                    str::parse::<u32>(item).map(|i| sum + i)
                })
            })
            .collect::<Result<Vec<u32>, ParseIntError>>()
            .unwrap();

        per_elf.sort();

        let max = per_elf.iter().rev().take(1).sum();

        let top_three = per_elf.iter().rev().take(3).sum();

        (max, top_three)
    }

    assert_eq!(solve(EXAMPLE), (24_000, 45_000));

    b.iter(|| {
        assert_eq!(solve(INPUT), (71_023, 206_289));
    });
}

#[bench]
fn bench_solve_02_noalloc(b: &mut test::Bencher) {
    struct StaticMaxHeap<const N: usize> {
        heap: [Option<u32>; N],
    }

    impl<const N: usize> StaticMaxHeap<N> {
        fn new() -> StaticMaxHeap<N> {
            StaticMaxHeap { heap: [None; N] }
        }

        fn add(&mut self, calories: u32) {
            for i in self.heap.iter_mut() {
                if i.is_none() {
                    *i = Some(calories);
                    return;
                }
            }

            let min = self.heap.iter_mut().min().unwrap();
            if min.unwrap() < calories {
                *min = Some(calories);
            }
        }
    }

    fn solve(input: &str) -> (u32, u32) {
        let heap = input
            .split("\n\n")
            .try_fold(StaticMaxHeap::new(), |mut heap, line| {
                line.lines()
                    .try_fold(0, |sum, item| {
                        str::parse::<u32>(item).map(|i| sum + i)
                    })
                    .map(|calories| {
                        heap.add(calories);

                        heap
                    })
            })
            .expect("couldn't parse calories");

        let top_three: [u32; 3] = heap.heap.map(|i| i.expect("not enough to get top three"));

        let &max = top_three.iter().max().unwrap();

        (max, top_three.iter().sum())
    }

    assert_eq!(solve(EXAMPLE), (24_000, 45_000));

    b.iter(|| {
        assert_eq!(solve(INPUT), (71_023, 206_289));
    });
}
