#![feature(test)]

use std::collections::HashSet;

use itertools::Itertools;

use anyhow::Result;

extern crate test;

const EXAMPLE: &str = include_str!("example14.txt");
const INPUT: &str = include_str!("input14.txt");

type Point = (i32, i32);

const SOURCE: Point = (500, 0);

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let lines = parse(input).expect("couldn't parse input");

    let mut rocks = HashSet::<Point>::new();
    for line in &lines {
        for (start, end) in line.iter().tuple_windows() {
            match (start.0.cmp(&end.0), start.1.cmp(&end.1)) {
                (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => {}
                (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => {
                    for y in end.1..=start.1 {
                        rocks.insert((start.0, y));
                    }
                }
                (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => {
                    for y in start.1..=end.1 {
                        rocks.insert((start.0, y));
                    }
                }
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => {
                    for x in end.0..=start.0 {
                        rocks.insert((x, start.1));
                    }
                }
                (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => {
                    for x in start.0..=end.0 {
                        rocks.insert((x, start.1));
                    }
                }
                _ => panic!("invalid line"),
            }
        }
    }

    let void_y = rocks.iter().map(|&(_, y)| y).max().unwrap();

    let mut sand = HashSet::<Point>::new();

    'place_all: loop {
        let mut pos = SOURCE;
        'place_one: loop {
            if pos.1 > void_y {
                // do not place
                break 'place_all;
            }
            let moves = [
                (pos.0, pos.1 + 1),
                (pos.0 - 1, pos.1 + 1),
                (pos.0 + 1, pos.1 + 1),
            ];
            for m in moves {
                if !rocks.contains(&m) && !sand.contains(&m) {
                    pos = m;
                    continue 'place_one;
                }
            }
            // could not move block
            break 'place_one;
        }
        sand.insert(pos);
    }

    print(SOURCE, &rocks, &sand);

    (sand.len(), 0)
}

fn parse(input: &str) -> Result<Vec<Vec<Point>>> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| {
                    let (a, b) = pair
                        .split_once(',')
                        .ok_or(anyhow::anyhow!("couldn't split pair"))?;

                    Ok((a.parse()?, b.parse()?))
                })
                .collect()
        })
        .collect()
}

fn print(source: Point, rocks: &HashSet<Point>, sand: &HashSet<Point>) {
    let rock_x_range = rocks
        .iter()
        .map(|&(x, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let rock_y_range = rocks
        .iter()
        .map(|&(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();

    let sand_x_range = sand.iter().map(|&(x, _)| x).minmax().into_option().unwrap();
    let sand_y_range = sand.iter().map(|&(_, y)| y).minmax().into_option().unwrap();

    let (min_x, max_x) = (
        rock_x_range.0.min(sand_x_range.0).min(source.0),
        rock_x_range.1.max(sand_x_range.1).max(source.0),
    );

    let (min_y, max_y) = (
        rock_y_range.0.min(sand_y_range.0).min(source.1),
        rock_y_range.1.max(sand_y_range.1).max(source.1),
    );

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if (x, y) == source {
                print!("+");
            } else if sand.contains(&(x, y)) {
                print!("O");
            } else if rocks.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (24, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (1_003, 0));
    });
}
