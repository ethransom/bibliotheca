#![feature(test)]

use regex::Regex;

extern crate test;

const EXAMPLE: &str = include_str!("example13.txt");
const INPUT: &str = include_str!("input13.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

type Point = (usize, usize);

fn solve(input: &str) -> (usize, usize) {
    let (points, folds) = parse(input);

    dbg!(points);

    dbg!(folds);
    (0, 0)
}

fn parse(input: &str) -> (Vec<Point>, Vec<Point>) {
    let (points, folds) = input.split_once("\n\n").expect("expected two sections");

    let points = points
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').expect("expected ','");
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<Point>>();

    let regex = Regex::new(r"fold along (x|y)=(\d+)$").expect("couldn't compile regex");

    let folds = folds
        .lines()
        .map(|line| {
            let caps = regex
                .captures(line)
                .expect("couldn't parse fold instruction");

            let pos = caps[2].parse().unwrap();

            match &caps[1] {
                "x" => (pos, 0),
                "y" => (0, pos),
                _ => unreachable!(),
            }
        })
        .collect();

    (points, folds)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (17, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
