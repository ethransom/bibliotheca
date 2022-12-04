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
    let total_score = parse(input).map(|(theirs, ours)| score(theirs, ours)).sum();

    (total_score, 0)
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
    // dbg!(
    //     (ours, theirs),
    //     shape_score,
    //     outcome,
    //     outcome_score,
    //     shape_score + outcome_score
    // );
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

fn parse(input: &str) -> impl Iterator<Item = (Throw, Throw)> + '_ {
    input.lines().map(|line| {
        let (theirs, ours) = line.split_once(' ').unwrap();

        (
            match theirs {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissors,
                _ => panic!(),
            },
            match ours {
                "X" => Rock,
                "Y" => Paper,
                "Z" => Scissors,
                _ => panic!(),
            },
        )
    })
}

#[test]
fn test_parse() {
    let mut iter = parse("A X\nC Z");
    assert_eq!(iter.next(), Some((Rock, Rock)));
    assert_eq!(iter.next(), Some((Scissors, Scissors)));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (15, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (14_264, 0));
    });
}
