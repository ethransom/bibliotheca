#![feature(test)]

use std::collections::HashSet;

use anyhow::{anyhow, Context, Error, Result};

extern crate test;

const EXAMPLE: &str = include_str!("example09.txt");
const INPUT: &str = include_str!("input09.txt");

fn main() -> Result<()> {
    dbg!(solve(EXAMPLE)?);
    dbg!(solve(INPUT)?);

    Ok(())
}

fn solve(input: &str) -> Result<(usize, usize)> {
    let [mut head, mut tail]: [(i64, i64); 2] = [(0, 0); 2];

    let mut tail_positions = HashSet::<(i64, i64)>::new();

    for (direction, amount) in parse(input)? {
        for _ in 0..amount {
            let delta: (i64, i64) = match direction {
                Direction::Up => (0, 1),
                Direction::Down => (0, -1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };

            head.0 += delta.0;
            head.1 += delta.1;

            // chase the head with the tail
            if tail.0 < head.0 - 1 {
                tail.0 += 1;
                tail.1 = head.1;
            } else if tail.0 > head.0 + 1 {
                tail.0 -= 1;
                tail.1 = head.1;
            } else if tail.1 < head.1 - 1 {
                tail.1 += 1;
                tail.0 = head.0;
            } else if tail.1 > head.1 + 1 {
                tail.1 -= 1;
                tail.0 = head.0;
            }

            tail_positions.insert(tail);
        }
    }

    Ok((tail_positions.len(), 0))
}

fn parse(input: &str) -> Result<Vec<(Direction, i64)>> {
    input
        .lines()
        .enumerate()
        .map(|(num, line)| {
            let (dir, amt) = line
                .split_once(' ')
                .with_context(|| format!("line {num} needed two fields"))?;
            Ok((
                dir.try_into()
                    .with_context(|| format!("line {num} invalid"))?,
                amt.parse()
                    .with_context(|| format!("line {num} had invalid integer"))?,
            ))
        })
        .collect()
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&str> for Direction {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(anyhow!("invalid direction: {}", value)),
        }
    }
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE).unwrap(), (13, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT).unwrap(), (6_266, 0));
    });
}
