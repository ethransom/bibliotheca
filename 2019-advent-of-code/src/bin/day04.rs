#![feature(test)]

extern crate test;

const INPUT: &str = "108457-562041";

fn main() {
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let (start, end) = parse(input);

    let passwords = (start..=end).filter(|&n| {
        let digits = to_digits(n);

        is_increasing(digits) && has_repeat(digits)
    });

    (passwords.count(), 0)
}

fn parse(input: &str) -> (u64, u64) {
    let (start, end) = input.split_once('-').expect("expected both start and end");

    (
        start.parse().expect("range start not an integer"),
        end.parse().expect("range end not an integer"),
    )
}

fn to_digits(num: u64) -> [u8; 6] {
    let mut digits = [0u8; 6];

    digits.iter_mut().rev().fold(num, |num, digit| {
        *digit = (num % 10) as u8;
        num / 10
    });

    digits
}

#[test]
fn test_to_digits() {
    assert_eq!(to_digits(987654), [9, 8, 7, 6, 5, 4]);
}

fn has_repeat(digits: [u8; 6]) -> bool {
    digits.windows(2).any(|window| window[0] == window[1])
}

#[test]
fn test_has_repeat() {
    assert!(has_repeat(to_digits(122345)));
    assert!(!has_repeat(dbg!(to_digits(123789))));
}

fn is_increasing(digits: [u8; 6]) -> bool {
    digits.windows(2).all(|window| window[0] <= window[1])
}

#[test]
fn test_is_increasing() {
    assert!(is_increasing(to_digits(135679)));
    assert!(!is_increasing(dbg!(to_digits(223450))));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (2779, 0));
    });
}
