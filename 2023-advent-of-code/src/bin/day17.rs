#![feature(let_chains)]
// #![feature(test)]

// extern crate test;

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

const EXAMPLE: &str = include_str!("example17.txt");
const INPUT: &str = include_str!("input17.txt");

fn main() {
    println!("part 1: {:?}", solve(EXAMPLE));
    dbg!(solve(INPUT));
}

const STRAIGHT_LINE_MAX: i32 = 3;

fn solve(input: &str) -> (usize, usize) {
    let map = Map::parse(input);

    println!("{}", map);

    let start = (0, 0);
    let end = (map.width - 1, map.height - 1);
    // TODO: can absence of distance mean unreachable?
    let mut distances = map
        .loss
        .keys()
        .map(|&pos| (pos, None))
        .collect::<HashMap<_, Option<usize>>>();
    distances.insert(start, Some(0));
    let mut previous = HashMap::<Point, Point>::new();
    // TODO: can we insert into "frontier" as we discover, to keep length of queue down
    let mut unvisited = map.loss.keys().collect::<Vec<_>>();
    while let Some(current) = unvisited
        .iter()
        .position_min_by(|&a, &b| match [a, b].map(|v| distances[v]) {
            [None, None] => Ordering::Equal,
            [Some(_), None] => Ordering::Less,
            [None, Some(_)] => Ordering::Greater,
            [Some(a), Some(b)] => a.cmp(&b),
        })
        .map(|pos| unvisited.swap_remove(pos))
    {
        let dist = distances[current];
        println!("visiting {current:?} @ distance {dist:?}",);

        let dist = dist.expect("uhhhh, 'unrechable' much??");

        let mut prevv = vec![];
        let mut prev = current;
        for _i in 0..STRAIGHT_LINE_MAX {
            if let Some(prev2) = previous.get(prev) {
                prevv.push(*prev2);
                prev = prev2;
            }
        }

        println!("prevv to this were: {prevv:?}");

        for neighbor in map.neighbors(current, &prevv) {
            let alt = dist + map.loss[&neighbor] as usize;
            println!("\tneighbor of {neighbor:?}, previously reachable with {:?} now reachable with {alt}", distances[&neighbor]);
            if distances[&neighbor].map_or(true, |distance| alt < distance) {
                distances.insert(neighbor, Some(alt));
                previous.insert(neighbor, *current);
            }
        }
    }

    let final_dist = distances[&end].expect("no path");
    println!("SOLVEDDDD with a distance of {final_dist}");

    let mut path = VecDeque::new(); // purely so we don't have to reverse
    let mut point = end;
    while point != start {
        path.push_front(point);
        point = previous[&point];
    }

    println!();

    for y in 0..map.height {
        if y != 0 {
            println!();
        }
        for x in 0..map.width {
            let c = if path.contains(&(x, y))
                && let Some(&prev) = previous.get(&(x, y))
            {
                match point_delta(x, y, &prev.0, &prev.1) {
                    (0, 1) => 'v',
                    (-1, 0) => '<',
                    (1, 0) => '>',
                    (0, -1) => '^',
                    _ => panic!(),
                }
                .to_string()
            } else {
                map.loss[&(x, y)].to_string()
            };
            print!("{}", c);
        }
    }

    (final_dist, 0)
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

    fn neighbors(&self, point: &Point, previous_three: &[Point]) -> Vec<Point> {
        let &(x, y) = point;

        if !self.loss.contains_key(&(x, y)) {
            return vec![];
        }

        let pp: Vec<(i64, i64)> = previous_three
            .iter()
            .map(|(px, py)| point_delta(x, y, px, py))
            .collect();

        let coming_from_straight_line = pp.len() >= 3 && pp[0] == pp[1] && pp[1] == pp[2];

        println!("previous three deltas of {:?}", pp);

        if coming_from_straight_line {
            println!("COMING FROM STRAIGHT LINE");
        }

        [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .cloned()
            .filter_map(|(dx, dy)| {
                if coming_from_straight_line && (pp[0].0, pp[0].1) == (dx, dy) {
                    return None;
                }

                if let Ok(x) = (x as i64 + dx).try_into()
                    && let Ok(y) = (y as i64 + dy).try_into()
                    && self.loss.contains_key(&(x, y))
                {
                    return Some((x, y));
                }

                None
            })
            .collect()
    }
}

fn point_delta(x: usize, y: usize, px: &usize, py: &usize) -> (i64, i64) {
    [(x, px), (y, py)]
        .map(|(current, previous)| match current.cmp(previous) {
            Ordering::Greater => 1,
            Ordering::Equal => 0,
            Ordering::Less => -1,
        })
        .into()
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y != 0 {
                writeln!(f)?;
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
    assert_eq!(solve(EXAMPLE), (102, 0));
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
