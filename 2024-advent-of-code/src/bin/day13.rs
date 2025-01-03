// #![feature(test)]

// extern crate test;

use std::collections::{HashMap, HashSet};

const EXAMPLE: &str = include_str!("example13.txt");
const INPUT: &str = include_str!("input13.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let arcades = parse(input);

    let cost = arcades.iter().filter_map(|a| dbg!(min_cost(a))).sum();

    (cost, 0)
}

const A_COST: usize = 3;
const B_COST: usize = 1;

fn min_cost(arcade: &Arcade) -> Option<usize> {
    // let frontier = BinaryHeap::<(usize, Point)>::new();
    type Node = ((usize, usize), Point);
    let mut frontier = HashSet::<Node>::new();
    let start = Point { x: 0, y: 0 };
    frontier.insert(((0, 0), start));

    let mut came_from = HashMap::<Point, Point>::new();

    let mut cost = HashMap::<Point, usize>::new();
    cost.insert(start, 0);

    let mut est_cost = HashMap::<Point, usize>::new();
    est_cost.insert(start, 0); // a little weird but I guess we have to hit the start first

    while !frontier.is_empty() {
        let &((a_presses, b_presses), current) =
            frontier.iter().min_by_key(|&(_, p)| est_cost[p]).unwrap();
        let current_cost = cost[&current];
        if current == arcade.prize {
            return Some(current_cost);
        }

        frontier.remove(&((a_presses, b_presses), current));

        if a_presses >= 100 || b_presses >= 100 {
            continue;
        }

        for (neighbor, incremental_cost, presses) in [
            (current + arcade.a, A_COST, (a_presses + 1, b_presses)),
            (current + arcade.b, B_COST, (a_presses, b_presses + 1)),
        ] {
            let tentative_cost = current_cost + incremental_cost;
            if tentative_cost < *cost.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, current);
                cost.insert(neighbor, tentative_cost);
                est_cost.insert(neighbor, tentative_cost + heuristic(neighbor, arcade.prize));
                frontier.insert((presses, neighbor));
            }
        }
    }

    None
}

fn heuristic(neighbor: Point, prize: Point) -> usize {
    ((neighbor.x + prize.x).pow(2) as usize + (neighbor.y + prize.y).pow(2) as usize).isqrt()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Arcade {
    a: Point,
    b: Point,
    prize: Point,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn parse(input: &str) -> Vec<Arcade> {
    input
        .split("\n\n")
        .map(|block| {
            let [a, b, prize] = block.trim().lines().collect::<Vec<_>>().try_into().unwrap();

            let a = parse_button(a, "Button A: ");
            let b = parse_button(b, "Button B: ");

            let prize = parse_prize(prize);

            Arcade { a, b, prize }
        })
        .collect()
}

fn parse_prize(prize: &str) -> Point {
    let Some(("", prize)) = prize.split_once("Prize: ") else {
        panic!();
    };

    let (x, y) = prize.split_once(", ").unwrap();

    let Some(("X", x)) = x.split_once("=") else {
        panic!();
    };

    let Some(("Y", y)) = y.split_once("=") else {
        panic!("no num in {x:?}");
    };

    let x = x.parse().unwrap();
    let y = y.parse().unwrap();

    Point { x, y }
}

fn parse_button(line: &str, header: &str) -> Point {
    let Some(("", point)) = line.split_once(header) else {
        panic!("couldn't get header from {line}");
    };

    let (x, y) = point.split_once(", ").unwrap();

    let Some(("X", x)) = x.split_once('+') else {
        panic!();
    };

    let Some(("Y", y)) = y.split_once('+') else {
        panic!();
    };

    let x = x.parse().unwrap();
    let y = y.parse().unwrap();

    dbg!(point);

    Point { x, y }
}

#[test]
fn test_one() {
    let arcade = Arcade {
        a: Point { x: 94, y: 34 },
        b: Point { x: 22, y: 67 },
        prize: Point { x: 8400, y: 5400 },
    };

    assert_eq!(min_cost(&arcade), Some(23));
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (480, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (37680, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
