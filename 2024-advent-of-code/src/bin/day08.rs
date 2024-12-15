#![feature(test)]
#![feature(let_chains)]

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

extern crate test;

const EXAMPLE: &str = include_str!("example08.txt");
const INPUT: &str = include_str!("input08.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

type Point = (isize, isize);

fn solve(input: &str) -> (usize, usize) {
    let (nodes, height, width) = parse(input);

    let mut antinodes = HashSet::<Point>::new();

    for (_node, locs) in nodes.clone() {
        let new = {
            let locs = locs.clone();
            let mut antinodes = vec![];

            for (a, b) in locs.iter().tuple_combinations() {
                let &(ax, ay) = a;
                let &(bx, by) = b;

                let antinode = (ax + (ax - bx), ay + (ay - by));
                antinodes.push(antinode);

                let antinode = (bx + (bx - ax), by + (by - ay));

                antinodes.push(antinode);
            }

            antinodes
        };
        // for y in 0..=height {
        //     for x in 0..=width {
        //         let (x, y) = (x as isize, y as isize);
        //         let c = if new.contains(&(x, y)) {
        //             '#'
        //         } else if locs.contains(&(x, y)) {
        //             node
        //         } else {
        //             '.'
        //         };
        //         print!("{c}");
        //     }
        //     println!("");
        // }
        antinodes = antinodes.into_iter().chain(new.into_iter()).collect();
    }

    // get all node pair combos
    let lines: Vec<(Point, Point)> = nodes
        .iter()
        .flat_map(|(_c, locs)| locs.clone().into_iter().tuple_combinations::<(_, _)>())
        .collect();

    let mut part2_antinodes = HashSet::<Point>::new();

    for y in 0..=height {
        for x in 0..=width {
            let (x, y) = (x as isize, y as isize);

            let is_antinode = lines
                .iter()
                .any(|&((x1, y1), (x2, y2))| (x2 - x1) * (y - y1) - (y2 - y1) * (x - x1) == 0);

            if is_antinode {
                part2_antinodes.insert((x, y));
            }

            // let c = if is_antinode { '#' } else { '.' };
            // print!("{c}");
        }
        // println!("");
    }

    antinodes.retain(|&(x, y)| x >= 0 && y >= 0 && x <= width as isize && y <= height as isize);

    (antinodes.len(), part2_antinodes.len())
}

fn parse(input: &str) -> (HashMap<char, Vec<Point>>, usize, usize) {
    let mut nodes = HashMap::<char, Vec<Point>>::default();

    let mut height = 0;
    let mut width = 0;

    for (y, line) in input.trim().lines().enumerate() {
        height = height.max(y);
        for (x, c) in line.trim().chars().enumerate() {
            width = width.max(x);
            if c == '.' {
                continue;
            }
            nodes.entry(c).or_default().push((x as isize, y as isize));
        }
    }

    (nodes, height, width)
}

#[test]
fn test_small_example() {
    assert_eq!(
        solve(
            "T.........
            ...T......
            .T........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            .........."
        ),
        (3, 9)
    );

    assert_eq!(
        solve(
            "..........
            ..........
            ..........
            ....a.....
            ..........
            .....a....
            ..........
            ..........
            ..........
            .........."
        ),
        (2, 5)
    );

    assert_eq!(
        solve(
            "..........
            ..........
            ..........
            ....a.....
            ........a.
            .....a....
            ..........
            ..........
            ..........
            .........."
        ),
        (4, 8)
    );
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (14, 34));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (293, 934));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
