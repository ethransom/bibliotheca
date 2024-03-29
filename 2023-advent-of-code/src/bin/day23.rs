// #![feature(test)]

// extern crate test;

use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::thread;

const STACK_SIZE: usize = 8 * 1024 * 1024;

const EXAMPLE: &str = include_str!("example23.txt");
const INPUT: &str = include_str!("input23.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let map = parse(input);

    let start = Point { x: 1, y: 0 };
    let end = Point {
        x: map.width - 2,
        y: map.height - 1,
    };

    assert_eq!(map.tiles[&start], '.');
    assert_eq!(map.tiles[&end], '.');

    // Spawn thread with explicit stack size
    thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(move || {
            (
                map.longest_path(&start, &end, false)
                    .expect("no longest downhill path"),
                map.longest_path(&start, &end, true)
                    .expect("no longest uphill/downhill path"),
            )
        })
        .expect("couldn't spawn big boi thread")
        .join()
        .expect("child thread couldn't compute longest path")
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

struct Map {
    width: usize,
    height: usize,

    tiles: HashMap<Point, char>,
}

impl Map {
    fn longest_path(&self, start: &Point, end: &Point, allow_uphill: bool) -> Option<usize> {
        dbg!(self.longest_path_2(&mut Default::default(), start, end, allow_uphill))
    }
    fn longest_path_2(
        &self,
        visited: &mut HashSet<Point>,
        start: &Point,
        end: &Point,
        allow_uphill: bool,
    ) -> Option<usize> {
        let pos = start;

        visited.insert(*pos);

        if pos == end {
            return Some(0);
        }

        let mut neighbors = vec![];
        // UP
        // only unwalled boundary is above start
        if pos.y != 0 {
            let n = Point {
                x: pos.x,
                y: pos.y - 1,
            };
            let v = self.tiles.get(&n).unwrap_or(&'#');
            if v == &'.'
                || v == &'^'
                || (allow_uphill && (v == &'^' || v == &'>' || v == &'v' || v == &'<'))
            {
                neighbors.push(n);
            }
        }
        // RIGHT
        let n = Point {
            x: pos.x + 1,
            y: pos.y,
        };
        let v = self.tiles.get(&n).unwrap_or(&'#');
        if v == &'.'
            || v == &'>'
            || (allow_uphill && (v == &'^' || v == &'>' || v == &'v' || v == &'<'))
        {
            neighbors.push(n);
        }
        // DOWN
        let n = Point {
            x: pos.x,
            y: pos.y + 1,
        };
        let v = self.tiles.get(&n).unwrap_or(&'#');
        if v == &'.'
            || v == &'v'
            || (allow_uphill && (v == &'^' || v == &'>' || v == &'v' || v == &'<'))
        {
            neighbors.push(n);
        }
        // LEFT
        let n = Point {
            x: pos.x - 1,
            y: pos.y,
        };
        let v = self.tiles.get(&n).unwrap_or(&'#');
        if v == &'.'
            || v == &'<'
            || (allow_uphill && (v == &'^' || v == &'>' || v == &'v' || v == &'<'))
        {
            neighbors.push(n);
        }

        neighbors.retain(|n| !visited.contains(n));

        // dbg!(pos, &neighbors);

        if neighbors.is_empty() {
            return None;
        }

        if neighbors.len() > 1 {
            return neighbors
                .iter()
                .map(|neighbor| {
                    self.longest_path_2(&mut visited.clone(), neighbor, end, allow_uphill)
                        .map(|d| d + 1)
                })
                .max()
                .unwrap();
        }

        self.longest_path_2(visited, &neighbors[0], end, allow_uphill)
            .map(|d| d + 1)
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y != 0 {
                writeln!(f)?;
            }
            for x in 0..self.width {
                write!(f, "{}", self.tiles[&Point { x, y }])?;
            }
        }

        Ok(())
    }
}

fn parse(input: &str) -> Map {
    let tiles: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(move |(c, t)| (Point { x: c, y: r }, t))
        })
        .collect();

    let (max_x, max_y) = tiles
        .iter()
        .fold((0, 0), |(width, height), (&Point { x, y }, _char)| {
            (width.max(x), height.max(y))
        });

    let (width, height) = (max_x + 1, max_y + 1);

    Map {
        tiles,
        width,

        height,
    }
}

#[test]
fn test_parse_display() {
    assert_eq!(format!("{:?}", parse(EXAMPLE)), EXAMPLE);
    assert_eq!(format!("{:?}", parse(INPUT)), INPUT);
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (94, 154));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (2306, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
