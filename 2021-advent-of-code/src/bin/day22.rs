#![feature(test)]

use fxhash::FxHashSet;
use regex::Regex;

extern crate test;

const EXAMPLE: &str = include_str!("example22.txt");
const INPUT: &str = include_str!("input22.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let active = input
        .lines()
        .fold(FxHashSet::<(i32, i32, i32)>::default(), |mut set, line| {
            let (order, x1, x2, y1, y2, z1, z2) = parse_step(line);

            println!("size will be {}", (x2 - x1) * (y2 - y1) * (z2 - z1));

            for x in x1..x2 {
                for y in y1..y2 {
                    for z in z1..z2 {
                        let cube = (x, y, z);
                        if order {
                            set.insert(cube);
                        } else {
                            set.remove(&cube);
                        }
                    }
                }
            }

            println!("completed step: {}", line);

            set
        });

    (active.len(), 0)
}

fn parse_step(line: &str) -> (bool, i32, i32, i32, i32, i32, i32) {
    // TODO: can we do this at compile time or something? I could have sworn this was the point of
    // this particular library.
    let regex = Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$")
        .expect("couldn't compile regex");

    let caps = regex.captures(line).expect("couldn't parse line");

    // NOTE: we can unwrap because regex guarantees things will be digits
    (
        match &caps[1] {
            "on" => true,
            "off" => false,
            _ => panic!(),
        },
        caps[2].parse().unwrap(),
        caps[3].parse().unwrap(),
        caps[4].parse().unwrap(),
        caps[5].parse().unwrap(),
        caps[6].parse().unwrap(),
        caps[7].parse().unwrap(),
    )
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (590784, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
