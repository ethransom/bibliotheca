#![feature(test)]

extern crate test;

use std::fmt::{Debug, Formatter};

const EXAMPLE: &str = include_str!("example23.txt");
const INPUT: &str = include_str!("input23.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let rooms = parse(input);

    println!("{}", rooms);

    (0, 0)
}

struct Burrow {
    hallway: [Option<char>; 5],
    rooms: [[Option<char>; 2]; 4],
}

impl std::fmt::Display for Burrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#############
#.{}.{}.{}.{}.{}.#
###{}#{}#{}#{}###
  #{}#{}#{}#{}#
  #########",
            self.hallway[0].unwrap_or('.'),
            self.hallway[1].unwrap_or('.'),
            self.hallway[2].unwrap_or('.'),
            self.hallway[3].unwrap_or('.'),
            self.hallway[4].unwrap_or('.'),
            self.rooms[0][0].unwrap_or('.'),
            self.rooms[1][0].unwrap_or('.'),
            self.rooms[2][0].unwrap_or('.'),
            self.rooms[3][0].unwrap_or('.'),
            self.rooms[0][1].unwrap_or('.'),
            self.rooms[1][1].unwrap_or('.'),
            self.rooms[2][1].unwrap_or('.'),
            self.rooms[3][1].unwrap_or('.'),
        )
    }
}

fn parse(input: &str) -> Burrow {
    let lines = input.lines().collect::<Vec<&str>>();

    let rooms = [3, 5, 7, 9].map(|col| [2, 3].map(|row| lines[row].chars().nth(col)));

    Burrow {
        hallway: [None; 5],
        rooms,
    }
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (0, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
