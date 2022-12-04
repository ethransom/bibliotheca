#![feature(test)]

use core::panic;

extern crate test;

const EXAMPLE: &str = include_str!("example02.txt");
const INPUT: &str = include_str!("input02.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

use Throw::*;

fn solve(input: &str) -> (usize, usize) {
    let part1 = parse(input)
        .map(|(theirs, ours)| {
            let ours = match ours {
                "X" => Rock,
                "Y" => Paper,
                "Z" => Scissors,
                _ => panic!(),
            };
            score(theirs, ours)
        })
        .sum();

    let part2 = parse(input)
        .map(|(theirs, ours)| {
            let ours = match ours {
                "X" => match theirs {
                    Rock => Scissors,
                    Paper => Rock,
                    Scissors => Paper,
                },
                "Y" => theirs,
                "Z" => match theirs {
                    Rock => Paper,
                    Paper => Scissors,
                    Scissors => Rock,
                },
                _ => panic!(),
            };
            score(theirs, ours)
        })
        .sum();

    (part1, part2)
}

fn score(theirs: Throw, ours: Throw) -> usize {
    let shape_score = match ours {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };
    let outcome = outcome(ours, theirs);
    let outcome_score = match outcome {
        Some(false) => 0,
        None => 3,
        Some(true) => 6,
    };
    shape_score + outcome_score
}

fn outcome(ours: Throw, theirs: Throw) -> Option<bool> {
    match (ours, theirs) {
        (Rock, Scissors) => Some(true),
        (Rock, Paper) => Some(false),
        (Paper, Rock) => Some(true),
        (Paper, Scissors) => Some(false),
        (Scissors, Paper) => Some(true),
        (Scissors, Rock) => Some(false),
        (Rock, Rock) => None,
        (Paper, Paper) => None,
        (Scissors, Scissors) => None,
    }
}

#[test]
fn test_score() {
    assert_eq!(score(Rock, Paper), 8);
    assert_eq!(score(Paper, Rock), 1);
    assert_eq!(score(Scissors, Scissors), 6);
}

fn parse(input: &str) -> impl Iterator<Item = (Throw, &str)> + '_ {
    input.lines().map(|line| {
        let (theirs, ours) = line.split_once(' ').unwrap();

        (
            match theirs {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissors,
                _ => panic!(),
            },
            ours,
        )
    })
}

#[test]
fn test_parse() {
    let mut iter = parse("A X\nC Z");
    assert_eq!(iter.next(), Some((Rock, "X")));
    assert_eq!(iter.next(), Some((Scissors, "Z")));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (15, 12));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (14_264, 12_382));
    });
}
