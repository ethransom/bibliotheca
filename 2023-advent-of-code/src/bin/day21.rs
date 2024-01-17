#![feature(test)]

extern crate test;

use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};

const EXAMPLE: &str = include_str!("example21.txt");
const INPUT: &str = include_str!("input21.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let map = parse(input);

    // println!("{:?}", map);

    let steps = 64;

    let mut distance = HashMap::default();
    distance.insert(map.start, 0);
    let mut queue = VecDeque::new();
    queue.push_back(map.start);
    while let Some(point) = queue.pop_front() {
        let dist = distance[&point];
        if dist > steps {
            continue;
        }
        let Point { x, y } = point;
        for (x, y) in [
            (x, y - 1), // never
            (x + 1, y), // eat
            (x, y + 1), // soggy
            (x - 1, y), // waffles
        ] {
            let n = Point { x, y };

            if distance.contains_key(&n) {
                continue;
            }

            if *map.tiles.get(&n).unwrap_or(&'.') == '#' {
                continue;
            }

            distance.insert(n, dist + 1);
            queue.push_back(n);
        }
    }
    // for i in 1..=3 {
    //     let mut out = String::new();
    //     for y in 0..map.height {
    //         if y != 0 {
    //             out.push('\n');
    //         }
    //         for x in 0..map.width {
    //             let point = Point {
    //                 x: x as isize,
    //                 y: y as isize,
    //             };
    //             let c = if distance.contains_key(&point)
    //                 && distance[&point] <= i
    //                 && distance[&point] % 2 == i % 2
    //             {
    //                 'O'
    //             } else if map.start == point {
    //                 'S'
    //             } else {
    //                 map.tiles[&point]
    //             };
    //
    //             out.push(c);
    //         }
    //     }
    //
    //     println!("\n{}", out);
    // }

    let stepped = distance
        .values()
        .filter(|&dist| dist % 2 == steps % 2)
        .count();

    (stepped, 0)
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
    let mut tiles = HashMap::default();

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

impl Map {
    fn fmt_with_steps(&self, stepped: &HashSet<Point>) -> String {
        let mut out = String::new();
        for y in 0..self.height {
            if y != 0 {
                out.push('\n');
            }
            for x in 0..self.width {
                let point = Point {
                    x: x as isize,
                    y: y as isize,
                };
                let c = if stepped.contains(&point) {
                    'O'
                } else if self.start == point {
                    'S'
                } else {
                    self.tiles[&point]
                };

                out.push(c);
            }
        }

        out
    }
}

#[test]
fn test_parse_display() {
    assert_eq!(format!("{:?}", parse(EXAMPLE)), EXAMPLE);
    assert_eq!(format!("{:?}", parse(INPUT)), INPUT);
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (4056, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (3689, 0));
}

#[bench]
fn bench_solve_03_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (3689, 0));
    });
}

#[bench]
fn bench_solve_02_steps(b: &mut test::Bencher) {
    fn solve(input: &str) -> (usize, usize) {
        let map = parse(input);

        // println!("{:?}", map);

        let mut stepped = HashSet::default();
        stepped.insert(map.start);
        let mut next = HashSet::default();
        for _i in 1..=64 {
            for &Point { x, y } in &stepped {
                for (x, y) in [
                    (x, y - 1), // never
                    (x + 1, y), // eat
                    (x, y + 1), // soggy
                    (x - 1, y), // waffles
                ] {
                    let n = Point { x, y };

                    if *map.tiles.get(&n).unwrap_or(&'.') == '.' {
                        next.insert(n);
                    }
                }
            }

            std::mem::swap(&mut stepped, &mut next);
            next.clear();

            // println!("\n{}", map.fmt_with_steps(&stepped));
        }

        (stepped.len(), 0)
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (3689, 0));
    });
}

#[bench]
fn bench_solve_01_fxhash(b: &mut test::Bencher) {
    fn solve(input: &str) -> (usize, usize) {
        let map = parse(input);

        // println!("{:?}", map);

        let mut stepped = HashSet::default();
        stepped.insert(map.start);
        for _i in 1..=64 {
            let mut next = HashSet::default();

            for &Point { x, y } in &stepped {
                for (x, y) in [
                    (x, y - 1), // never
                    (x + 1, y), // eat
                    (x, y + 1), // soggy
                    (x - 1, y), // waffles
                ] {
                    let n = Point { x, y };

                    if *map.tiles.get(&n).unwrap_or(&'.') == '.' {
                        next.insert(n);
                    }
                }
            }

            stepped = next;

            // println!("\n{}", map.fmt_with_steps(&stepped));
        }

        (stepped.len(), 0)
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (3689, 0));
    });
}

#[bench]
fn bench_solve_00_original(b: &mut test::Bencher) {
    use std::collections::HashSet;
    fn solve(input: &str) -> (usize, usize) {
        let map = parse(input);

        // println!("{:?}", map);

        let mut stepped = HashSet::from([map.start]);
        for _i in 1..=64 {
            let mut next = HashSet::default();

            for &Point { x, y } in &stepped {
                for (x, y) in [
                    (x, y - 1), // never
                    (x + 1, y), // eat
                    (x, y + 1), // soggy
                    (x - 1, y), // waffles
                ] {
                    let n = Point { x, y };

                    if *map.tiles.get(&n).unwrap_or(&'.') == '.' {
                        next.insert(n);
                    }
                }
            }

            stepped = next;

            // println!("\n{}", map.fmt_with_steps(&stepped));
        }

        (stepped.len(), 0)
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (3689, 0));
    });
}
