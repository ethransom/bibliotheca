#![feature(test)]
#![feature(array_try_map)]

extern crate test;

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

// https://en.wikipedia.org/wiki/Rotation_matrix#Common_2D_rotations
const ROTATIONS: [[[isize; 2]; 2]; 4] = [
    [[1, 0], [0, 1]],   // 0°
    [[0, -1], [1, 0]],  // 90°
    [[-1, 0], [0, -1]], // 180°
    [[0, 1], [-1, 0]],  // 270°
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

    let mut x_mases = 0;

    fn rotate((x, y): (isize, isize), rot: [[isize; 2]; 2]) -> (isize, isize) {
        (rot[0][0] * x + rot[0][1] * y, rot[1][0] * x + rot[1][1] * y)
    }

    for (x1, y1) in search.keys() {
        let pattern = [[0, 0], [1, 1], [2, 2], [2, 0], [1, 1], [0, 2]];

        for rot in ROTATIONS {
            let pattern = pattern.map(|[x, y]| rotate((x, y), rot));

            let positions = pattern.map(|(dx, dy)| (x1 + dx, y1 + dy));

            let Some(values) = positions.try_map(|(x, y)| search.get(&(x, y)).copied()) else {
                continue;
            };

            if values == ['M', 'A', 'S', 'M', 'A', 'S'] {
                x_mases += 1;
            }
        }
    }

    (xmases, x_mases)
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
    assert_eq!(solve(EXAMPLE), (18, 9));
}

#[test]
fn test_easy() {
    assert_eq!(solve("MYM\nYAY\nSYS"), (0, 1));
}
#[test]
fn test_rotation() {
    assert_eq!(solve("SYS\nYAY\nMYM"), (0, 1));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (2434, 1835));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        test_input();
    });
}
