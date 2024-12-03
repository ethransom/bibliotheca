// #![feature(test)]
#![feature(let_chains)]

// extern crate test;

use core::iter::Iterator;

const EXAMPLE: &str = include_str!("example03.txt");
const INPUT: &str = include_str!("input03.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn matches(buf: &mut &str, pat: &str) -> bool {
    if buf.starts_with(pat) {
        *buf = &buf[pat.len()..];
        true
    } else {
        false
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut instructions: Vec<(usize, usize)> = Vec::new();
    for line in input.lines() {
        let mut buf = line;

        while !buf.is_empty() {
            let mut peek = buf;
            buf = &buf[1..];

            println!("{:?}", peek);

            if !matches(&mut peek, "mul") {
                continue;
            }

            if !matches(&mut peek, "(") {
                continue;
            }

            let mut digit = 0;
            while let Some(c) = peek.chars().next()
                && let Some(d) = c.to_digit(10)
            {
                peek = &peek[1..];
                digit *= 10;
                digit += d as usize;
            }
            let left = digit;

            if !matches(&mut peek, ",") {
                continue;
            }

            let mut digit = 0;
            while let Some(c) = peek.chars().next()
                && let Some(d) = c.to_digit(10)
            {
                peek = &peek[1..];
                digit *= 10;
                digit += d as usize;
            }
            let right = digit;

            if !matches(&mut peek, ")") {
                continue;
            }

            buf = peek; // "commit"

            instructions.push((left, right));
        }
    }
    (instructions.iter().map(|(l, r)| l * r).sum(), 0)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (161, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (160672468, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
