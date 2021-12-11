#![feature(test)]

extern crate test;

use std::collections::{HashSet, VecDeque};

const EXAMPLE: &str = include_str!("example11.txt");
const INPUT: &str = include_str!("input11.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let mut grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("not a number"))
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    let octopuses_len = grid.iter().map(|row| row.len()).sum::<usize>();

    let flashed: Vec<usize> = (0..1000).map(|_s| step(&mut grid)).collect();

    let first_100_flashes = flashed.iter().take(100).sum();

    let first_flash = 1 + flashed
        .into_iter()
        .position(|f| f == octopuses_len)
        .expect("did not synchronize");

    (first_100_flashes, first_flash)
}

fn step(grid: &mut Vec<Vec<u32>>) -> usize {
    let mut to_flash = VecDeque::<(usize, usize)>::new();
    // take my energy!
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            grid[r][c] += 1;
            if grid[r][c] > 9 {
                to_flash.push_back((c, r));
            }
        }
    }

    let mut flashed = HashSet::<(usize, usize)>::new();
    while let Some((c, r)) = to_flash.pop_front() {
        if flashed.contains(&(c, r)) {
            continue;
        }
        flashed.insert((c, r));

        for rr in -1..=1 {
            for cc in -1..=1 {
                if rr == 0 && cc == 0 {
                    continue;
                }

                let r = (r as i64 + rr) as usize;
                let c = (c as i64 + cc) as usize;

                if let Some(row) = grid.get_mut(r) {
                    if let Some(val) = row.get_mut(c) {
                        *val += 1;
                        if *val > 9 {
                            to_flash.push_back((c, r));
                        }
                    }
                }
            }
        }
    }

    // could also loop through `flashed` for this
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] > 9 {
                grid[r][c] = 0;
            }
        }
    }

    flashed.len()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (1656, 195));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (1591, 314));
    });
}
