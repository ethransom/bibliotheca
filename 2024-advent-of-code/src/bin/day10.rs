#![feature(test)]

extern crate test;

use std::collections::{HashMap, HashSet};

const EXAMPLE: &str = include_str!("example10.txt");
const INPUT: &str = include_str!("input10.txt");

type Point = (isize, isize);

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

const NEIGHBORS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn solve(input: &str) -> (usize, usize) {
    let map = parse(input);

    // println!("{map}");

    let starts = map
        .map
        .iter()
        .filter(|&(_pos, h)| *h == 9)
        .map(|(pos, _h)| *pos);

    // println!("{s:?}", s = starts.clone().collect::<Vec<_>>());

    let mut trailhead_scores = HashMap::<Point, HashMap<Point, usize>>::new();

    for start in starts {
        let visited = HashSet::<Point>::from([start]);
        let mut stack = vec![(start, visited)];

        while let Some(((x, y), visited)) = stack.pop() {
            let &height = map.map.get(&(x, y)).unwrap();

            if height == 0 {
                // valid trailhead
                trailhead_scores
                    .entry((x, y))
                    .or_default()
                    .entry(start)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }

            for (dx, dy) in NEIGHBORS {
                let neighbor = (x + dx, y + dy);
                let Some(neighbor_height) = map.map.get(&neighbor) else {
                    continue;
                };

                if neighbor_height + 1 == height && !visited.contains(&neighbor) {
                    let visited = visited.clone();
                    stack.push((neighbor, visited));
                }
            }
        }
    }

    // for y in 0..map.height {
    //     for x in 0..map.width {
    //         let c = trailhead_scores
    //             .get(&(x as isize, y as isize))
    //             .map_or('.', |&c| char::from_digit(c as u32, 10).unwrap());

    //         print!("{c}");
    //     }
    //     println!();
    // }

    (
        // number of paths unique by starting point
        trailhead_scores
            .values()
            .map(|m| m.keys().collect::<HashSet<_>>().len())
            .sum(),
        // number of paths summed by starting point
        trailhead_scores.values().flat_map(|m| m.values()).sum(),
    )
}

#[derive(Clone)]
struct Map {
    map: HashMap<Point, u8>,
    height: usize,
    width: usize,
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.map.get(&(x as isize, y as isize)).unwrap();
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn parse(input: &str) -> Map {
    let mut map = HashMap::default();

    let mut height = 0;
    let mut width = 0;

    for (y, line) in input.lines().enumerate() {
        height = height.max(y);
        for (x, c) in line.chars().enumerate() {
            width = width.max(x);
            map.insert(
                (x as isize, y as isize),
                c.to_digit(10).unwrap().try_into().unwrap(),
            );
        }
    }

    Map { map, height, width }
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (36, 81));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (611, 1380));
}

#[bench]
fn bench_solve_00_current(b: &mut test::Bencher) {
    b.iter(|| {
        test_input();
    });
}

#[bench]
fn bench_solve_01_current_fxhash(b: &mut test::Bencher) {
    use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};

    fn solve(input: &str) -> (usize, usize) {
        let map = parse(input);

        // println!("{map}");

        let starts = map
            .map
            .iter()
            .filter(|&(_pos, h)| *h == 9)
            .map(|(pos, _h)| *pos);

        let mut trailhead_scores = HashMap::<Point, HashMap<Point, usize>>::default();

        for start in starts {
            let mut visited = HashSet::<Point>::default();
            visited.insert(start);
            let mut stack = vec![(start, visited)];

            while let Some(((x, y), visited)) = stack.pop() {
                let &height = map.map.get(&(x, y)).unwrap();

                if height == 0 {
                    // valid trailhead
                    trailhead_scores
                        .entry((x, y))
                        .or_default()
                        .entry(start)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }

                for (dx, dy) in NEIGHBORS {
                    let neighbor = (x + dx, y + dy);
                    let Some(neighbor_height) = map.map.get(&neighbor) else {
                        continue;
                    };

                    if neighbor_height + 1 == height && !visited.contains(&neighbor) {
                        let visited = visited.clone();
                        stack.push((neighbor, visited));
                    }
                }
            }
        }

        (
            trailhead_scores
                .values()
                .map(|m| m.keys().collect::<HashSet<_>>().len())
                .sum(),
            trailhead_scores.values().flat_map(|m| m.values()).sum(),
        )
    }

    #[derive(Clone)]
    struct Map {
        map: HashMap<Point, u8>,
    }

    fn parse(input: &str) -> Map {
        let mut map = HashMap::default();

        let mut height = 0;
        let mut width = 0;

        for (y, line) in input.lines().enumerate() {
            height = height.max(y);
            for (x, c) in line.chars().enumerate() {
                width = width.max(x);
                map.insert(
                    (x as isize, y as isize),
                    c.to_digit(10).unwrap().try_into().unwrap(),
                );
            }
        }

        Map { map }
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (611, 1380));
    });
}
