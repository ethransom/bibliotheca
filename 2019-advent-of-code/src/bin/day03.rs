#![feature(test)]

use std::{collections::HashMap, hash::BuildHasherDefault};

use fxhash::FxHasher;

extern crate test;

const EXAMPLE: &str = include_str!("example03.txt");
const INPUT: &str = include_str!("input03.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

type FastCoordinateMap = HashMap<(i64, i64), u64, BuildHasherDefault<FxHasher>>;

fn solve(input: &str) -> (u64, u64) {
    let wires: [Vec<&str>; 2] = input
        .lines()
        .map(|line| line.split(',').collect())
        .collect::<Vec<Vec<&str>>>()
        .try_into()
        .expect("expected two wires in input");

    let [path1, path2] = wires.map(|wire| {
        let mut path: FastCoordinateMap = HashMap::default();

        wire.iter().fold(((0, 0), 0), |(cursor, dist), segment| {
            let (dir, amount) = (&segment[..1], &segment[1..]);
            let amount: u64 = amount.parse().expect("couldn't parse segment length");

            let unit = match dir {
                "U" => (0, 1),
                "R" => (1, 0),
                "D" => (0, -1),
                "L" => (-1, 0),
                _ => panic!("unknown segment direction: '{}'", dir),
            };

            (1..=amount).fold((cursor, dist), |((x, y), dist), _amount| {
                let (cursor, dist) = ((x + unit.0, y + unit.1), dist + 1);
                path.insert(cursor, dist);
                (cursor, dist)
            })
        });

        path
    });

    let intersections = path1.keys().filter(|coord| path2.contains_key(coord));

    let closest_intersection = intersections
        .clone()
        .cloned()
        .map(manhattan_dist)
        .min()
        .expect("no wire intersections found");

    let soonest_intersection = intersections
        .map(|coord| path1.get(coord).unwrap() + path2.get(coord).unwrap())
        .min()
        .expect("no wire intersections found");

    (closest_intersection, soonest_intersection)
}

fn manhattan_dist(coords: (i64, i64)) -> u64 {
    coords.0.unsigned_abs() + coords.1.unsigned_abs()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (6, 30));
}

#[bench]
fn bench_solve_part1_03_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (557, 56_410));
    });
}

#[bench]
fn bench_solve_part1_02_fxhash(b: &mut test::Bencher) {
    use std::collections::HashSet;

    type FastCoordinateSet = HashSet<(i64, i64), BuildHasherDefault<FxHasher>>;

    fn solve(input: &str) -> (u64, u64) {
        let wires: [Vec<&str>; 2] = input
            .lines()
            .map(|line| line.split(',').collect())
            .collect::<Vec<Vec<&str>>>()
            .try_into()
            .expect("expected two wires in input");

        let [path1, path2] = wires.map(|wire| {
            let mut path: FastCoordinateSet = HashSet::default();

            wire.iter().fold((0, 0), |cursor, segment| {
                let (dir, amount) = (&segment[..1], &segment[1..]);
                let amount: u64 = amount.parse().expect("couldn't parse segment length");

                let unit = match dir {
                    "U" => (0, 1),
                    "R" => (1, 0),
                    "D" => (0, -1),
                    "L" => (-1, 0),
                    _ => panic!("unknown segment direction: '{}'", dir),
                };

                (1..=amount).fold(cursor, |(x, y), _amount| {
                    let cursor = (x + unit.0, y + unit.1);
                    path.insert(cursor);
                    cursor
                })
            });

            path
        });

        let closest_intersection = path1
            .iter()
            .filter(|coord| path2.contains(coord))
            .map(|p| manhattan_dist(*p))
            .min()
            .expect("no wire intersections found");

        (closest_intersection, 0)
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (557, 0));
    });
}

#[bench]
fn bench_solve_part1_01_original(b: &mut test::Bencher) {
    use std::collections::HashSet;

    fn solve(input: &str) -> (u64, u64) {
        let wires: [Vec<&str>; 2] = input
            .lines()
            .map(|line| line.split(',').collect())
            .collect::<Vec<Vec<&str>>>()
            .try_into()
            .expect("expected two wires in input");

        let [path1, path2] = wires.map(|wire| {
            let mut path: HashSet<(i64, i64)> = HashSet::new();
            wire.iter().fold((0, 0), |cursor, segment| {
                let (dir, amount) = (&segment[..1], &segment[1..]);
                let amount: u64 = amount.parse().expect("couldn't parse segment length");

                let unit = match dir {
                    "U" => (0, 1),
                    "R" => (1, 0),
                    "D" => (0, -1),
                    "L" => (-1, 0),
                    _ => panic!("unknown segment direction: '{}'", dir),
                };

                (1..=amount).fold(cursor, |(x, y), _amount| {
                    let cursor = (x + unit.0, y + unit.1);
                    path.insert(cursor);
                    cursor
                })
            });

            path
        });

        let closest_intersection = path1
            .intersection(&path2)
            .cloned()
            .map(manhattan_dist)
            .min()
            .expect("no wire intersections found");

        (closest_intersection, 0)
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (557, 0));
    });
}
