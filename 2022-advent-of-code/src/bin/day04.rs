#![feature(test)]

extern crate test;

const EXAMPLE: &str = include_str!("example04.txt");
const INPUT: &str = include_str!("input04.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let assignments = parse(input);

    let full_overlaps = assignments
        .iter()
        .filter(|(a, b)| is_fully_contained(a, b) || is_fully_contained(b, a))
        .count();

    let overlaps = assignments
        .iter()
        .filter(|(a, b)| is_contained(a, b))
        .count();

    (full_overlaps, overlaps)
}

fn is_fully_contained(range1: &(u32, u32), range2: &(u32, u32)) -> bool {
    range1.0 <= range2.0 && range1.1 >= range2.1
}

fn is_contained(range1: &(u32, u32), range2: &(u32, u32)) -> bool {
    (range2.0..=range2.1).contains(&range1.0) || (range1.0..=range1.1).contains(&range2.0)
}

fn parse(input: &str) -> Vec<((u32, u32), (u32, u32))> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').expect("expected two assignments");

            let ((a, b), (c, d)) = (
                left.split_once('-').unwrap(),
                right.split_once('-').unwrap(),
            );

            (
                (a.parse().unwrap(), b.parse().unwrap()),
                (c.parse().unwrap(), d.parse().unwrap()),
            )
        })
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (2, 4));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (540, 872));
    });
}
