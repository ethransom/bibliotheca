#![feature(test)]

use std::{collections::HashSet, ops::Add};

extern crate test;

const EXAMPLE: &str = include_str!("example17.txt");
const INPUT: &str = include_str!("input17.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

type Point = (i32, i32);

fn blocks() -> impl Iterator<Item = Vec<Point>> {
    [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],         // _
        vec![(0, 1), (1, 2), (1, 1), (1, 0), (2, 1)], // +
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], //backwards L
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],         // |
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],         // square
    ]
    .into_iter()
    .cycle()
}

fn solve(input: &str) -> (i32, i32) {
    let mut jets = parse(input).into_iter().cycle();

    let mut chamber: HashSet<Point> = HashSet::new();

    for (i, shape) in blocks().enumerate() {
        if i + 1 > 2022 {
            break;
        }

        // println!("block {}", i + 1);

        let mut pos = (2, chamber.iter().fold(-1, |m, &(_x, y)| m.max(y)).add(4));

        // println!("first rock");
        // print(&chamber, &pos, &shape);

        for jet in jets.by_ref() {
            let mut new_pos = pos;

            let delta = match jet {
                Jet::Left => -1,
                Jet::Right => 1,
            };

            new_pos.0 += delta;

            if shape
                .iter()
                .any(|p| p.0 + new_pos.0 > 6 || p.0 + new_pos.0 < 0)
                || shape
                    .iter()
                    .any(|p| chamber.contains(&(p.0.add(new_pos.0), p.1.add(new_pos.1))))
            {
                // println!("couldn't move");
            } else {
                pos = new_pos;
            }

            // println!(
            //     "after jet: {:#?}",
            //     match jet {
            //         Jet::Left => "<",
            //         Jet::Right => ">",
            //     }
            // );
            // print(&chamber, &pos, &shape);

            new_pos = pos;
            new_pos.1 -= 1;

            if new_pos.1 < 0
                || shape
                    .iter()
                    .any(|p| chamber.contains(&(p.0.add(new_pos.0), p.1.add(new_pos.1))))
            {
                // collision
                // println!("boop");
                break;
            } else {
                pos = new_pos;
            }

            // println!("after falling");
            // print(&chamber, &pos, &shape);
        }

        for p in shape.iter().map(|p| (p.0.add(pos.0), p.1.add(pos.1))) {
            chamber.insert(p);
        }

        // println!("after placement");
        // print(&chamber, &pos, &shape);
    }

    (chamber.iter().fold(0, |m, &(_x, y)| m.max(y + 1)), 0)
}

#[derive(Clone)]
enum Jet {
    Left,
    Right,
}

fn print(chamber: &HashSet<Point>, pos: &Point, shape: &[Point]) {
    let max_y = shape
        .iter()
        .map(|p| (p.0 + pos.0, p.1 + pos.1))
        .chain(chamber.iter().cloned())
        .fold(0, |max_y, (_x, y)| max_y.max(y));

    for y in (0..=max_y.max(3)).rev() {
        print!("|");
        for x in 0..7 {
            if chamber.contains(&(x, y)) {
                print!("#");
            } else if shape.contains(&(x - pos.0, y - pos.1)) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!("|");
    }

    println!("+{}+", "-".repeat(7));
}

fn parse(input: &str) -> Vec<Jet> {
    input
        .chars()
        .map(|c| match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("invalid input"),
        })
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (3_068, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (3_177, 0));
    });
}
