// #![feature(test)]

// extern crate test;

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

const EXAMPLE: &str = include_str!("example21.txt");
const INPUT: &str = include_str!("input21.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let map = parse(input);

    println!("{:?}", map);

    (0, 0)
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

struct Map {
    tiles: HashMap<Point, char>,
    start: Point,
    height: usize,
    width: usize,
}

fn parse(input: &str) -> Map {
    let mut start = None;
    let mut tiles = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, mut char) in line.chars().enumerate() {
            let point = Point {
                x: x as isize,
                y: y as isize,
            };
            if char == 'S' {
                start = Some(point);
                char = '.'
            }
            tiles.insert(point, char);
        }
    }

    let start = start.expect("map did not have start marked");

    let (max_x, max_y) = tiles
        .iter()
        .fold((0, 0), |(width, height), (&Point { x, y }, _char)| {
            (width.max(x), height.max(y))
        });

    let (width, height) = (max_x as usize + 1, max_y as usize + 1);

    Map {
        tiles,
        width,

        start,

        height,
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y != 0 {
                writeln!(f)?;
            }
            for x in 0..self.width {
                let point = Point {
                    x: x as isize,
                    y: y as isize,
                };
                let c = if self.start == point {
                    'S'
                } else {
                    self.tiles[&point]
                };
                write!(f, "{}", c)?;
            }
        }

        Ok(())
    }
}

#[test]
fn test_parse_display() {
    assert_eq!(format!("{:?}", parse(EXAMPLE)), EXAMPLE);
    assert_eq!(format!("{:?}", parse(INPUT)), INPUT);
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
