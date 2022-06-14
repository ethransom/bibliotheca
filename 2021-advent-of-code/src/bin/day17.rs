#![feature(test)]

use regex::Regex;

extern crate test;

const EXAMPLE: &str = include_str!("example17.txt");
const INPUT: &str = include_str!("input17.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let re = Regex::new(r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)\n?$").unwrap();
    let cap = re.captures(input).expect("parse error");

    let nums = [&cap[1], &cap[2], &cap[3], &cap[4]]
        .iter()
        .map(|cap| cap.parse::<i32>().expect("not a number"))
        .collect::<Vec<_>>();

    let mut max = nums.iter().map(|n| n.abs()).max().unwrap();

    max *= 2;

    let [x1, x2, y1, y2]: [i32; 4] = nums.try_into().unwrap();
    let (a, b) = ((x1, y1), (x2, y2));
    println!("a = ({}, {}) b = ({}, {})", x1, y1, x2, y2);

    let mut heights: Vec<i32> = vec![];

    for y in -max..max {
        for x in -max..max {
            // simulate for vel x, y, discarding those that don't intersect dest and keeping max
            // height
            if let Some(height) = stepped_intersect(x, y, a, b) {
                heights.push(height);
            }
        }
    }

    (*heights.iter().max().unwrap() as usize, heights.len())
}

/// Returns Some height obtained if given initial velocity intersects given bounds, None if no
/// intersection.
fn stepped_intersect(mut vx: i32, mut vy: i32, a: (i32, i32), b: (i32, i32)) -> Option<i32> {
    let (mut x, mut y) = (0, 0);
    let mut max_y = 0;
    for _i in 0..1000 {
        x += vx;
        y += vy;

        // drag
        if vx > 0 {
            vx -= 1;
        }
        // gravity
        vy -= 1;

        if y > max_y {
            max_y = y;
        }

        if x >= a.0 && x <= b.0 && y >= a.1 && y <= b.1 {
            return Some(max_y);
        }
    }

    None
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (45, 112));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (9730, 4110));
    });
}
