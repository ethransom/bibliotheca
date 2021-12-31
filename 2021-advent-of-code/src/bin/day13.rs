#![feature(test)]

use regex::Regex;
use std::collections::HashSet;

extern crate test;

const EXAMPLE: &str = include_str!("example13.txt");
const INPUT: &str = include_str!("input13.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

type Point = (usize, usize);

fn solve(input: &str) -> (usize, usize) {
    let (initial_points, folds) = parse(input);

    let mut counts = folds
        .iter()
        .scan(Box::new(initial_points), |points, &fold| {
            let new_points = points
                .iter()
                .map(|&point| fold_point(point, fold))
                .collect();

            print_points(&new_points);

            println!("===========================");

            *points = Box::new(new_points);
            Some(points.clone())
        });

    let one = counts.next().unwrap().len();

    let _ = counts.last().unwrap();

    (one, 0)
}

fn fold_point((px, py): Point, fold: Point) -> Point {
    match fold {
        (fx, 0) => (fold_scalar(px, fx), py),
        (0, fy) => (px, fold_scalar(py, fy)),
        (_, _) => panic!(),
    }
}

fn fold_scalar(point: usize, fold: usize) -> usize {
    let protrusion = point as i64 - fold as i64;
    if protrusion > 0 {
        fold - protrusion as usize
    } else {
        point
    }
}

fn print_points(points: &HashSet<Point>) {
    let &x = points.iter().map(|(x, _)| x).max().unwrap();
    let &y = points.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..=y {
        for x in 0..=x {
            if points.contains(&(x, y)) {
                print!("#")
            } else {
                print!(" ")
            }
        }

        println!();
    }

    dbg!(x, y);
}

#[test]
fn test_fold_scalar() {
    assert_eq!(fold_scalar(6, 8), 6);
    assert_eq!(fold_scalar(8, 6), 4);
}

fn parse(input: &str) -> (HashSet<Point>, Vec<Point>) {
    let (points, folds) = input.split_once("\n\n").expect("expected two sections");

    let points = points
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').expect("expected ','");
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

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
