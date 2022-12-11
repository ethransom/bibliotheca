#![feature(test)]

use regex::Regex;

extern crate test;

const EXAMPLE: &str = include_str!("example11.txt");
const INPUT: &str = include_str!("input11.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let notes = parse(input);

    dbg!(&notes);

    (0, 0)
}

#[derive(Debug)]
struct Note {
    starting_items: Vec<usize>,
    operation: Operation,
    test: usize,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug)]
struct Operation {
    left: OperationExpr,
    op: OperationOp,
    right: OperationExpr,
}

impl TryFrom<&str> for Operation {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(" ");

        let left = OperationExpr::try_from(parts.next().unwrap())?;
        let op = OperationOp::try_from(parts.next().unwrap())?;
        let right = OperationExpr::try_from(parts.next().unwrap())?;

        assert_eq!(parts.next(), None, "too many parts: {:?}", parts);

        Ok(Operation { left, op, right })
    }
}

#[derive(Debug)]
enum OperationExpr {
    Num(usize),
    Old,
}

impl TryFrom<&str> for OperationExpr {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "old" => Ok(OperationExpr::Old),
            _ => Ok(OperationExpr::Num(value.parse().unwrap())),
        }
    }
}

#[derive(Debug)]
enum OperationOp {
    Add,
    Mul,
}

impl TryFrom<&str> for OperationOp {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(OperationOp::Add),
            "*" => Ok(OperationOp::Mul),
            _ => Err(()),
        }
    }
}

const RE: &str = r#"Monkey (\d+):
  Starting items: ([\d, ]+)
  Operation: new = (.+)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)"#;

fn parse(input: &str) -> Vec<Note> {
    // TODO: I guess lazily compile this regex?
    let re = Regex::new(RE).unwrap();

    // TODO: I wonder if this is faster than .split("\n\n")?
    // for cap in re.captures_iter(text) {
    //     // handle captures
    // }

    input
        .split("\n\n")
        .enumerate()
        .map(|(i, monkey)| {
            let captures = re.captures(monkey).unwrap();

            assert_eq!(
                captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                i,
                "monkey number mismatch"
            );

            Note {
                starting_items: captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|s| s.parse().unwrap())
                    .collect(),
                operation: captures.get(3).unwrap().as_str().try_into().unwrap(),
                test: captures.get(4).unwrap().as_str().parse().unwrap(),
                if_true: captures.get(5).unwrap().as_str().parse().unwrap(),
                if_false: captures.get(6).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (10_605, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
