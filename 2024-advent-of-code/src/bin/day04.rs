// #![feature(test)]

// extern crate test;

use core::iter::Iterator;
use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example04.txt");
const INPUT: &str = include_str!("input04.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

const DIRECTIONS: [(isize, isize); 8] = [
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (1, 0),
];

fn solve(input: &str) -> (usize, usize) {
    let search = parse(input);

    let mut xmases = 0;

    for ((x0, y0), &c0) in search.iter() {
        for (dx, dy) in DIRECTIONS {
            let Some(&c1) = search.get(&(x0 + dx, y0 + dy)) else {
                continue;
            };
            let Some(&c2) = search.get(&(x0 + dx * 2, y0 + dy * 2)) else {
                continue;
            };
            let Some(&c3) = search.get(&(x0 + dx * 3, y0 + dy * 3)) else {
                continue;
            };

            if [c0, c1, c2, c3] == ['X', 'M', 'A', 'S'] {
                xmases += 1;
            }
        }
    }

    (xmases, 0)
}

fn parse(input: &str) -> HashMap<(isize, isize), char> {
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as isize, y as isize), c);
        }
    }

    map
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (18, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (2434, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
