// #![feature(test)]

// extern crate test;

use std::collections::{HashMap, HashSet};

const EXAMPLE: &str = include_str!("example12.txt");
const INPUT: &str = include_str!("input12.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

const NEIGHBORS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn solve(input: &str) -> (usize, usize) {
    let map = parse(input);

    let mut regions = vec![];
    let mut visited = HashSet::new();
    for (&start, &c) in map.map.iter() {
        if visited.contains(&start) {
            continue;
        }
        let mut stack = vec![start];
        let mut region = HashSet::from([start]);
        while let Some(point) = stack.pop() {
            for neighbor in NEIGHBORS {
                let neighbor = (point.0 + neighbor.0, point.1 + neighbor.1);
                if map.map.get(&neighbor) == Some(&c) && !region.contains(&neighbor) {
                    stack.push(neighbor);
                    region.insert(neighbor);
                    visited.insert(neighbor);
                }
            }
        }
        regions.push((c, region));
    }

    // for (c, points) in regions.iter() {
    //     println!("{c}: {:?}", points);

    //     for y in 0..=map.height {
    //         for x in 0..=map.width {
    //             let c = if points.contains(&(x as isize, y as isize)) {
    //                 *c
    //             } else {
    //                 '.'
    //             };

    //             print!("{c}");
    //         }
    //         println!();
    //     }
    //     println!("PER: {a}", a = perimeter(points));
    //     println!("AREA: {a}", a = area(points));
    // }

    let fence_price = regions
        .iter()
        .map(|(_c, points)| area(points) * perimeter(points))
        .sum();

    (fence_price, 0)
}

fn area(points: &HashSet<(isize, isize)>) -> usize {
    points.len()
}

fn perimeter(points: &HashSet<(isize, isize)>) -> usize {
    // assume every point is an island
    let mut area = points.len() * 4;

    // subtract out the shared edges
    for point in points.iter() {
        for neighbor in NEIGHBORS {
            let neighbor = (point.0 + neighbor.0, point.1 + neighbor.1);
            if points.contains(&neighbor) {
                // you might think we'd subtract 2, for each shared edge,
                // but the "other" side of the shared fence will be gotten to in time
                area -= 1;
            }
        }
    }

    area
}

type Point = (isize, isize);

#[derive(Clone)]
struct Map {
    map: HashMap<Point, char>,
    height: usize,
    width: usize,
}

fn parse(input: &str) -> Map {
    let mut map = HashMap::default();

    let mut height = 0;
    let mut width = 0;

    for (y, line) in input.lines().enumerate() {
        height = height.max(y);
        for (x, c) in line.trim().chars().enumerate() {
            width = width.max(x);
            map.insert((x as isize, y as isize), c);
        }
    }

    Map { map, height, width }
}

#[test]
fn test_small_example() {
    assert_eq!(
        solve(
            "AAAA
    BBCD
    BBCC
    EEEC"
        ),
        (140, 0)
    )
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (1930, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (1471452, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
