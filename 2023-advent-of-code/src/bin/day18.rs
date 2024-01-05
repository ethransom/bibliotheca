// #![feature(test)]

// extern crate test;

use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example18.txt");
const INPUT: &str = include_str!("input18.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

type Point = (isize, isize);

fn solve(input: &str) -> (usize, usize) {
    let plan = parse(input);

    let mut dug = HashSet::<Point>::new();
    let mut position = (0, 0);
    dug.insert(position);
    for &(dir, amount, _color) in &plan {
        let (dx, dy) = match dir {
            "U" => (0, -1),
            "R" => (1, 0),
            "D" => (0, 1),
            "L" => (-1, 0),
            _ => panic!(),
        };
        for _ in 0..amount {
            let (x, y) = position;
            position = (x + dx, y + dy);

            dug.insert(position);
        }
    }

    let ((min_x, min_y), (max_x, max_y)) = dug.iter().fold(
        ((0, 0), (0, 0)),
        |((min_x, min_y), (max_x, max_y)), &(x, y)| {
            ((min_x.min(x), min_y.min(y)), (max_x.max(x), max_y.max(y)))
        },
    );

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = if dug.contains(&(x, y)) { '#' } else { '.' };
            print!("{}", c);
        }
        println!();
    }

    let mut count = 0;
    for y in min_y..=max_y {
        let mut in_region = false;
        let mut line = String::new();
        let mut io = String::new();
        for x in min_x..=max_x {
            let c = if dug.contains(&(x, y)) { '#' } else { '.' };
            line.push(c);
            let c = if in_region { 'I' } else { 'O' };
            io.push(c);
            if dug.contains(&(x, y)) {
                in_region = !in_region;
            }
            if in_region {
                count += 1;
            }
        }
        println!("{line}");
        println!("{io}");
        println!();
    }

    (count, 0)
}

fn parse(input: &str) -> Vec<(&str, usize, &str)> {
    input
        .lines()
        .map(|line| {
            let [dir, amount, color] = line
                .split_whitespace()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            (dir, amount.parse().unwrap(), color)
        })
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (62, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (0, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
