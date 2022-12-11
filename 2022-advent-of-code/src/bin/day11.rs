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
    let mut monkeys = parse(input);

    let mut inspections = vec![0; monkeys.len()];

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let (operation, test, if_true, if_false) = (
                monkeys[i].operation,
                monkeys[i].test,
                monkeys[i].if_true,
                monkeys[i].if_false,
            );

            let items = monkeys[i].items.drain(0..).collect::<Vec<_>>();

            inspections[i] += items.len();

            println!("Monkey {i}:");
            for worry in items {
                println!("  Monkey inspects an item with a worry level of {worry}.");

                let worry = operation.apply(worry);
                println!(
                    "    Worry level is {operation:?} to {new_worry}.",
                    operation = match operation.op {
                        OperationOp::Add => "increases",
                        OperationOp::Mul => "is multiplied",
                    },
                    new_worry = worry
                );

                let worry = worry / 3;
                println!(
                    "    Monkey gets bored with item. Worry level is divided by 3 to {worry}."
                );

                let monkey = if worry % test == 0 {
                    println!("    Current worry level is divisible by {test}.");
                    if_true
                } else {
                    println!("    Current worry level is not divisible by {test}.");
                    if_false
                };

                println!("    Item with worry level {worry} is thrown to monkey {monkey}.");
                monkeys[monkey].items.push(worry);
            }
        }

        for (i, monkey) in monkeys.iter().enumerate() {
            let items = monkey
                .items
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            println!("Monkey {i}: {items}");
        }
    }

    for (i, v) in inspections.iter().enumerate() {
        println!("Monkey {i} inspected items {v} times");
    }

    inspections.sort();

    let monkey_business = inspections.iter().rev().take(2).product();

    (monkey_business, 0)
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug, Copy, Clone)]
struct Operation {
    left: OperationExpr,
    op: OperationOp,
    right: OperationExpr,
}

impl TryFrom<&str> for Operation {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(' ');

        let left = OperationExpr::try_from(parts.next().unwrap())?;
        let op = OperationOp::try_from(parts.next().unwrap())?;
        let right = OperationExpr::try_from(parts.next().unwrap())?;

        assert_eq!(parts.next(), None, "too many parts: {:?}", parts);

        Ok(Operation { left, op, right })
    }
}

impl Operation {
    fn apply(&self, item: usize) -> usize {
        let (left, right) = (self.left.apply(item), self.right.apply(item));
        match self.op {
            OperationOp::Add => left + right,
            OperationOp::Mul => left * right,
        }
    }
}

#[derive(Debug, Copy, Clone)]
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

impl OperationExpr {
    fn apply(&self, item: usize) -> usize {
        match self {
            OperationExpr::Num(v) => *v,
            OperationExpr::Old => item,
        }
    }
}

#[derive(Debug, Copy, Clone)]
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

fn parse(input: &str) -> Vec<Monkey> {
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

            Monkey {
                items: captures
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
        assert_eq!(solve(INPUT), (99_852, 0));
    });
}
