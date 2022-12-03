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
                    str::parse::<u32>(item).and_then(|i| Ok(sum + i))
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
    fn solve(input: &str) -> (u32, u32) {
        let top_three = input
            .split("\n\n")
            .try_fold([Option::None::<u32>; 3], |mut top_three, line| {
                line.lines()
                    .try_fold(0, |sum, item| {
                        str::parse::<u32>(item).and_then(|i| Ok(sum + i))
                    })
                    .and_then(|calories| {
                        // refactor: extract this into a StaticNHeap or similar
                        for i in top_three.iter_mut() {
                            if i.is_none() {
                                *i = Some(calories);
                                return Ok(top_three);
                            }
                        }

                        let min = top_three.iter_mut().min().unwrap();
                        if min.unwrap() < calories {
                            *min = Some(calories);
                        }

                        Ok(top_three)
                    })
            })
            .expect("couldn't parse calories")
            .map(|i| i.expect("not enough to get top three"));

        let &max = top_three.iter().max().unwrap();

        let top_three = top_three.iter().sum();

        (max, top_three)
    }

    assert_eq!(solve(EXAMPLE), (24_000, 45_000));

    b.iter(|| {
        assert_eq!(solve(INPUT), (71_023, 206_289));
    });
}
