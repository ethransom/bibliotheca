#![feature(test)]

use std::collections::HashSet;

use anyhow::{anyhow, Context, Error, Result};

extern crate test;

const EXAMPLE: &str = include_str!("example09.txt");
const LARGER_EXAMPLE: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
const INPUT: &str = include_str!("input09.txt");

fn main() -> Result<()> {
    dbg!(solve::<true>(EXAMPLE)?);
    dbg!(solve::<true>(LARGER_EXAMPLE)?);
    dbg!(solve::<true>(INPUT)?);

    Ok(())
}

fn solve<const PRINT: bool>(input: &str) -> Result<(usize, usize)> {
    Ok((simulate::<2, PRINT>(input)?, simulate::<10, PRINT>(input)?))
}

fn simulate<const LENGTH: usize, const PRINT: bool>(input: &str) -> Result<usize, Error> {
    let mut rope: [(i64, i64); LENGTH] = [(0, 0); LENGTH];

    let mut rope_states = vec![];

    let mut tail_positions = HashSet::new();

    for (direction, amount) in parse(input)? {
        for _ in 0..amount {
            let delta: (i64, i64) = match direction {
                Direction::Up => (0, 1),
                Direction::Down => (0, -1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };

            rope[0].0 += delta.0;
            rope[0].1 += delta.1;

            for i in 0..LENGTH - 1 {
                rope[i + 1] = chase(rope[i], rope[i + 1]);
            }

            rope_states.push(rope);

            tail_positions.insert(rope[LENGTH - 1]);
        }
    }

    if PRINT {
        print_states(rope_states);
    }

    Ok(tail_positions.len())
}

fn print_states<const LENGTH: usize>(states: Vec<[(i64, i64); LENGTH]>) {
    let x_range = states
        .iter()
        .map(|state| state.iter().map(|v| v.0).min().unwrap())
        .min()
        .unwrap()
        ..=states
            .iter()
            .map(|state| state.iter().map(|v| v.0).max().unwrap())
            .max()
            .unwrap();

    let y_range = states
        .iter()
        .map(|state| state.iter().map(|v| v.1).min().unwrap())
        .min()
        .unwrap()
        ..=states
            .iter()
            .map(|state| state.iter().map(|v| v.1).max().unwrap())
            .max()
            .unwrap();

    for state in states {
        for r in y_range.clone().rev() {
            for c in x_range.clone() {
                if let Some(pos) = state.iter().position(|v| v.0 == c && v.1 == r) {
                    let digit = if pos == 0 {
                        'H'
                    } else if pos == LENGTH - 1 {
                        'T'
                    } else {
                        char::from_digit(pos as u32, 10).unwrap()
                    };
                    print!("{}", digit);
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn chase(head: (i64, i64), mut tail: (i64, i64)) -> (i64, i64) {
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

    tail
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
    assert_eq!(solve::<false>(EXAMPLE).unwrap(), (13, 1));
}

#[test]
fn test_larger_example() {
    assert_eq!(solve::<false>(LARGER_EXAMPLE).unwrap().1, 36);
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve::<false>(INPUT).unwrap(), (6_266, 2_424));
    });
}
