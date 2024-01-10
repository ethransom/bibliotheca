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

    let mut undug = HashSet::<Point>::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if !dug.contains(&(x, y)) {
                undug.insert((x, y));
            }
        }
    }
    while let Some(&(x, y)) = undug.iter().next() {
        // println!("random visit: {pos:?}", pos = (x, y));
        let mut outside = false;
        let mut region = HashSet::new();
        let mut unvisited = Vec::<Point>::new();
        unvisited.push((x, y));
        while let Some((x, y)) = unvisited.pop() {
            undug.remove(&(x, y));
            region.insert((x, y));
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let (x, y) = (x + dx, y + dy);
                // println!("neighbor of: {pos:?}", pos = (x, y));
                if x < min_x || x > max_x || y < min_y || y > max_y {
                    // println!("\t out of bounds");
                    outside = true;
                    continue;
                }
                if dug.contains(&(x, y)) {
                    continue;
                }
                if region.contains(&(x, y)) {
                    continue;
                }
                unvisited.push((x, y));
            }
        }
        // println!(
        //     "region {region:?} was {outside}outside",
        //     outside = if outside { "" } else { "not " }
        // );
        if !outside {
            dug.extend(region);
        }
    }
    let count = dug.len();

    for &(_dir, _amount, color) in &plan {
        let color = color
            .trim_start_matches('(')
            .trim_start_matches('#')
            .trim_end_matches(')');
        assert_eq!(color.len(), 6, "invalid hex digits: '{color}'");
        let amount = i64::from_str_radix(&color[..5], 16).expect("invalid hex number");
        let (dx, dy, d) = match color[5..].parse::<u8>() {
            // 0 means R, 1 means D, 2 means L, and 3 means U
            Ok(0) => (1, 0, 'R'),
            Ok(1) => (0, 1, 'D'),
            Ok(2) => (-1, 0, 'L'),
            Ok(3) => (0, -1, 'U'),
            err => panic!("error with dir digit: '{err:?}"),
        };
        println!("#{color} -> {d} {amount}");
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
