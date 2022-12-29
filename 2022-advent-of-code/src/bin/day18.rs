#![feature(test)]

use anyhow::Result;
use std::collections::HashSet;

extern crate test;

const EXAMPLE: &str = include_str!("example18.txt");
const INPUT: &str = include_str!("input18.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

type Point = (i32, i32, i32);

fn solve(input: &str) -> (usize, usize) {
    let cubes = parse(input).expect("couldn't parse input");

    let surface_area = cubes
        .iter()
        .map(|&cube| {
            neighbors(cube)
                .filter(|neighbor| !cubes.contains(neighbor))
                .count()
        })
        .sum();

    (surface_area, 0)
}

fn parse(input: &str) -> Result<HashSet<Point>> {
    input
        .lines()
        .map(|line| -> Result<Point> {
            let [x, y, z]: [&str; 3] = line.split(',').collect::<Vec<&str>>().try_into().unwrap();
            Ok((x.parse()?, y.parse()?, z.parse()?))
        })
        .collect()
}

fn neighbors(p: Point) -> impl Iterator<Item = Point> {
    [
        (-1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (1, 0, 0),
        (0, 0, -1),
        (0, 0, 1),
    ]
    .into_iter()
    .map(move |d| (d.0 + p.0, d.1 + p.1, d.2 + p.2))
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (64, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (4_418, 0));
    });
}
