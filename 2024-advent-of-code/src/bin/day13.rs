// #![feature(test)]

// extern crate test;

const EXAMPLE: &str = include_str!("example13.txt");
const INPUT: &str = include_str!("input13.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let arcades = parse(input);

    let cost = arcades
        .iter()
        .map(|arcade| min_cost(arcade, Point { x: 0, y: 0 }, 0, 0))
        .sum();

    (cost, 0)
}

const A_COST: usize = 3;
const B_COST: usize = 1;

fn min_cost(arcade: &Arcade, pos: Point, cost: usize, depth: usize) -> usize {
    if depth > 100 {
        return usize::MAX;
    }

    if pos == arcade.prize {
        return cost;
    }

    // min_cost(arcade, pos.clone(), cost + 1, depth + 1)
    let a_min = min_cost(arcade, pos + arcade.a, cost + A_COST, depth + 1);
    let b_min = min_cost(arcade, pos + arcade.b, cost + B_COST, depth + 1);

    a_min.min(b_min)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    assert_eq!(min_cost(&arcade, Point { x: 0, y: 0 }, 0, 0), 23);
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (480, 0));
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
