// #![feature(test)]
#![feature(let_chains)]

// extern crate test;

const EXAMPLE: &str = include_str!("example03.txt");
const EXAMPLE_2: &str = include_str!("example03_2.txt");
const INPUT: &str = include_str!("input03.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(EXAMPLE_2));
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
    (scan(input), scan_with_conditionals(input))
}

fn scan(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let mut buf = line;

        while !buf.is_empty() {
            if let Some(mul) = parse_mul(&mut buf) {
                let (left, right) = mul;
                sum += left * right;

                continue;
            }

            buf = &buf[1..];
        }
    }
    sum
}

fn scan_with_conditionals(input: &str) -> usize {
    let mut sum = 0;
    let mut mul_enabled = true;

    for line in input.lines() {
        let mut buf = line;

        while !buf.is_empty() {
            if let Some(mul) = parse_mul(&mut buf) {
                if mul_enabled {
                    let (left, right) = mul;
                    sum += left * right;
                }

                continue;
            }

            if let Some(()) = parse_do(&mut buf) {
                mul_enabled = true;

                continue;
            }

            if let Some(()) = parse_dont(&mut buf) {
                mul_enabled = false;

                continue;
            }

            buf = &buf[1..];
        }
    }
    sum
}

fn parse_mul(buf: &mut &str) -> Option<(usize, usize)> {
    let mut peek = *buf;
    if !matches(&mut peek, "mul") {
        return None;
    }
    if !matches(&mut peek, "(") {
        return None;
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
        return None;
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
        return None;
    }
    *buf = peek;
    let mul = (left, right);
    Some(mul)
}

fn parse_do(buf: &mut &str) -> Option<()> {
    parse_statement(buf, "do")
}

fn parse_dont(buf: &mut &str) -> Option<()> {
    parse_statement(buf, "don't")
}

fn parse_statement(buf: &mut &str, name: &str) -> Option<()> {
    let mut peek = *buf;
    if !matches(&mut peek, name) {
        return None;
    }
    if !matches(&mut peek, "(") {
        return None;
    }
    if !matches(&mut peek, ")") {
        return None;
    }
    *buf = peek;

    Some(())
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE).0, 161);
}

#[test]
fn test_example_2() {
    assert_eq!(solve(EXAMPLE_2), (161, 48));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (160672468, 84893551));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
