// #![feature(test)]

// extern crate test;

use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

const EXAMPLE: &str = include_str!("example20.txt");
const INPUT: &str = include_str!("input20.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

const NEIGHBORS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn solve(input: &str) -> (usize, usize) {
    let map = parse(input);

    let mut dists = HashMap::<Point, usize>::new();
    dists.insert(map.start, 0);
    let mut queue = VecDeque::new();
    queue.push_back((map.start, 0));
    while let Some((node, dist)) = queue.pop_front() {
        if node == map.end {
            break;
        }

        let neighbors = NEIGHBORS.iter().map(|(dx, dy)| Point {
            x: node.x + dx,
            y: node.y + dy,
        });
        for neighbor in neighbors {
            if !map.walkable.contains(&neighbor) {
                continue;
            }

            if dists.contains_key(&neighbor) {
                continue;
            }
            dists.insert(neighbor, dist + 1);
            queue.push_back((neighbor, dist + 1));
        }
    }

    for y in 0..=map.height {
        for x in 0..=map.width {
            let c = dists
                .get(&Point {
                    x: x as isize,
                    y: y as isize,
                })
                .and_then(|d| d.to_string().chars().last())
                .unwrap_or(' ');
            print!("{c}");
        }
        println!("");
    }

    println!("{cost}", cost = dists[&map.end]);

    let mut savings = HashMap::<usize, usize>::new();
    let mut queue = VecDeque::new();
    queue.push_back((map.start, 0, 1));
    let mut visited = HashSet::new();
    while let Some((node, dist, cheatleft)) = queue.pop_front() {
        // if node == map.end {
        //     break;
        // }

        if let Some(&prev) = dists.get(&node) {
            if dist < prev {
                let diff = prev - dist;
                println!("found better path, saved {diff}",);

                *savings.entry(diff).or_default() += 1;
            }
        }

        let neighbors = NEIGHBORS.iter().map(|(dx, dy)| Point {
            x: node.x + dx,
            y: node.y + dy,
        });
        for neighbor in neighbors {
            if !map.walkable.contains(&neighbor) {
                if cheatleft <= 0 {
                    continue;
                }
                queue.push_back((neighbor, dist + 1, cheatleft - 1));
            } else if visited.insert((node, neighbor)) {
                queue.push_back((neighbor, dist + 1, cheatleft));
            }
        }
    }

    let savings = savings.into_iter().sorted().collect_vec();

    for (saves, count) in savings {
        println!("{count} that save {saves}");
    }

    unreachable!("no solution?");

    (0, 0)
}

fn print_visited(map: &Map, visited: &HashSet<Point>) {
    for y in 0..=map.height {
        for x in 0..=map.width {
            let c = if visited.contains(&Point {
                x: x as isize,
                y: y as isize,
            }) {
                '*'
            } else {
                ' '
            };
            print!("{c}");
        }
        println!();
    }
}

fn parse(input: &str) -> Map {
    let mut walkable = HashSet::new();
    let mut height = 0;
    let mut width = 0;

    let mut start = None;
    let mut end = None;

    for (y, line) in input.trim().lines().enumerate() {
        height = height.max(y);
        let y = y as isize;
        for (x, c) in line.trim().chars().enumerate() {
            width = width.max(x);
            let x = x as isize;

            if c == '#' {
                continue;
            }

            let point = Point { x, y };
            if c == 'S' {
                start = Some(point);
            }
            if c == 'E' {
                end = Some(point);
            }
            walkable.insert(point);
        }
    }

    Map {
        walkable,
        height,
        width,

        start: start.expect("map had no start"),
        end: end.expect("map had no end"),
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

struct Map {
    walkable: HashSet<Point>,
    height: usize,
    width: usize,

    start: Point,
    end: Point,
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (44, 0));
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
