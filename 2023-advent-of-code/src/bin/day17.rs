// #![feature(test)]

// extern crate test;

use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example17.txt");
const INPUT: &str = include_str!("input17.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let map = Map::parse(input);

    println!("{}", map);

    (0, 0)
}

type Point = (usize, usize);

struct Map {
    loss: HashMap<Point, u8>,
    height: usize,
    width: usize,
}

impl Map {
    fn parse(input: &str) -> Map {
        let loss = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .flat_map(move |(x, c)| Some(((x, y), c.to_digit(10).unwrap() as u8)))
            })
            .collect::<HashMap<_, _>>();

        let (width, height) = loss.keys().fold((0, 0), |(max_x, max_y), &(x, y)| {
            (max_x.max(x), max_y.max(y))
        });

        let (width, height) = (width + 1, height + 1);

        Map {
            loss,
            width,
            height,
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y != 0 {
                writeln!(f, "")?;
            }
            for x in 0..self.width {
                write!(f, "{}", self.loss[&(x, y)])?;
            }
        }

        Ok(())
    }
}

#[test]
fn test_parse_debug() {
    assert_eq!(format!("{}", Map::parse(EXAMPLE)), EXAMPLE);
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (0, 0));
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
