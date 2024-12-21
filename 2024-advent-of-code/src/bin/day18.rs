// #![feature(test)]

// extern crate test;

use std::collections::{HashMap, HashSet, VecDeque};

type Problem = (&'static str, Point, usize);

const EXAMPLE: Problem = (include_str!("example18.txt"), (6, 6), 12);

const INPUT: Problem = (include_str!("input18.txt"), (70, 70), 1024);

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve((input, (w, h), time): Problem) -> (usize, usize) {
    let locs = parse(input);

    let mut map = HashSet::<Point>::new();

    for &(x, y) in locs.iter().take(time) {
        map.insert((x, y));
    }

    for y in 0..=h {
        for x in 0..=w {
            let c = if map.contains(&(x, y)) { '#' } else { '.' };
            print!("{c}");
        }
        println!();
    }

    let start = (0, 0);
    let end = (w, h);

    let p = pathfind(map, start, end).unwrap();

    (p, 0)
}
const NEIGHBORS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn pathfind(corrupted: HashSet<Point>, start: Point, end: Point) -> Option<usize> {
    let mut frontier = VecDeque::new();
    frontier.push_back(start);

    let mut dists = HashMap::<Point, usize>::new();
    dists.insert(start, 0);

    let mut prev = HashMap::<Point, Point>::new();

    while let Some(current) = frontier.pop_front() {
        let current_dist = dists[&current];
        if current == end {
            let mut path = HashSet::new();
            let mut pointer = end;
            while pointer != start {
                path.insert(pointer);
                pointer = prev[&pointer];
            }

            for y in 0..=end.1 {
                for x in 0..=end.0 {
                    let c = if corrupted.contains(&(x, y)) {
                        '#'
                    } else if path.contains(&(x, y)) {
                        'O'
                    } else {
                        '.'
                    };
                    print!("{c}");
                }
                println!();
            }
            return Some(current_dist);
        }

        for neighbor in NEIGHBORS.map(|(dx, dy)| (current.0 + dx, current.1 + dy)) {
            let (x, y) = neighbor;
            if x < 0 || x > end.0 || y < 0 || y > end.1 {
                continue;
            }

            if corrupted.contains(&(x, y)) {
                continue;
            }

            let dist = current_dist + 1;
            if dists.get(&(x, y)).is_none_or(|&d| dist < d) {
                dists.insert((x, y), dist);
                prev.insert(neighbor, current);
                frontier.push_back(neighbor);
            }
        }
    }

    None
}

type Point = (isize, isize);

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            [x, y].map(|n| n.parse().unwrap()).into()
        })
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (22, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (298, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
